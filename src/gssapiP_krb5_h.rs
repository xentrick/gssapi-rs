pub type krb5_gss_ctx_id_t = *mut crate::gssapiP_krb5_h::_krb5_gss_ctx_id_rec;
pub type sgn_alg = u32;
pub type seal_alg = u32;
pub const SGN_ALG_HMAC_SHA1_DES3_KD: crate::gssapiP_krb5_h::sgn_alg = 4;
pub const SGN_ALG_HMAC_MD5: crate::gssapiP_krb5_h::sgn_alg = 17;
pub const SEAL_ALG_DES3KD: crate::gssapiP_krb5_h::seal_alg = 2;
pub const SEAL_ALG_MICROSOFT_RC4: crate::gssapiP_krb5_h::seal_alg = 16;
pub const SEAL_ALG_NONE: crate::gssapiP_krb5_h::seal_alg = 65535;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_gss_ccache_name_req {
    pub name: *const i8,
    pub out_name: *mut *const i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_gss_set_allowable_enctypes_req {
    pub num_ktypes: crate::gssapi_h::OM_uint32,
    pub ktypes: *mut crate::krb5_h::krb5_enctype,
}
pub type krb5_gss_name_rec = crate::gssapiP_krb5_h::_krb5_gss_name_rec;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_gss_import_cred_req {
    pub id: crate::krb5_h::krb5_ccache,
    pub keytab_principal: crate::krb5_h::krb5_principal,
    pub keytab: crate::krb5_h::krb5_keytab,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub conv: *mut crate::krb5_h::krb5_data,
    pub verified: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_gss_name_rec {
    pub princ: crate::krb5_h::krb5_principal,
    pub service: *mut i8,
    pub host: *mut i8,
    pub lock: crate::k5_thread_h::k5_mutex_t,
    pub ad_context: crate::k5_int_h::krb5_authdata_context,
}
pub type krb5_gss_name_t = *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
#[repr(C)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct _krb5_gss_cred_id_rec {
    pub lock: crate::k5_thread_h::k5_mutex_t,
    pub usage: crate::gssapi_h::gss_cred_usage_t,
    pub name: crate::gssapiP_krb5_h::krb5_gss_name_t,
    pub impersonator: crate::krb5_h::krb5_principal,
    #[bitfield(name = "default_identity", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "iakerb_mech", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "destroy_ccache", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "suppress_ci_flags", ty = "libc::c_uint", bits = "3..=3")]
    pub default_identity_iakerb_mech_destroy_ccache_suppress_ci_flags: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub keytab: crate::krb5_h::krb5_keytab,
    pub rcache: crate::krb5_h::krb5_rcache,
    pub ccache: crate::krb5_h::krb5_ccache,
    pub client_keytab: crate::krb5_h::krb5_keytab,
    pub have_tgt: crate::krb5_h::krb5_boolean,
    pub expire: crate::krb5_h::krb5_timestamp,
    pub refresh_time: crate::krb5_h::krb5_timestamp,
    pub req_enctypes: *mut crate::krb5_h::krb5_enctype,
    pub password: *mut i8,
}
pub type krb5_gss_cred_id_rec = crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
pub type krb5_gss_cred_id_t = *mut crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_gss_ctx_ext_rec {
    pub iakerb: crate::gssapiP_krb5_h::C2RustUnnamed_0,
}
pub type krb5_gss_ctx_ext_rec = crate::gssapiP_krb5_h::_krb5_gss_ctx_ext_rec;
pub type krb5_gss_ctx_ext_t = *mut crate::gssapiP_krb5_h::_krb5_gss_ctx_ext_rec;
#[repr(C)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct _krb5_gss_ctx_id_rec {
    pub magic: crate::krb5_h::krb5_magic,
    #[bitfield(name = "initiate", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "established", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "have_acceptor_subkey", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "seed_init", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "terminated", ty = "libc::c_uint", bits = "4..=4")]
    pub initiate_established_have_acceptor_subkey_seed_init_terminated: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub gss_flags: crate::gssapi_h::OM_uint32,
    pub seed: [u8; 16],
    pub here: crate::gssapiP_krb5_h::krb5_gss_name_t,
    pub there: crate::gssapiP_krb5_h::krb5_gss_name_t,
    pub subkey: crate::krb5_h::krb5_key,
    pub signalg: i32,
    pub cksum_size: crate::stddef_h::size_t,
    pub sealalg: i32,
    pub enc: crate::krb5_h::krb5_key,
    pub seq: crate::krb5_h::krb5_key,
    pub krb_times: crate::krb5_h::krb5_ticket_times,
    pub krb_flags: crate::krb5_h::krb5_flags,
    pub seq_send: crate::stdlib::uint64_t,
    pub seq_recv: crate::stdlib::uint64_t,
    pub seqstate: crate::gssapiP_generic_h::g_seqnum_state,
    pub k5_context: crate::krb5_h::krb5_context,
    pub auth_context: crate::krb5_h::krb5_auth_context,
    pub mech_used: *mut crate::gssapi_h::gss_OID_desc,
    pub proto: i32,
    pub cksumtype: crate::krb5_h::krb5_cksumtype,
    pub acceptor_subkey: crate::krb5_h::krb5_key,
    pub acceptor_subkey_cksumtype: crate::krb5_h::krb5_cksumtype,
    pub cred_rcache: i32,
    pub authdata: *mut *mut crate::krb5_h::krb5_authdata,
}
pub type krb5_gss_ctx_id_rec = crate::gssapiP_krb5_h::_krb5_gss_ctx_id_rec;
