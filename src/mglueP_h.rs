use std::default::Default;
use std::os::raw::c_void;
use crate::gssapi_h::gss_OID;
use crate::gssapi_h::gss_OID_desc_struct;
// use crate::mglueP_h::gss_name_struct;

pub type gss_mech_spec_name = *mut crate::mglueP_h::gss_mech_spec_name_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_mech_spec_name_t {
    pub name_type: crate::gssapi_h::gss_OID,
    pub mech: crate::gssapi_h::gss_OID,
    pub next: *mut crate::mglueP_h::gss_mech_spec_name_t,
    pub prev: *mut crate::mglueP_h::gss_mech_spec_name_t,
}
pub type gss_mech_spec_name_desc = crate::mglueP_h::gss_mech_spec_name_t;
pub type gss_union_name_desc = crate::mglueP_h::gss_name_struct;
pub type gss_union_ctx_id_t = *mut crate::mglueP_h::gss_union_ctx_id_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_union_ctx_id_struct {
    pub loopback: *mut crate::mglueP_h::gss_union_ctx_id_struct,
    pub mech_type: crate::gssapi_h::gss_OID,
    pub internal_ctx_id: crate::gssapi_h::gss_ctx_id_t,
}
pub type gss_union_cred_t = *mut crate::mglueP_h::gss_cred_id_struct;
pub type gss_union_cred_desc = crate::mglueP_h::gss_cred_id_struct;
pub type gss_union_name_t = *mut crate::mglueP_h::gss_name_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_name_struct {
    pub loopback: *mut crate::mglueP_h::gss_name_struct,
    pub name_type: crate::gssapi_h::gss_OID,
    pub external_name: crate::gssapi_h::gss_buffer_t,
    pub mech_type: crate::gssapi_h::gss_OID,
    pub mech_name: crate::gssapi_h::gss_name_t,
}

// impl Default for gss_name_struct {
//     fn default() -> Self {
//         let oid: Vec<u8> = Vec::new();
//         let mut loopback: gss_name_struct = gss_name_struct::default();
//         gss_name_struct {
//             loopback:  loopback,
//             name_type: oid.as_mut_ptr() as *mut gss_OID_desc_struct,
//         }
//     }
// }


#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_cred_id_struct {
    pub loopback: *mut crate::mglueP_h::gss_cred_id_struct,
    pub count: i32,
    pub mechs_array: crate::gssapi_h::gss_OID,
    pub cred_array: *mut crate::gssapi_h::gss_cred_id_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_mech_config {
    pub kmodName: *mut i8,
    pub uLibName: *mut i8,
    pub mechNameStr: *mut i8,
    pub optionStr: *mut i8,
    pub dl_handle: *mut libc::c_void,
    pub mech_type: crate::gssapi_h::gss_OID,
    pub mech: crate::mglueP_h::gss_mechanism,
    pub priority: i32,
    pub freeMech: i32,
    pub is_interposer: i32,
    pub int_mech_type: crate::gssapi_h::gss_OID,
    pub int_mech: crate::mglueP_h::gss_mechanism,
    pub next: *mut crate::mglueP_h::gss_mech_config,
}
/*
 * This is the definition of the mechs_array struct, which is used to
 * define the mechs array table. This table is used to indirectly
 * access mechanism specific versions of the gssapi routines through
 * the routines in the glue module (gssd_mech_glue.c)
 *
 * This contants all of the functions defined in gssapi.h except for
 * gss_release_buffer() and gss_release_oid_set(), which I am
 * assuming, for now, to be equal across mechanisms.
 */
pub type gss_mechanism = *mut crate::mglueP_h::gss_config;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_config {
    pub mech_type: crate::gssapi_h::gss_OID_desc,
    pub context: *mut libc::c_void,
    pub gss_acquire_cred: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_OID_set,
            _: i32,
            _: *mut crate::gssapi_h::gss_cred_id_t,
            _: *mut crate::gssapi_h::gss_OID_set,
            _: *mut crate::gssapi_h::OM_uint32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_release_cred: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_cred_id_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_init_sec_context: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_cred_id_t,
            _: *mut crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_channel_bindings_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::OM_uint32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_accept_sec_context: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_cred_id_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: crate::gssapi_h::gss_channel_bindings_t,
            _: *mut crate::gssapi_h::gss_name_t,
            _: *mut crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_cred_id_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_process_context_token: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_delete_sec_context: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_context_time: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: *mut crate::gssapi_h::OM_uint32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_get_mic: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_qop_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_verify_mic: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut crate::gssapi_h::gss_qop_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_wrap: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: i32,
            _: crate::gssapi_h::gss_qop_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut i32,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_unwrap: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut i32,
            _: *mut crate::gssapi_h::gss_qop_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_display_status: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::OM_uint32,
            _: i32,
            _: crate::gssapi_h::gss_OID,
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_indicate_mechs: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_OID_set,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_compare_name: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::gss_name_t,
            _: *mut i32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_display_name: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut crate::gssapi_h::gss_OID,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_import_name: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_buffer_t,
            _: crate::gssapi_h::gss_OID,
            _: *mut crate::gssapi_h::gss_name_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_release_name: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_name_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_inquire_cred: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_cred_id_t,
            _: *mut crate::gssapi_h::gss_name_t,
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut i32,
            _: *mut crate::gssapi_h::gss_OID_set,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_add_cred: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_cred_id_t,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_cred_usage_t,
            _: crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_cred_id_t,
            _: *mut crate::gssapi_h::gss_OID_set,
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::OM_uint32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_export_sec_context: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_import_sec_context: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut crate::gssapi_h::gss_ctx_id_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_inquire_cred_by_mech: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_cred_id_t,
            _: crate::gssapi_h::gss_OID,
            _: *mut crate::gssapi_h::gss_name_t,
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_cred_usage_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_inquire_names_for_mech: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_OID,
            _: *mut crate::gssapi_h::gss_OID_set,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_inquire_context: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: *mut crate::gssapi_h::gss_name_t,
            _: *mut crate::gssapi_h::gss_name_t,
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_OID,
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut i32,
            _: *mut i32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_internal_release_oid: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_OID,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_wrap_size_limit: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: i32,
            _: crate::gssapi_h::gss_qop_t,
            _: crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::OM_uint32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_localname: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::gss_const_OID,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gssspi_authorize_localname: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::gss_const_buffer_t,
            _: crate::gssapi_h::gss_const_OID,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_export_name: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_duplicate_name: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: *mut crate::gssapi_h::gss_name_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_store_cred: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_cred_id_t,
            _: crate::gssapi_h::gss_cred_usage_t,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_OID_set,
            _: *mut crate::gssapi_h::gss_cred_usage_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_inquire_sec_context_by_oid: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_OID,
            _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_inquire_cred_by_oid: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_cred_id_t,
            _: crate::gssapi_h::gss_OID,
            _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_set_sec_context_option: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gssspi_set_cred_option: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_cred_id_t,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gssspi_mech_invoke: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_wrap_aead: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: i32,
            _: crate::gssapi_h::gss_qop_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut i32,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_unwrap_aead: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut i32,
            _: *mut crate::gssapi_h::gss_qop_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_wrap_iov: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: i32,
            _: crate::gssapi_h::gss_qop_t,
            _: *mut i32,
            _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
            _: i32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_unwrap_iov: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: *mut i32,
            _: *mut crate::gssapi_h::gss_qop_t,
            _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
            _: i32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_wrap_iov_length: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: i32,
            _: crate::gssapi_h::gss_qop_t,
            _: *mut i32,
            _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
            _: i32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_complete_auth_token: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_acquire_cred_impersonate_name: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_cred_id_t,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_OID_set,
            _: crate::gssapi_h::gss_cred_usage_t,
            _: *mut crate::gssapi_h::gss_cred_id_t,
            _: *mut crate::gssapi_h::gss_OID_set,
            _: *mut crate::gssapi_h::OM_uint32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_add_cred_impersonate_name: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_cred_id_t,
            _: crate::gssapi_h::gss_cred_id_t,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_cred_usage_t,
            _: crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_cred_id_t,
            _: *mut crate::gssapi_h::gss_OID_set,
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::OM_uint32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_display_name_ext: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_inquire_name: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: *mut i32,
            _: *mut crate::gssapi_h::gss_OID,
            _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_get_name_attribute: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut i32,
            _: *mut i32,
            _: crate::gssapi_h::gss_buffer_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut i32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_set_name_attribute: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: i32,
            _: crate::gssapi_h::gss_buffer_t,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_delete_name_attribute: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_export_name_composite: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_map_name_to_any: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: i32,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut crate::gssapi_ext_h::gss_any_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_release_any_name_mapping: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut crate::gssapi_ext_h::gss_any_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_pseudo_random: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: i32,
            _: crate::gssapi_h::gss_buffer_t,
            _: crate::stdlib::ssize_t,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_set_neg_mechs: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_cred_id_t,
            _: crate::gssapi_h::gss_OID_set,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_inquire_saslname_for_mech: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_buffer_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_inquire_mech_for_saslname: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut crate::gssapi_h::gss_OID,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_inquire_attrs_for_mech: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_const_OID,
            _: *mut crate::gssapi_h::gss_OID_set,
            _: *mut crate::gssapi_h::gss_OID_set,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_acquire_cred_from: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_OID_set,
            _: crate::gssapi_h::gss_cred_usage_t,
            _: crate::gssapi_ext_h::gss_const_key_value_set_t,
            _: *mut crate::gssapi_h::gss_cred_id_t,
            _: *mut crate::gssapi_h::gss_OID_set,
            _: *mut crate::gssapi_h::OM_uint32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_store_cred_into: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_cred_id_t,
            _: crate::gssapi_h::gss_cred_usage_t,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::OM_uint32,
            _: crate::gssapi_ext_h::gss_const_key_value_set_t,
            _: *mut crate::gssapi_h::gss_OID_set,
            _: *mut crate::gssapi_h::gss_cred_usage_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gssspi_acquire_cred_with_password: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::gss_buffer_t,
            _: crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_OID_set,
            _: i32,
            _: *mut crate::gssapi_h::gss_cred_id_t,
            _: *mut crate::gssapi_h::gss_OID_set,
            _: *mut crate::gssapi_h::OM_uint32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_export_cred: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_cred_id_t,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_import_cred: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut crate::gssapi_h::gss_cred_id_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gssspi_import_sec_context_by_mech: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut crate::gssapi_h::gss_ctx_id_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gssspi_import_name_by_mech: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_buffer_t,
            _: crate::gssapi_h::gss_OID,
            _: *mut crate::gssapi_h::gss_name_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gssspi_import_cred_by_mech: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_buffer_t,
            _: *mut crate::gssapi_h::gss_cred_id_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_get_mic_iov: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_qop_t,
            _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
            _: i32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_verify_mic_iov: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: *mut crate::gssapi_h::gss_qop_t,
            _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
            _: i32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gss_get_mic_iov_length: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_qop_t,
            _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
            _: i32,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gssspi_query_meta_data: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_const_OID,
            _: crate::gssapi_h::gss_cred_id_t,
            _: *mut crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gssspi_exchange_meta_data: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_const_OID,
            _: crate::gssapi_h::gss_cred_id_t,
            _: *mut crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_name_t,
            _: crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_const_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
    pub gssspi_query_mechanism_info: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_const_OID,
            _: *mut u8,
        ) -> crate::gssapi_h::OM_uint32,
    >,
}
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

/* token_buffer */

/* minor_status */

/* context_handle */

/* output_token */

/* minor_status */

/* context_handle */

/* time_rec */

/* minor_status */

/* context_handle */

/* qop_req */

/* message_buffer */

/* message_token */

/* minor_status */

/* context_handle */

/* message_buffer */

/* token_buffer */

/* qop_state */

/* minor_status */

/* context_handle */

/* conf_req_flag */

/* qop_req */

/* input_message_buffer */

/* conf_state */

/* output_message_buffer */

/* minor_status */

/* context_handle */

/* input_message_buffer */

/* output_message_buffer */

/* conf_state */

/* qop_state */

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

/* input_cred_handle */

/* desired_name */

/* desired_mech */

/* cred_usage */

/* initiator_time_req */

/* acceptor_time_req */

/* output_cred_handle */

/* actual_mechs */

/* initiator_time_rec */

/* acceptor_time_rec */

/* minor_status */

/* context_handle */

/* interprocess_token */

/* minor_status */

/* interprocess_token */

/* context_handle */

/* minor_status */

/* cred_handle */

/* mech_type */

/* name */

/* initiator_lifetime */

/* acceptor_lifetime */

/* cred_usage */

/* minor_status */

/* mechanism */

/* name_types */

/* minor_status */

/* context_handle */

/* src_name */

/* targ_name */

/* lifetime_rec */

/* mech_type */

/* ctx_flags */

/* locally_initiated */

/* open */

/* minor_status */

/* OID */

/* minor_status */

/* context_handle */

/* conf_req_flag */

/* qop_req */

/* req_output_size */

/* max_input_size */

/* minor */

/* name */

/* mech_type */

/* localname */

/* minor_status */

/* pname */

/* local user */

/* local nametype */

/* */

/* minor_status */

/* input_name */

/* exported_name */

/* */

/* minor_status */

/* input_name */

/* output_name */

/* */

/* minor_status */

/* input_cred */

/* cred_usage */

/* desired_mech */

/* overwrite_cred */

/* default_cred */

/* elements_stored */

/* cred_usage_stored */

/* */

/* GGF extensions */

/* minor_status */

/* context_handle */

/* OID */

/* data_set */

/* minor_status */

/* cred_handle */

/* OID */

/* data_set */

/* minor_status */

/* context_handle */

/* OID */

/* value */

/* minor_status */

/* cred_handle */

/* OID */

/* value */

/* minor_status */

/* mech OID */

/* OID */

/* value */

/* AEAD extensions */

/* minor_status */

/* context_handle */

/* conf_req_flag */

/* qop_req */

/* input_assoc_buffer */

/* input_payload_buffer */

/* conf_state */

/* output_message_buffer */

/* */

/* minor_status */

/* context_handle */

/* input_message_buffer */

/* input_assoc_buffer */

/* output_payload_buffer */

/* conf_state */

/* qop_state */

/* */

/* SSPI extensions */

/* minor_status */

/* context_handle */

/* conf_req_flag */

/* qop_req */

/* conf_state */

/* iov */

/* iov_count */

/* */

/* minor_status */

/* context_handle */

/* conf_state */

/* qop_state */

/* iov */

/* iov_count */

/* */

/* minor_status */

/* context_handle */

/* conf_req_flag*/

/* qop_req */

/* conf_state */

/* iov */

/* iov_count */

/* */

/* minor_status */

/* context_handle */

/* input_message_buffer */

/* New for 1.8 */

/* minor_status */

/* impersonator_cred_handle */

/* desired_name */

/* time_req */

/* desired_mechs */

/* cred_usage */

/* output_cred_handle */

/* actual_mechs */

/* time_rec */

/* */

/* minor_status */

/* input_cred_handle */

/* impersonator_cred_handle */

/* desired_name */

/* desired_mech */

/* cred_usage */

/* initiator_time_req */

/* acceptor_time_req */

/* output_cred_handle */

/* actual_mechs */

/* initiator_time_rec */

/* acceptor_time_rec */

/* */

/* minor_status */

/* name */

/* display_as_name_type */

/* display_name */

/* */

/* minor_status */

/* name */

/* name_is_MN */

/* MN_mech */

/* attrs */

/* */

/* minor_status */

/* name */

/* attr */

/* authenticated */

/* complete */

/* value */

/* display_value */

/* more */

/* */

/* minor_status */

/* name */

/* complete */

/* attr */

/* value */

/* */

/* minor_status */

/* name */

/* attr */

/* */

/* minor_status */

/* name */

/* exp_composite_name */

/* */

/* minor_status */

/* name */

/* authenticated */

/* type_id */

/* output */

/* */

/* minor_status */

/* name */

/* type_id */

/* input */

/* */

/* minor_status */

/* context */

/* prf_key */

/* prf_in */

/* desired_output_len */

/* prf_out */

/* */

/* minor_status */

/* cred_handle */

/* mech_set */

/* */

/* minor_status */

/* desired_mech */

/* sasl_mech_name */

/* mech_name */

/* mech_description */

/* */

/* minor_status */

/* sasl_mech_name */

/* mech_type */

/* */

/* minor_status */

/* mech */

/* mech_attrs */

/* known_mech_attrs */

/* */

/* Credential store extensions */

/* minor_status */

/* desired_name */

/* time_req */

/* desired_mechs */

/* cred_usage */

/* cred_store */

/* output_cred_handle */

/* actual_mechs */

/* time_rec */

/* */

/* minor_status */

/* input_cred_handle */

/* input_usage */

/* desired_mech */

/* overwrite_cred */

/* default_cred */

/* cred_store */

/* elements_stored */

/* cred_usage_stored */

/* */

/* minor_status */

/* desired_name */

/* password */

/* time_req */

/* desired_mechs */

/* cred_usage */

/* output_cred_handle */

/* actual_mechs */

/* time_rec */

/* */

/* minor_status */

/* cred_handle */

/* token */

/* */

/* minor_status */

/* token */

/* cred_handle */

/* */

/* minor_status */

/* desired_mech */

/* interprocess_token */

/* context_handle */

/* */

/* minor_status */

/* mech_type */

/* input_name_buffer */

/* input_name_type */

/* output_name */

/* */

/* minor_status */

/* mech_type */

/* token */

/* cred_handle */

/* */

/* get_mic_iov extensions, added in 1.12 */

/* minor_status */

/* context_handle */

/* qop_req */

/* iov */

/* iov_count */

/* minor_status */

/* context_handle */

/* qop_state */

/* iov */

/* iov_count */

/* minor_status */

/* context_handle */

/* qop_req */

/* iov */

/* iov_count */

/* NegoEx extensions added in 1.18 */

/* minor_status */

/* mech_oid */

/* cred_handle */

/* context_handle */

/* targ_name */

/* req_flags */

/* meta_data */

/* */

/* minor_status */

/* mech_oid */

/* cred_handle */

/* context_handle */

/* targ_name */

/* req_flags */

/* meta_data */

/* */

/* minor_status */

/* mech_oid */

/* auth_scheme */

/* */

/*
 * In the user space we use a wrapper structure to encompass the
 * mechanism entry points.  The wrapper contain the mechanism
 * entry points and other data which is only relevant to the gss-api
 * layer.  In the kernel we use only the gss_config strucutre because
 * the kernal does not cantain any of the extra gss-api specific data.
 */
pub type gss_mech_info = *mut crate::mglueP_h::gss_mech_config;
