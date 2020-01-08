#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#[path = "src/authdata_plugin_h.rs"]
pub mod authdata_plugin_h;
#[path = "src/com_err_h.rs"]
pub mod com_err_h;
#[path = "src/errmap_h.rs"]
pub mod errmap_h;
#[path = "src/error_map_h.rs"]
pub mod error_map_h;
#[path = "src/gssapiP_generic_h.rs"]
pub mod gssapiP_generic_h;
#[path = "src/gssapiP_krb5_h.rs"]
pub mod gssapiP_krb5_h;
#[path = "src/gssapiP_negoex_h.rs"]
pub mod gssapiP_negoex_h;
#[path = "src/gssapiP_spnego_h.rs"]
pub mod gssapiP_spnego_h;
#[path = "src/gssapi_ext_h.rs"]
pub mod gssapi_ext_h;
#[path = "src/gssapi_h.rs"]
pub mod gssapi_h;
#[path = "src/gssapi_krb5_h.rs"]
pub mod gssapi_krb5_h;
#[path = "src/internal.rs"]
pub mod internal;
#[path = "src/k5_buf_h.rs"]
pub mod k5_buf_h;
#[path = "src/k5_err_h.rs"]
pub mod k5_err_h;
#[path = "src/k5_input_h.rs"]
pub mod k5_input_h;
#[path = "src/k5_int_h.rs"]
pub mod k5_int_h;
#[path = "src/k5_int_pkinit_h.rs"]
pub mod k5_int_pkinit_h;
#[path = "src/k5_json_h.rs"]
pub mod k5_json_h;
#[path = "src/k5_platform_h.rs"]
pub mod k5_platform_h;
#[path = "src/k5_plugin_h.rs"]
pub mod k5_plugin_h;
#[path = "src/k5_thread_h.rs"]
pub mod k5_thread_h;
#[path = "src/k5_trace_h.rs"]
pub mod k5_trace_h;
#[path = "src/krb5_h.rs"]
pub mod krb5_h;
#[path = "src/mglueP_h.rs"]
pub mod mglueP_h;
#[path = "src/profile_h.rs"]
pub mod profile_h;
#[path = "src/stdarg_h.rs"]
pub mod stdarg_h;
#[path = "src/stddef_h.rs"]
pub mod stddef_h;
#[path = "src/stdlib.rs"]
pub mod stdlib;
#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod src {
    pub mod generic {
        pub mod disp_com_err_status;
        pub mod disp_major_status;
        pub mod gssapi_err_generic;
        pub mod gssapi_generic;
        pub mod oid_ops;
        pub mod rel_buffer;
        pub mod rel_oid_set;
        pub mod util_buffer;
        pub mod util_buffer_set;
        pub mod util_errmap;
        pub mod util_seqstate;
        pub mod util_set;
        pub mod util_token;
    } // mod generic
    pub mod krb5 {
        pub mod accept_sec_context;
        pub mod acquire_cred;
        pub mod canon_name;
        pub mod compare_name;
        pub mod context_time;
        pub mod copy_ccache;
        pub mod cred_store;
        pub mod delete_sec_context;
        pub mod disp_name;
        pub mod disp_status;
        pub mod duplicate_name;
        pub mod export_cred;
        pub mod export_name;
        pub mod export_sec_context;
        pub mod get_tkt_flags;
        pub mod gssapi_err_krb5;
        pub mod gssapi_krb5;
        pub mod iakerb;
        pub mod import_cred;
        pub mod import_name;
        pub mod import_sec_context;
        pub mod indicate_mechs;
        pub mod init_sec_context;
        pub mod inq_context;
        pub mod inq_cred;
        pub mod inq_names;
        pub mod k5seal;
        pub mod k5sealiov;
        pub mod k5sealv3;
        pub mod k5sealv3iov;
        pub mod k5unseal;
        pub mod k5unsealiov;
        pub mod krb5_gss_glue;
        pub mod lucid_context;
        pub mod naming_exts;
        pub mod prf;
        pub mod process_context_token;
        pub mod rel_cred;
        pub mod rel_name;
        pub mod rel_oid;
        pub mod s4u_gss_glue;
        pub mod ser_sctx;
        pub mod set_allowable_enctypes;
        pub mod set_ccache;
        pub mod store_cred;
        pub mod util_cksum;
        pub mod util_crypt;
        pub mod util_seed;
        pub mod util_seqnum;
        pub mod val_cred;
        pub mod wrap_size_limit;
    } // mod krb5
    pub mod mechglue {
        pub mod g_accept_sec_context;
        pub mod g_acquire_cred;
        pub mod g_acquire_cred_imp_name;
        pub mod g_acquire_cred_with_pw;
        pub mod g_authorize_localname;
        pub mod g_buffer_set;
        pub mod g_canon_name;
        pub mod g_compare_name;
        pub mod g_complete_auth_token;
        pub mod g_context_time;
        pub mod g_decapsulate_token;
        pub mod g_del_name_attr;
        pub mod g_delete_sec_context;
        pub mod g_dsp_name;
        pub mod g_dsp_name_ext;
        pub mod g_dsp_status;
        pub mod g_dup_name;
        pub mod g_encapsulate_token;
        pub mod g_exp_sec_context;
        pub mod g_export_cred;
        pub mod g_export_name;
        pub mod g_export_name_comp;
        pub mod g_get_name_attr;
        pub mod g_glue;
        pub mod g_imp_cred;
        pub mod g_imp_name;
        pub mod g_imp_sec_context;
        pub mod g_init_sec_context;
        pub mod g_initialize;
        pub mod g_inq_context;
        pub mod g_inq_context_oid;
        pub mod g_inq_cred;
        pub mod g_inq_cred_oid;
        pub mod g_inq_name;
        pub mod g_inq_names;
        pub mod g_map_name_to_any;
        pub mod g_mech_invoke;
        pub mod g_mechattr;
        pub mod g_mechname;
        pub mod g_negoex;
        pub mod g_oid_ops;
        pub mod g_prf;
        pub mod g_process_context;
        pub mod g_rel_buffer;
        pub mod g_rel_cred;
        pub mod g_rel_name;
        pub mod g_rel_name_mapping;
        pub mod g_rel_oid_set;
        pub mod g_saslname;
        pub mod g_seal;
        pub mod g_set_context_option;
        pub mod g_set_cred_option;
        pub mod g_set_name_attr;
        pub mod g_set_neg_mechs;
        pub mod g_sign;
        pub mod g_store_cred;
        pub mod g_unseal;
        pub mod g_unwrap_aead;
        pub mod g_unwrap_iov;
        pub mod g_verify;
        pub mod g_wrap_aead;
        pub mod g_wrap_iov;
        pub mod gssd_pname_to_uid;
    } // mod mechglue
    pub mod spnego {
        pub mod negoex_ctx;
        pub mod negoex_util;
        pub mod spnego_mech;
    } // mod spnego
} // mod src
