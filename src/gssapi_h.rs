extern "C" {
    pub type gss_ctx_id_struct;
}
// =============== BEGIN gssapi_h ================
pub type gss_int32 = crate::stdlib::int32_t;
/*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
pub type gss_uint32 = crate::stdlib::uint32_t;
/* OM_STRING */

/*
 * We can't use X/Open definitions, so roll our own.
 */
pub type OM_uint32 = crate::gssapi_h::gss_uint32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_buffer_desc_struct {
    pub length: crate::stddef_h::size_t,
    pub value: *mut libc::c_void,
}
pub type gss_buffer_t = *mut crate::gssapi_h::gss_buffer_desc_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_OID_desc_struct {
    pub length: crate::gssapi_h::OM_uint32,
    pub elements: *mut libc::c_void,
}
pub type gss_OID_desc = crate::gssapi_h::gss_OID_desc_struct;
pub type gss_OID = *mut crate::gssapi_h::gss_OID_desc_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_OID_set_desc_struct {
    pub count: crate::stddef_h::size_t,
    pub elements: crate::gssapi_h::gss_OID,
}
pub type gss_OID_set_desc = crate::gssapi_h::gss_OID_set_desc_struct;
pub type gss_OID_set = *mut crate::gssapi_h::gss_OID_set_desc_struct;
pub type gss_buffer_desc = crate::gssapi_h::gss_buffer_desc_struct;
pub type gss_const_OID = *const crate::gssapi_h::gss_OID_desc;
pub type gss_const_OID_set = *const crate::gssapi_h::gss_OID_set_desc;
/*
 * For now, define a QOP-type as an OM_uint32 (pending resolution of ongoing
 * discussions).
 */
pub type gss_qop_t = crate::gssapi_h::OM_uint32;
/* mech_set */

/* XXXX these are not part of the GSSAPI C bindings!  (but should be) */

/* XXXX This is a necessary evil until the spec is fixed */

/*
 * RFC 5587
 */
pub type gss_const_buffer_t = *const crate::gssapi_h::gss_buffer_desc;
pub type gss_name_t = *mut crate::mglueP_h::gss_name_struct;
pub type gss_cred_id_t = *mut crate::mglueP_h::gss_cred_id_struct;
pub type gss_ctx_id_t = *mut crate::gssapi_h::gss_ctx_id_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_channel_bindings_struct {
    pub initiator_addrtype: crate::gssapi_h::OM_uint32,
    pub initiator_address: crate::gssapi_h::gss_buffer_desc,
    pub acceptor_addrtype: crate::gssapi_h::OM_uint32,
    pub acceptor_address: crate::gssapi_h::gss_buffer_desc,
    pub application_data: crate::gssapi_h::gss_buffer_desc,
}
pub type gss_channel_bindings_t = *mut crate::gssapi_h::gss_channel_bindings_struct;
pub type gss_cred_usage_t = i32;
