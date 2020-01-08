#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_krb5_lucid_key {
    pub type_0: crate::gssapi_h::OM_uint32,
    pub length: crate::gssapi_h::OM_uint32,
    pub data: *mut libc::c_void,
}
pub type gss_krb5_lucid_key_t = crate::gssapi_krb5_h::gss_krb5_lucid_key;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_krb5_rfc1964_keydata {
    pub sign_alg: crate::gssapi_h::OM_uint32,
    pub seal_alg: crate::gssapi_h::OM_uint32,
    pub ctx_key: crate::gssapi_krb5_h::gss_krb5_lucid_key_t,
}
pub type gss_krb5_rfc1964_keydata_t = crate::gssapi_krb5_h::gss_krb5_rfc1964_keydata;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_krb5_cfx_keydata {
    pub have_acceptor_subkey: crate::gssapi_h::OM_uint32,
    pub ctx_key: crate::gssapi_krb5_h::gss_krb5_lucid_key_t,
    pub acceptor_subkey: crate::gssapi_krb5_h::gss_krb5_lucid_key_t,
}
pub type gss_krb5_cfx_keydata_t = crate::gssapi_krb5_h::gss_krb5_cfx_keydata;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_krb5_lucid_context_v1 {
    pub version: crate::gssapi_h::OM_uint32,
    pub initiate: crate::gssapi_h::OM_uint32,
    pub endtime: crate::gssapi_h::OM_uint32,
    pub send_seq: crate::stdlib::uint64_t,
    pub recv_seq: crate::stdlib::uint64_t,
    pub protocol: crate::gssapi_h::OM_uint32,
    pub rfc1964_kd: crate::gssapi_krb5_h::gss_krb5_rfc1964_keydata_t,
    pub cfx_kd: crate::gssapi_krb5_h::gss_krb5_cfx_keydata_t,
}
pub type gss_krb5_lucid_context_v1_t = crate::gssapi_krb5_h::gss_krb5_lucid_context_v1;
/* actual key data */

/* Context key
(Kerberos session key or subkey) */

/* acceptor-asserted subkey or
0's if no acceptor subkey */

/*
 * Mask for determining the version of a lucid context structure.  Callers
 * should not require this.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_krb5_lucid_context_version {
    pub version: crate::gssapi_h::OM_uint32,
}
pub type gss_krb5_lucid_context_version_t = crate::gssapi_krb5_h::gss_krb5_lucid_context_version;
