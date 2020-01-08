extern "C" {
    pub type gss_any;
}
// =============== BEGIN gssapi_ext_h ================

/* acceptor_time_rec */

/*
 * GGF extensions
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_buffer_set_desc_struct {
    pub count: crate::stddef_h::size_t,
    pub elements: *mut crate::gssapi_h::gss_buffer_desc,
}
pub type gss_buffer_set_desc = crate::gssapi_ext_h::gss_buffer_set_desc_struct;
pub type gss_buffer_set_t = *mut crate::gssapi_ext_h::gss_buffer_set_desc_struct;
pub type gss_iov_buffer_t = *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
/* Credential store extensions */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_key_value_element_struct {
    pub key: *const i8,
    pub value: *const i8,
}
pub type gss_key_value_element_desc = crate::gssapi_ext_h::gss_key_value_element_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_key_value_set_struct {
    pub count: crate::gssapi_h::OM_uint32,
    pub elements: *mut crate::gssapi_ext_h::gss_key_value_element_desc,
}
pub type gss_key_value_set_desc = crate::gssapi_ext_h::gss_key_value_set_struct;
pub type gss_const_key_value_set_t = *const crate::gssapi_ext_h::gss_key_value_set_desc;
pub type gss_iov_buffer_desc = crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
pub type gss_any_t = *mut crate::gssapi_ext_h::gss_any;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_iov_buffer_desc_struct {
    pub type_0: crate::gssapi_h::OM_uint32,
    pub buffer: crate::gssapi_h::gss_buffer_desc,
}
