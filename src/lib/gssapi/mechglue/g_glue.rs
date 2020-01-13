use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:25"]
pub mod types_h {
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:25"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:25"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/sys/types.h:25"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/mechglue/mglueP.h:25"]
pub mod mglueP_h {
    /* lib/gssapi/mechglue/mglueP.h */
    /*
 * Copyright (c) 1995, by Sun Microsystems, Inc.
 * All rights reserved.
 */
    /* This header contains the private mechglue definitions. */
    /*
 * Array of context IDs typed by mechanism OID
 */
    /*
 * Generic GSSAPI names.  A name can either be a generic name, or a
 * mechanism specific name....
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:16"]
    pub struct gss_name_struct {
        pub loopback: *mut gss_name_struct,
        pub name_type: gss_OID,
        pub external_name: gss_buffer_t,
        pub mech_type: gss_OID,
        pub mech_name: gss_name_t,
    }
    /*
 * Structure for holding list of mechanism-specific name types
 */
    /*
 * Set of Credentials typed on mechanism OID
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:16"]
    pub struct gss_cred_id_struct {
        pub loopback: *mut gss_cred_id_struct,
        pub count: libc::c_int,
        pub mechs_array: gss_OID,
        pub cred_array: *mut gss_cred_id_t,
    }
    #[c2rust::src_loc = "26:1"]
    pub type gss_union_ctx_id_t = *mut gss_union_ctx_id_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:16"]
    pub struct gss_union_ctx_id_struct {
        pub loopback: *mut gss_union_ctx_id_struct,
        pub mech_type: gss_OID,
        pub internal_ctx_id: gss_ctx_id_t,
    }
    #[c2rust::src_loc = "95:1"]
    pub type gss_mechanism = *mut gss_config;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "95:16"]
    pub struct gss_config {
        pub mech_type: gss_OID_desc,
        pub context: *mut libc::c_void,
        pub gss_acquire_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_name_t,
                                                          _: OM_uint32,
                                                          _: gss_OID_set,
                                                          _: libc::c_int,
                                                          _:
                                                              *mut gss_cred_id_t,
                                                          _: *mut gss_OID_set,
                                                          _: *mut OM_uint32)
                                         -> OM_uint32>,
        pub gss_release_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _:
                                                              *mut gss_cred_id_t)
                                         -> OM_uint32>,
        pub gss_init_sec_context: Option<unsafe extern "C" fn(_:
                                                                  *mut OM_uint32,
                                                              _:
                                                                  gss_cred_id_t,
                                                              _:
                                                                  *mut gss_ctx_id_t,
                                                              _: gss_name_t,
                                                              _: gss_OID,
                                                              _: OM_uint32,
                                                              _: OM_uint32,
                                                              _:
                                                                  gss_channel_bindings_t,
                                                              _: gss_buffer_t,
                                                              _: *mut gss_OID,
                                                              _: gss_buffer_t,
                                                              _:
                                                                  *mut OM_uint32,
                                                              _:
                                                                  *mut OM_uint32)
                                             -> OM_uint32>,
        pub gss_accept_sec_context: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut gss_ctx_id_t,
                                                                _:
                                                                    gss_cred_id_t,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    gss_channel_bindings_t,
                                                                _:
                                                                    *mut gss_name_t,
                                                                _:
                                                                    *mut gss_OID,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut gss_cred_id_t)
                                               -> OM_uint32>,
        pub gss_process_context_token: Option<unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       gss_ctx_id_t,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32>,
        pub gss_delete_sec_context: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut gss_ctx_id_t,
                                                                _:
                                                                    gss_buffer_t)
                                               -> OM_uint32>,
        pub gss_context_time: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_ctx_id_t,
                                                          _: *mut OM_uint32)
                                         -> OM_uint32>,
        pub gss_get_mic: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                     _: gss_ctx_id_t,
                                                     _: gss_qop_t,
                                                     _: gss_buffer_t,
                                                     _: gss_buffer_t)
                                    -> OM_uint32>,
        pub gss_verify_mic: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                        _: gss_ctx_id_t,
                                                        _: gss_buffer_t,
                                                        _: gss_buffer_t,
                                                        _: *mut gss_qop_t)
                                       -> OM_uint32>,
        pub gss_wrap: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                  _: gss_ctx_id_t,
                                                  _: libc::c_int,
                                                  _: gss_qop_t,
                                                  _: gss_buffer_t,
                                                  _: *mut libc::c_int,
                                                  _: gss_buffer_t)
                                 -> OM_uint32>,
        pub gss_unwrap: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                    _: gss_ctx_id_t,
                                                    _: gss_buffer_t,
                                                    _: gss_buffer_t,
                                                    _: *mut libc::c_int,
                                                    _: *mut gss_qop_t)
                                   -> OM_uint32>,
        pub gss_display_status: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                            _: OM_uint32,
                                                            _: libc::c_int,
                                                            _: gss_OID,
                                                            _: *mut OM_uint32,
                                                            _: gss_buffer_t)
                                           -> OM_uint32>,
        pub gss_indicate_mechs: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                            _:
                                                                *mut gss_OID_set)
                                           -> OM_uint32>,
        pub gss_compare_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_name_t,
                                                          _: gss_name_t,
                                                          _: *mut libc::c_int)
                                         -> OM_uint32>,
        pub gss_display_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_name_t,
                                                          _: gss_buffer_t,
                                                          _: *mut gss_OID)
                                         -> OM_uint32>,
        pub gss_import_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_buffer_t,
                                                         _: gss_OID,
                                                         _: *mut gss_name_t)
                                        -> OM_uint32>,
        pub gss_release_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: *mut gss_name_t)
                                         -> OM_uint32>,
        pub gss_inquire_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_cred_id_t,
                                                          _: *mut gss_name_t,
                                                          _: *mut OM_uint32,
                                                          _: *mut libc::c_int,
                                                          _: *mut gss_OID_set)
                                         -> OM_uint32>,
        pub gss_add_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                      _: gss_cred_id_t,
                                                      _: gss_name_t,
                                                      _: gss_OID,
                                                      _: gss_cred_usage_t,
                                                      _: OM_uint32,
                                                      _: OM_uint32,
                                                      _: *mut gss_cred_id_t,
                                                      _: *mut gss_OID_set,
                                                      _: *mut OM_uint32,
                                                      _: *mut OM_uint32)
                                     -> OM_uint32>,
        pub gss_export_sec_context: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut gss_ctx_id_t,
                                                                _:
                                                                    gss_buffer_t)
                                               -> OM_uint32>,
        pub gss_import_sec_context: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    *mut gss_ctx_id_t)
                                               -> OM_uint32>,
        pub gss_inquire_cred_by_mech: Option<unsafe extern "C" fn(_:
                                                                      *mut OM_uint32,
                                                                  _:
                                                                      gss_cred_id_t,
                                                                  _: gss_OID,
                                                                  _:
                                                                      *mut gss_name_t,
                                                                  _:
                                                                      *mut OM_uint32,
                                                                  _:
                                                                      *mut OM_uint32,
                                                                  _:
                                                                      *mut gss_cred_usage_t)
                                                 -> OM_uint32>,
        pub gss_inquire_names_for_mech: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_OID_set)
                                                   -> OM_uint32>,
        pub gss_inquire_context: Option<unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _: *mut gss_OID,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> OM_uint32>,
        pub gss_internal_release_oid: Option<unsafe extern "C" fn(_:
                                                                      *mut OM_uint32,
                                                                  _:
                                                                      *mut gss_OID)
                                                 -> OM_uint32>,
        pub gss_wrap_size_limit: Option<unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _: OM_uint32,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32>,
        pub gss_localname: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                       _: gss_name_t,
                                                       _: gss_const_OID,
                                                       _: gss_buffer_t)
                                      -> OM_uint32>,
        pub gssspi_authorize_localname: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_const_buffer_t,
                                                                    _:
                                                                        gss_const_OID)
                                                   -> OM_uint32>,
        pub gss_export_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_name_t,
                                                         _: gss_buffer_t)
                                        -> OM_uint32>,
        pub gss_duplicate_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                            _: gss_name_t,
                                                            _:
                                                                *mut gss_name_t)
                                           -> OM_uint32>,
        pub gss_store_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                        _: gss_cred_id_t,
                                                        _: gss_cred_usage_t,
                                                        _: gss_OID,
                                                        _: OM_uint32,
                                                        _: OM_uint32,
                                                        _: *mut gss_OID_set,
                                                        _:
                                                            *mut gss_cred_usage_t)
                                       -> OM_uint32>,
        pub gss_inquire_sec_context_by_oid: Option<unsafe extern "C" fn(_:
                                                                            *mut OM_uint32,
                                                                        _:
                                                                            gss_ctx_id_t,
                                                                        _:
                                                                            gss_OID,
                                                                        _:
                                                                            *mut gss_buffer_set_t)
                                                       -> OM_uint32>,
        pub gss_inquire_cred_by_oid: Option<unsafe extern "C" fn(_:
                                                                     *mut OM_uint32,
                                                                 _:
                                                                     gss_cred_id_t,
                                                                 _: gss_OID,
                                                                 _:
                                                                     *mut gss_buffer_set_t)
                                                -> OM_uint32>,
        pub gss_set_sec_context_option: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_ctx_id_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32>,
        pub gssspi_set_cred_option: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut gss_cred_id_t,
                                                                _: gss_OID,
                                                                _:
                                                                    gss_buffer_t)
                                               -> OM_uint32>,
        pub gssspi_mech_invoke: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                            _: gss_OID,
                                                            _: gss_OID,
                                                            _: gss_buffer_t)
                                           -> OM_uint32>,
        pub gss_wrap_aead: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                       _: gss_ctx_id_t,
                                                       _: libc::c_int,
                                                       _: gss_qop_t,
                                                       _: gss_buffer_t,
                                                       _: gss_buffer_t,
                                                       _: *mut libc::c_int,
                                                       _: gss_buffer_t)
                                      -> OM_uint32>,
        pub gss_unwrap_aead: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_ctx_id_t,
                                                         _: gss_buffer_t,
                                                         _: gss_buffer_t,
                                                         _: gss_buffer_t,
                                                         _: *mut libc::c_int,
                                                         _: *mut gss_qop_t)
                                        -> OM_uint32>,
        pub gss_wrap_iov: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                      _: gss_ctx_id_t,
                                                      _: libc::c_int,
                                                      _: gss_qop_t,
                                                      _: *mut libc::c_int,
                                                      _:
                                                          *mut gss_iov_buffer_desc,
                                                      _: libc::c_int)
                                     -> OM_uint32>,
        pub gss_unwrap_iov: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                        _: gss_ctx_id_t,
                                                        _: *mut libc::c_int,
                                                        _: *mut gss_qop_t,
                                                        _:
                                                            *mut gss_iov_buffer_desc,
                                                        _: libc::c_int)
                                       -> OM_uint32>,
        pub gss_wrap_iov_length: Option<unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32>,
        pub gss_complete_auth_token: Option<unsafe extern "C" fn(_:
                                                                     *mut OM_uint32,
                                                                 _:
                                                                     gss_ctx_id_t,
                                                                 _:
                                                                     gss_buffer_t)
                                                -> OM_uint32>,
        pub gss_acquire_cred_impersonate_name: Option<unsafe extern "C" fn(_:
                                                                               *mut OM_uint32,
                                                                           _:
                                                                               gss_cred_id_t,
                                                                           _:
                                                                               gss_name_t,
                                                                           _:
                                                                               OM_uint32,
                                                                           _:
                                                                               gss_OID_set,
                                                                           _:
                                                                               gss_cred_usage_t,
                                                                           _:
                                                                               *mut gss_cred_id_t,
                                                                           _:
                                                                               *mut gss_OID_set,
                                                                           _:
                                                                               *mut OM_uint32)
                                                          -> OM_uint32>,
        pub gss_add_cred_impersonate_name: Option<unsafe extern "C" fn(_:
                                                                           *mut OM_uint32,
                                                                       _:
                                                                           gss_cred_id_t,
                                                                       _:
                                                                           gss_cred_id_t,
                                                                       _:
                                                                           gss_name_t,
                                                                       _:
                                                                           gss_OID,
                                                                       _:
                                                                           gss_cred_usage_t,
                                                                       _:
                                                                           OM_uint32,
                                                                       _:
                                                                           OM_uint32,
                                                                       _:
                                                                           *mut gss_cred_id_t,
                                                                       _:
                                                                           *mut gss_OID_set,
                                                                       _:
                                                                           *mut OM_uint32,
                                                                       _:
                                                                           *mut OM_uint32)
                                                      -> OM_uint32>,
        pub gss_display_name_ext: Option<unsafe extern "C" fn(_:
                                                                  *mut OM_uint32,
                                                              _: gss_name_t,
                                                              _: gss_OID,
                                                              _: gss_buffer_t)
                                             -> OM_uint32>,
        pub gss_inquire_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_name_t,
                                                          _: *mut libc::c_int,
                                                          _: *mut gss_OID,
                                                          _:
                                                              *mut gss_buffer_set_t)
                                         -> OM_uint32>,
        pub gss_get_name_attribute: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _: gss_name_t,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    *mut libc::c_int,
                                                                _:
                                                                    *mut libc::c_int,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    *mut libc::c_int)
                                               -> OM_uint32>,
        pub gss_set_name_attribute: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _: gss_name_t,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    gss_buffer_t)
                                               -> OM_uint32>,
        pub gss_delete_name_attribute: Option<unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       gss_name_t,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32>,
        pub gss_export_name_composite: Option<unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       gss_name_t,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32>,
        pub gss_map_name_to_any: Option<unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: libc::c_int,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_any_t)
                                            -> OM_uint32>,
        pub gss_release_any_name_mapping: Option<unsafe extern "C" fn(_:
                                                                          *mut OM_uint32,
                                                                      _:
                                                                          gss_name_t,
                                                                      _:
                                                                          gss_buffer_t,
                                                                      _:
                                                                          *mut gss_any_t)
                                                     -> OM_uint32>,
        pub gss_pseudo_random: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                           _: gss_ctx_id_t,
                                                           _: libc::c_int,
                                                           _: gss_buffer_t,
                                                           _: ssize_t,
                                                           _: gss_buffer_t)
                                          -> OM_uint32>,
        pub gss_set_neg_mechs: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                           _: gss_cred_id_t,
                                                           _: gss_OID_set)
                                          -> OM_uint32>,
        pub gss_inquire_saslname_for_mech: Option<unsafe extern "C" fn(_:
                                                                           *mut OM_uint32,
                                                                       _:
                                                                           gss_OID,
                                                                       _:
                                                                           gss_buffer_t,
                                                                       _:
                                                                           gss_buffer_t,
                                                                       _:
                                                                           gss_buffer_t)
                                                      -> OM_uint32>,
        pub gss_inquire_mech_for_saslname: Option<unsafe extern "C" fn(_:
                                                                           *mut OM_uint32,
                                                                       _:
                                                                           gss_buffer_t,
                                                                       _:
                                                                           *mut gss_OID)
                                                      -> OM_uint32>,
        pub gss_inquire_attrs_for_mech: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_const_OID,
                                                                    _:
                                                                        *mut gss_OID_set,
                                                                    _:
                                                                        *mut gss_OID_set)
                                                   -> OM_uint32>,
        pub gss_acquire_cred_from: Option<unsafe extern "C" fn(_:
                                                                   *mut OM_uint32,
                                                               _: gss_name_t,
                                                               _: OM_uint32,
                                                               _: gss_OID_set,
                                                               _:
                                                                   gss_cred_usage_t,
                                                               _:
                                                                   gss_const_key_value_set_t,
                                                               _:
                                                                   *mut gss_cred_id_t,
                                                               _:
                                                                   *mut gss_OID_set,
                                                               _:
                                                                   *mut OM_uint32)
                                              -> OM_uint32>,
        pub gss_store_cred_into: Option<unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _: gss_OID,
                                                             _: OM_uint32,
                                                             _: OM_uint32,
                                                             _:
                                                                 gss_const_key_value_set_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut gss_cred_usage_t)
                                            -> OM_uint32>,
        pub gssspi_acquire_cred_with_password: Option<unsafe extern "C" fn(_:
                                                                               *mut OM_uint32,
                                                                           _:
                                                                               gss_name_t,
                                                                           _:
                                                                               gss_buffer_t,
                                                                           _:
                                                                               OM_uint32,
                                                                           _:
                                                                               gss_OID_set,
                                                                           _:
                                                                               libc::c_int,
                                                                           _:
                                                                               *mut gss_cred_id_t,
                                                                           _:
                                                                               *mut gss_OID_set,
                                                                           _:
                                                                               *mut OM_uint32)
                                                          -> OM_uint32>,
        pub gss_export_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_cred_id_t,
                                                         _: gss_buffer_t)
                                        -> OM_uint32>,
        pub gss_import_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_buffer_t,
                                                         _:
                                                             *mut gss_cred_id_t)
                                        -> OM_uint32>,
        pub gssspi_import_sec_context_by_mech: Option<unsafe extern "C" fn(_:
                                                                               *mut OM_uint32,
                                                                           _:
                                                                               gss_OID,
                                                                           _:
                                                                               gss_buffer_t,
                                                                           _:
                                                                               *mut gss_ctx_id_t)
                                                          -> OM_uint32>,
        pub gssspi_import_name_by_mech: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_name_t)
                                                   -> OM_uint32>,
        pub gssspi_import_cred_by_mech: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_cred_id_t)
                                                   -> OM_uint32>,
        pub gss_get_mic_iov: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_ctx_id_t,
                                                         _: gss_qop_t,
                                                         _:
                                                             *mut gss_iov_buffer_desc,
                                                         _: libc::c_int)
                                        -> OM_uint32>,
        pub gss_verify_mic_iov: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                            _: gss_ctx_id_t,
                                                            _: *mut gss_qop_t,
                                                            _:
                                                                *mut gss_iov_buffer_desc,
                                                            _: libc::c_int)
                                           -> OM_uint32>,
        pub gss_get_mic_iov_length: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    gss_ctx_id_t,
                                                                _: gss_qop_t,
                                                                _:
                                                                    *mut gss_iov_buffer_desc,
                                                                _:
                                                                    libc::c_int)
                                               -> OM_uint32>,
        pub gssspi_query_meta_data: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    gss_const_OID,
                                                                _:
                                                                    gss_cred_id_t,
                                                                _:
                                                                    *mut gss_ctx_id_t,
                                                                _: gss_name_t,
                                                                _: OM_uint32,
                                                                _:
                                                                    gss_buffer_t)
                                               -> OM_uint32>,
        pub gssspi_exchange_meta_data: Option<unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       gss_const_OID,
                                                                   _:
                                                                       gss_cred_id_t,
                                                                   _:
                                                                       *mut gss_ctx_id_t,
                                                                   _:
                                                                       gss_name_t,
                                                                   _:
                                                                       OM_uint32,
                                                                   _:
                                                                       gss_const_buffer_t)
                                                  -> OM_uint32>,
        pub gssspi_query_mechanism_info: Option<unsafe extern "C" fn(_:
                                                                         *mut OM_uint32,
                                                                     _:
                                                                         gss_const_OID,
                                                                     _:
                                                                         *mut libc::c_uchar)
                                                    -> OM_uint32>,
    }
    #[c2rust::src_loc = "36:1"]
    pub type gss_union_name_desc = gss_name_struct;
    #[c2rust::src_loc = "36:1"]
    pub type gss_union_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "60:1"]
    pub type gss_union_cred_t = *mut gss_cred_id_struct;
    use super::gssapi_h::{gss_OID, gss_buffer_t, gss_name_t, gss_cred_id_t,
                          gss_ctx_id_t, gss_OID_desc, OM_uint32, gss_OID_set,
                          gss_channel_bindings_t, gss_qop_t, gss_cred_usage_t,
                          gss_const_OID, gss_const_buffer_t};
    use super::gssapi_ext_h::{gss_buffer_set_t, gss_iov_buffer_desc,
                              gss_any_t, gss_const_key_value_set_t};
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "766:1"]
        pub fn gssint_get_public_oid(internal_oid: gss_const_OID) -> gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "769:1"]
        pub fn gssint_get_mechanism(_: gss_const_OID) -> gss_mechanism;
    }
    /* _GSS_MECHGLUEP_H */
    /* Use this to map an errno value or com_err error code being
   generated within the mechglue code (e.g., by calling generic oid
   ops).  Any errno or com_err values produced by mech operations
   should be processed with map_error.  This means they'll be stored
   separately even if the mech uses com_err, because we can't assume
   that it will use com_err.  */
    /* Use this to map an error code that was returned from a mech
   operation; the mech will be asked to produce the associated error
   messages.

   Remember that if the minor status code cannot be returned to the
   caller (e.g., if it's stuffed in an automatic variable and then
   ignored), then we don't care about producing a mapping.  */
    /* qop_state */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:25"]
pub mod gssapi_h {
    /* This is the gssapi.h prologue. */
/* no xom.h */
/* End of gssapi.h prologue. */
/* -*- mode: c; indent-tabs-mode: nil -*- */
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
 * Determine platform-dependent configuration.
 */
    /* __cplusplus */
    /*
 * First, include stddef.h to get size_t defined.
 */
    /*
 * POSIX says that sys/types.h is where size_t is defined.
 */
    /*
 * $Id$
 */
    /*
 * First, define the three platform-dependent pointer types.
 */
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    /*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
    /* OM_STRING */
    /*
 * We can't use X/Open definitions, so roll our own.
 */
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    /* OM_STRING */
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "122:16"]
    pub struct gss_channel_bindings_struct {
        pub initiator_addrtype: OM_uint32,
        pub initiator_address: gss_buffer_desc,
        pub acceptor_addrtype: OM_uint32,
        pub acceptor_address: gss_buffer_desc,
        pub application_data: gss_buffer_desc,
    }
    #[c2rust::src_loc = "122:1"]
    pub type gss_channel_bindings_t = *mut gss_channel_bindings_struct;
    /*
 * For now, define a QOP-type as an OM_uint32 (pending resolution of ongoing
 * discussions).
 */
    #[c2rust::src_loc = "134:1"]
    pub type gss_qop_t = OM_uint32;
    #[c2rust::src_loc = "135:1"]
    pub type gss_cred_usage_t = libc::c_int;
    /* mech_set */
    /* XXXX these are not part of the GSSAPI C bindings!  (but should be) */
    /* XXXX This is a necessary evil until the spec is fixed */
    /*
 * RFC 5587
 */
    #[c2rust::src_loc = "845:1"]
    pub type gss_const_buffer_t = *const gss_buffer_desc;
    #[c2rust::src_loc = "850:1"]
    pub type gss_const_OID = *const gss_OID_desc;
    use super::mglueP_h::{gss_name_struct, gss_cred_id_struct};
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        /* input_name */
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        /* context_handle */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "673:1"]
        pub fn gss_release_oid(_: *mut OM_uint32, _: *mut gss_OID)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:25"]
pub mod gssapi_ext_h {
    /* acceptor_time_rec */
    /*
 * GGF extensions
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "134:16"]
    pub struct gss_buffer_set_desc_struct {
        pub count: size_t,
        pub elements: *mut gss_buffer_desc,
    }
    #[c2rust::src_loc = "134:1"]
    pub type gss_buffer_set_t = *mut gss_buffer_set_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "248:16"]
    pub struct gss_iov_buffer_desc_struct {
        pub type_0: OM_uint32,
        pub buffer: gss_buffer_desc,
    }
    #[c2rust::src_loc = "248:1"]
    pub type gss_iov_buffer_desc = gss_iov_buffer_desc_struct;
    #[c2rust::src_loc = "488:1"]
    pub type gss_any_t = *mut gss_any;
    /* Credential store extensions */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "530:8"]
    pub struct gss_key_value_element_struct {
        pub key: *const libc::c_char,
        pub value: *const libc::c_char,
    }
    #[c2rust::src_loc = "534:1"]
    pub type gss_key_value_element_desc = gss_key_value_element_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "536:8"]
    pub struct gss_key_value_set_struct {
        pub count: OM_uint32,
        pub elements: *mut gss_key_value_element_desc,
    }
    #[c2rust::src_loc = "540:1"]
    pub type gss_key_value_set_desc = gss_key_value_set_struct;
    #[c2rust::src_loc = "541:1"]
    pub type gss_const_key_value_set_t = *const gss_key_value_set_desc;
    use super::stddef_h::size_t;
    use super::gssapi_h::{gss_buffer_desc, OM_uint32};
    extern "C" {
        #[c2rust::src_loc = "488:16"]
        pub type gss_any;
        #[no_mangle]
        #[c2rust::src_loc = "150:1"]
        pub fn gss_release_buffer_set(_: *mut OM_uint32,
                                      _: *mut gss_buffer_set_t) -> OM_uint32;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:25"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "561:5"]
    pub struct C2RustUnnamed {
        pub i: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "574:5"]
    pub struct C2RustUnnamed_0 {
        pub i: uint32_t,
    }
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
    #[c2rust::src_loc = "554:1"]
    pub unsafe extern "C" fn store_16_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = __bswap_16(val as __uint16_t);
    }
    #[inline]
    #[c2rust::src_loc = "567:1"]
    pub unsafe extern "C" fn store_32_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed_0)).i = __bswap_32(val);
    }
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    use super::byteswap_h::{__bswap_16, __bswap_32};
    use super::types_h::__uint16_t;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/string.h:25"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/byteswap.h:25"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
        return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
                    |
                    (__bsx as libc::c_int & 0xff as libc::c_int) <<
                        8 as libc::c_int) as __uint16_t;
    }
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
        return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
                   (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
                   (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
                   (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
    }
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/stdlib.h:25"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_alloc.h:25"]
pub mod gssapi_alloc_h {
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn gssalloc_malloc(mut size: size_t)
     -> *mut libc::c_void {
        return malloc(size);
    }
    use super::stddef_h::size_t;
    use super::stdlib_h::malloc;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:25"]
pub mod gssapiP_generic_h {
    use super::gssapi_h::{OM_uint32, gss_OID_desc, gss_OID};
    extern "C" {
        /* set */
        #[no_mangle]
        #[c2rust::src_loc = "206:1"]
        pub fn generic_gss_copy_oid(_: *mut OM_uint32, _: *const gss_OID_desc,
                                    _: *mut gss_OID) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn gssint_mecherrmap_map(minor: OM_uint32,
                                     oid: *const gss_OID_desc) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "263:1"]
        pub fn gssint_mecherrmap_map_errcode(errcode: OM_uint32) -> OM_uint32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
pub use self::types_h::{__uint16_t, __uint32_t, __ssize_t};
pub use self::stdint_uintn_h::{uint16_t, uint32_t};
pub use self::stddef_h::size_t;
pub use self::sys_types_h::ssize_t;
pub use self::mglueP_h::{gss_name_struct, gss_cred_id_struct,
                         gss_union_ctx_id_t, gss_union_ctx_id_struct,
                         gss_mechanism, gss_config, gss_union_name_desc,
                         gss_union_name_t, gss_union_cred_t,
                         gssint_get_public_oid, gssint_get_mechanism};
pub use self::gssapi_h::{gss_name_t, gss_OID, gss_OID_desc_struct, OM_uint32,
                         gss_uint32, gss_buffer_t, gss_buffer_desc_struct,
                         gss_cred_id_t, gss_ctx_id_t, gss_OID_desc,
                         gss_OID_set_desc_struct, gss_OID_set,
                         gss_buffer_desc, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_qop_t, gss_cred_usage_t,
                         gss_const_buffer_t, gss_const_OID, gss_ctx_id_struct,
                         gss_release_buffer, gss_release_oid};
pub use self::gssapi_ext_h::{gss_buffer_set_desc_struct, gss_buffer_set_t,
                             gss_iov_buffer_desc_struct, gss_iov_buffer_desc,
                             gss_any_t, gss_key_value_element_struct,
                             gss_key_value_element_desc,
                             gss_key_value_set_struct, gss_key_value_set_desc,
                             gss_const_key_value_set_t, gss_any,
                             gss_release_buffer_set};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, store_16_be,
                              store_32_be};
use self::string_h::{memcpy, memset, memcmp};
pub use self::byteswap_h::{__bswap_16, __bswap_32};
use self::stdlib_h::{malloc, calloc, free};
pub use self::gssapi_alloc_h::gssalloc_malloc;
use self::gssapiP_generic_h::{generic_gss_copy_oid, gssint_mecherrmap_map,
                              gssint_mecherrmap_map_errcode};
/*
 * This file contains the support routines for the glue layer.
 */
/*
 * get_der_length: Givin a pointer to a buffer that contains a DER encoded
 * length, decode the length updating the buffer to point to the character
 * after the DER encoding. The parameter bytes will point to the number of
 * bytes that made up the DER encoding of the length originally pointed to
 * by the buffer. Note we return -1 on error.
 */
#[no_mangle]
#[c2rust::src_loc = "48:1"]
pub unsafe extern "C" fn gssint_get_der_length(mut buf:
                                                   *mut *mut libc::c_uchar,
                                               mut buf_len: libc::c_uint,
                                               mut bytes: *mut libc::c_uint)
 -> libc::c_int {
    /* p points to the beginning of the buffer */
    let mut p: *mut libc::c_uchar = *buf;
    let mut length: libc::c_int = 0;
    let mut new_length: libc::c_int = 0;
    let mut octets: libc::c_uint = 0;
    if buf_len < 1 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    /* We should have at least one byte */
    *bytes = 1 as libc::c_int as libc::c_uint;
    /*
     * If the High order bit is not set then the length is just the value
     * of *p.
     */
    if (*p as libc::c_int) < 128 as libc::c_int {
        *buf = p.offset(1 as libc::c_int as isize);
        return *p as libc::c_int /* Advance the buffer */
        /* return the length */
    }
    /*
     * if the High order bit is set, then the low order bits represent
     * the number of bytes that contain the DER encoding of the length.
     */
    let fresh0 = p;
    p = p.offset(1);
    octets = (*fresh0 as libc::c_int & 0x7f as libc::c_int) as libc::c_uint;
    *bytes = (*bytes).wrapping_add(octets);
    /* See if the supplied buffer contains enough bytes for the length. */
    if octets > buf_len.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        return -(1 as libc::c_int)
    }
    /*
     * Calculate a multibyte length. The length is encoded as an
     * unsigned integer base 256.
     */
    length = 0 as libc::c_int;
    while octets != 0 {
        let fresh1 = p;
        p = p.offset(1);
        new_length = (length << 8 as libc::c_int) + *fresh1 as libc::c_int;
        if new_length < length {
            /* overflow */
            return -(1 as libc::c_int)
        } /* Advance the buffer */
        length = new_length;
        octets = octets.wrapping_sub(1)
    }
    *buf = p;
    return length;
}
/*
 * der_length_size: Return the number of bytes to encode a given length.
 */
#[no_mangle]
#[c2rust::src_loc = "102:1"]
pub unsafe extern "C" fn gssint_der_length_size(mut len: libc::c_uint)
 -> libc::c_uint {
    let mut i: libc::c_int = 0;
    if len < 128 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as libc::c_uint
    }
    i = 0 as libc::c_int;
    while len != 0 { len >>= 8 as libc::c_int; i += 1 }
    return (i + 1 as libc::c_int) as libc::c_uint;
}
/* minor_status */
/* oid set */
/* new oid set */
/* name_type */
/* minor_status */
/* name_type */
/* mech */
/*
 * Sun extensions to GSS-API v2
 */
/* buf */
/* buf_len */
/* bytes */
/* len */
/*
 * put_der_length: Encode the supplied length into the buffer pointed to
 * by buf. max_length represents the maximum length of the buffer pointed
 * to by buff. We will advance buf to point to the character after the newly
 * DER encoded length. We return 0 on success or -l it the length cannot
 * be encoded in max_len characters.
 */
#[no_mangle]
#[c2rust::src_loc = "124:1"]
pub unsafe extern "C" fn gssint_put_der_length(mut length: libc::c_uint,
                                               mut buf:
                                                   *mut *mut libc::c_uchar,
                                               mut max_len: libc::c_uint)
 -> libc::c_int {
    let mut s: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    /* Oops */
    if buf.is_null() || max_len < 1 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    s = *buf;
    /* Single byte is the length */
    if length < 128 as libc::c_int as libc::c_uint {
        let fresh2 = s;
        s = s.offset(1);
        *fresh2 = length as libc::c_uchar;
        *buf = s;
        return 0 as libc::c_int
    }
    /* First byte contains the number of octets */
    p = s.offset(1 as libc::c_int as isize);
    /* Running total of the DER encoding length */
    buf_len = 0 as libc::c_int as libc::c_uint;
    /*
     * Encode MSB first. We do the encoding by setting a shift
     * factor to MSO_BIT (24 for 32 bit words) and then shifting the length
     * by the factor. We then encode the resulting low order byte.
     * We subtract 8 from the shift factor and repeat to ecnode the next
     * byte. We stop when the shift factor is zero or we've run out of
     * buffer to encode into.
     */
    first = 0 as libc::c_int;
    i =
        (8 as libc::c_int as
             libc::c_ulong).wrapping_mul((::std::mem::size_of::<libc::c_int>()
                                              as
                                              libc::c_ulong).wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong))
            as libc::c_int;
    while i >= 0 as libc::c_int && buf_len <= max_len {
        let mut v: libc::c_uint = 0;
        v = length >> i & 0xff as libc::c_int as libc::c_uint;
        if v != 0 || first != 0 {
            buf_len = buf_len.wrapping_add(1 as libc::c_int as libc::c_uint);
            let fresh3 = p;
            p = p.offset(1);
            *fresh3 = v as libc::c_uchar;
            first = 1 as libc::c_int
        }
        i -= 8 as libc::c_int
    }
    if i >= 0 as libc::c_int {
        /* buffer overflow */
        return -(1 as libc::c_int)
    }
    /*
     * We go back now and set the first byte to be the length with
     * the high order bit set.
     */
    *s = (buf_len | 0x80 as libc::c_int as libc::c_uint) as libc::c_uchar;
    *buf = p;
    return 0 as libc::c_int;
}
/*
 *  glue routine for get_mech_type
 *
 */
#[no_mangle]
#[c2rust::src_loc = "187:1"]
pub unsafe extern "C" fn gssint_get_mech_type_oid(mut OID: gss_OID,
                                                  mut token: gss_buffer_t)
 -> OM_uint32 {
    let mut buffer_ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buflen: size_t = 0;
    let mut lenbytes: size_t = 0;
    let mut length: size_t = 0;
    let mut oidlen: size_t = 0;
    /*
     * This routine reads the prefix of "token" in order to determine
     * its mechanism type. It assumes the encoding suggested in
     * Appendix B of RFC 1508. This format starts out as follows :
     *
     * tag for APPLICATION 0, Sequence[constructed, definite length]
     * length of remainder of token
     * tag of OBJECT IDENTIFIER
     * length of mechanism OID
     * encoding of mechanism OID
     * <the rest of the token>
     *
     * Numerically, this looks like :
     *
     * 0x60
     * <length> - could be multiple bytes
     * 0x06
     * <length> - assume only one byte, hence OID length < 127
     * <mech OID bytes>
     *
     * The routine fills in the OID value and returns an error as necessary.
     */
    if OID.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    if token.is_null() || (*token).value.is_null() {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /* Skip past the APP/Sequnce byte and the token length */
    buffer_ptr = (*token).value as *mut libc::c_uchar;
    buflen = (*token).length;
    if buflen < 2 as libc::c_int as libc::c_ulong ||
           {
               let fresh4 = buffer_ptr;
               buffer_ptr = buffer_ptr.offset(1);
               (*fresh4 as libc::c_int) != 0x60 as libc::c_int
           } {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    let fresh5 = buffer_ptr;
    buffer_ptr = buffer_ptr.offset(1);
    length = *fresh5 as size_t;
    buflen =
        (buflen as
             libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong) as
            size_t as size_t;
    /* check if token length is null */
    if length == 0 as libc::c_int as libc::c_ulong {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if length & 0x80 as libc::c_int as libc::c_ulong != 0 {
        lenbytes = length & 0x7f as libc::c_int as libc::c_ulong;
        if lenbytes > 4 as libc::c_int as libc::c_ulong || lenbytes > buflen {
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        buffer_ptr = buffer_ptr.offset(lenbytes as isize);
        buflen =
            (buflen as libc::c_ulong).wrapping_sub(lenbytes) as size_t as
                size_t
    }
    if buflen < 2 as libc::c_int as libc::c_ulong ||
           {
               let fresh6 = buffer_ptr;
               buffer_ptr = buffer_ptr.offset(1);
               (*fresh6 as libc::c_int) != 0x6 as libc::c_int
           } {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    let fresh7 = buffer_ptr;
    buffer_ptr = buffer_ptr.offset(1);
    oidlen = *fresh7 as size_t;
    buflen =
        (buflen as
             libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong) as
            size_t as size_t;
    if oidlen > 0x7f as libc::c_int as libc::c_ulong || oidlen > buflen {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    (*OID).length = oidlen as OM_uint32;
    (*OID).elements = buffer_ptr as *mut libc::c_void;
    return 0 as libc::c_int as OM_uint32;
}
/*
 * The following mechanisms do not always identify themselves
 * per the GSS-API specification, when interoperating with MS
 * peers. We include the OIDs here so we do not have to ilnk
 * with the mechanism.
 */
#[c2rust::src_loc = "263:21"]
static mut gss_ntlm_mechanism_oid_desc: gss_OID_desc =
    {
        let mut init =
            gss_OID_desc_struct{length: 10 as libc::c_int as OM_uint32,
                                elements:
                                    b"+\x06\x01\x04\x01\x827\x02\x02\n\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_void,};
        init
    };
#[c2rust::src_loc = "265:21"]
static mut gss_spnego_mechanism_oid_desc: gss_OID_desc =
    {
        let mut init =
            gss_OID_desc_struct{length: 6 as libc::c_int as OM_uint32,
                                elements:
                                    b"+\x06\x01\x05\x05\x02\x00" as *const u8
                                        as *const libc::c_char as
                                        *mut libc::c_void,};
        init
    };
#[c2rust::src_loc = "267:21"]
static mut gss_krb5_mechanism_oid_desc: gss_OID_desc =
    {
        let mut init =
            gss_OID_desc_struct{length: 9 as libc::c_int as OM_uint32,
                                elements:
                                    b"*\x86H\x86\xf7\x12\x01\x02\x02\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_void,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "272:1"]
pub unsafe extern "C" fn gssint_get_mech_type(mut OID: gss_OID,
                                              mut token: gss_buffer_t)
 -> OM_uint32 {
    /* Check for interoperability exceptions */
    if (*token).length >=
           ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong &&
           memcmp((*token).value,
                  b"NTLMSSP\x00" as *const u8 as *const libc::c_char as
                      *const libc::c_void,
                  ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
               == 0 as libc::c_int {
        *OID = gss_ntlm_mechanism_oid_desc
    } else if (*token).length != 0 as libc::c_int as libc::c_ulong &&
                  *((*token).value as
                        *mut libc::c_char).offset(0 as libc::c_int as isize)
                      as libc::c_int == 0x6e as libc::c_int {
        /* Could be a raw AP-REQ (check for APPLICATION tag) */
        *OID = gss_krb5_mechanism_oid_desc
    } else if (*token).length == 0 as libc::c_int as libc::c_ulong {
        *OID = gss_spnego_mechanism_oid_desc
    } else { return gssint_get_mech_type_oid(OID, token) }
    return 0 as libc::c_int as OM_uint32;
}
#[c2rust::src_loc = "294:1"]
unsafe extern "C" fn import_internal_attributes(mut minor: *mut OM_uint32,
                                                mut dmech: gss_mechanism,
                                                mut sname: gss_union_name_t,
                                                mut dname: gss_name_t)
 -> OM_uint32 {
    let mut major: OM_uint32 = 0;
    let mut tmpMinor: OM_uint32 = 0;
    let mut smech: gss_mechanism = 0 as *mut gss_config;
    let mut attrs: gss_buffer_set_t = 0 as gss_buffer_set_t;
    let mut i: size_t = 0;
    if (*sname).mech_name.is_null() {
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    smech = gssint_get_mechanism((*sname).mech_type as gss_const_OID);
    if smech.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if (*smech).gss_inquire_name.is_none() ||
           (*smech).gss_get_name_attribute.is_none() {
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if (*dmech).gss_set_name_attribute.is_none() {
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    major =
        (*smech).gss_inquire_name.expect("non-null function pointer")(minor,
                                                                      (*sname).mech_name,
                                                                      0 as
                                                                          *mut libc::c_int,
                                                                      0 as
                                                                          *mut gss_OID,
                                                                      &mut attrs);
    if major &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 || attrs.is_null() {
        gss_release_buffer_set(&mut tmpMinor, &mut attrs);
        return major
    }
    i = 0 as libc::c_int as size_t;
    while i < (*attrs).count {
        let mut more: libc::c_int = -(1 as libc::c_int);
        while more != 0 as libc::c_int {
            let mut value: gss_buffer_desc =
                gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
            let mut display_value: gss_buffer_desc =
                gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
            let mut authenticated: libc::c_int = 0;
            let mut complete: libc::c_int = 0;
            major =
                (*smech).gss_get_name_attribute.expect("non-null function pointer")(minor,
                                                                                    (*sname).mech_name,
                                                                                    &mut *(*attrs).elements.offset(i
                                                                                                                       as
                                                                                                                       isize),
                                                                                    &mut authenticated,
                                                                                    &mut complete,
                                                                                    &mut value,
                                                                                    &mut display_value,
                                                                                    &mut more);
            if major &
                   ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
                        |
                        (0o377 as libc::c_ulong as OM_uint32) <<
                            16 as libc::c_int) != 0 {
                continue ;
            }
            if authenticated != 0 {
                (*dmech).gss_set_name_attribute.expect("non-null function pointer")(minor,
                                                                                    dname,
                                                                                    complete,
                                                                                    &mut *(*attrs).elements.offset(i
                                                                                                                       as
                                                                                                                       isize),
                                                                                    &mut value);
            }
            gss_release_buffer(&mut tmpMinor, &mut value);
            gss_release_buffer(&mut tmpMinor, &mut display_value);
        }
        i = i.wrapping_add(1)
    }
    gss_release_buffer_set(&mut tmpMinor, &mut attrs);
    return 0 as libc::c_int as OM_uint32;
}
/*
 *  Internal routines to get and release an internal mechanism name
 */
#[no_mangle]
#[c2rust::src_loc = "360:1"]
pub unsafe extern "C" fn gssint_import_internal_name(mut minor_status:
                                                         *mut OM_uint32,
                                                     mut mech_type: gss_OID,
                                                     mut union_name:
                                                         gss_union_name_t,
                                                     mut internal_name:
                                                         *mut gss_name_t)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    let mut tmpMinor: OM_uint32 = 0;
    let mut mech: gss_mechanism = 0 as *mut gss_config;
    let mut public_mech: gss_OID = 0 as *mut gss_OID_desc_struct;
    mech = gssint_get_mechanism(mech_type as gss_const_OID);
    if mech.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /*
     * If we are importing a name for the same mechanism, and the
     * mechanism implements gss_duplicate_name, then use that.
     */
    if !(*union_name).mech_type.is_null() &&
           !(*union_name).mech_name.is_null() &&
           ((*(*union_name).mech_type).length == (*mech_type).length &&
                memcmp((*(*union_name).mech_type).elements,
                       (*mech_type).elements,
                       (*(*union_name).mech_type).length as libc::c_ulong) ==
                    0 as libc::c_int) && (*mech).gss_duplicate_name.is_some()
       {
        status =
            (*mech).gss_duplicate_name.expect("non-null function pointer")(minor_status,
                                                                           (*union_name).mech_name,
                                                                           internal_name);
        if status != (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
            if status != 0 as libc::c_int as libc::c_uint {
                *minor_status =
                    gssint_mecherrmap_map(*minor_status,
                                          &mut (*mech).mech_type)
            }
            return status
        }
    }
    if (*mech).gssspi_import_name_by_mech.is_some() {
        public_mech = gssint_get_public_oid(mech_type as gss_const_OID);
        status =
            (*mech).gssspi_import_name_by_mech.expect("non-null function pointer")(minor_status,
                                                                                   public_mech,
                                                                                   (*union_name).external_name,
                                                                                   (*union_name).name_type,
                                                                                   internal_name)
    } else if (*mech).gss_import_name.is_some() {
        status =
            (*mech).gss_import_name.expect("non-null function pointer")(minor_status,
                                                                        (*union_name).external_name,
                                                                        (*union_name).name_type,
                                                                        internal_name)
    } else { return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int }
    if status == 0 as libc::c_int as libc::c_uint {
        /* Attempt to round-trip attributes */
        import_internal_attributes(&mut tmpMinor, mech, union_name,
                                   *internal_name);
    } else {
        *minor_status =
            gssint_mecherrmap_map(*minor_status, &mut (*mech).mech_type)
    }
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "417:1"]
pub unsafe extern "C" fn gssint_export_internal_name(mut minor_status:
                                                         *mut OM_uint32,
                                                     mech_type: gss_OID,
                                                     internal_name:
                                                         gss_name_t,
                                                     mut name_buf:
                                                         gss_buffer_t)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    let mut mech: gss_mechanism = 0 as *mut gss_config;
    let mut dispName: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut nameOid: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let tokId: [libc::c_uchar; 3] =
        *::std::mem::transmute::<&[u8; 3],
                                 &[libc::c_uchar; 3]>(b"\x04\x01\x00");
    let tokIdLen: libc::c_uint = 2 as libc::c_int as libc::c_uint;
    let mechOidLenLen: libc::c_int = 2 as libc::c_int;
    let mechOidTagLen: libc::c_int = 1 as libc::c_int;
    let nameLenLen: libc::c_int = 4 as libc::c_int;
    let mut mechOidDERLen: libc::c_int = 0 as libc::c_int;
    let mut mechOidLen: libc::c_int = 0 as libc::c_int;
    mech = gssint_get_mechanism(mech_type as gss_const_OID);
    if mech.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if (*mech).gss_export_name.is_some() {
        status =
            (*mech).gss_export_name.expect("non-null function pointer")(minor_status,
                                                                        internal_name,
                                                                        name_buf);
        if status != 0 as libc::c_int as libc::c_uint {
            *minor_status =
                gssint_mecherrmap_map(*minor_status, &mut (*mech).mech_type)
        }
        return status
    }
    /*
     * if we are here it is because the mechanism does not provide
     * a gss_export_name so we will use our implementation.  We
     * do required that the mechanism define a gss_display_name.
     */
    if (*mech).gss_display_name.is_none() {
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /*
     * NOTE: RFC2743 (section 3.2) governs the format of the outer
     *	 wrapper of exported names; the mechanisms' specs govern
     *	 the format of the inner portion of the exported name
     *	 and, for some (e.g., RFC1964, the Kerberos V mech), a
     *	 generic default as implemented here will do.
     *
     * The outer wrapper of an exported MN is: 2-octet tok Id
     * (0x0401) + 2-octet network-byte order mech OID length + mech
     * oid (in DER format, including DER tag and DER length) +
     * 4-octet network-byte order length of inner portion + inner
     * portion.
     *
     * For the Kerberos V mechanism the inner portion of an exported
     * MN is the display name string and ignores the name type OID
     * altogether.  And we hope this will be so for any future
     * mechanisms also, so that factoring name export/import out of
     * the mech and into libgss pays off.
     */
    status =
        (*mech).gss_display_name.expect("non-null function pointer")(minor_status,
                                                                     internal_name,
                                                                     &mut dispName,
                                                                     &mut nameOid);
    if status != 0 as libc::c_int as libc::c_uint {
        *minor_status =
            gssint_mecherrmap_map(*minor_status, &mut (*mech).mech_type);
        return status
    }
    /* determine the size of the buffer needed */
    mechOidDERLen =
        gssint_der_length_size((*mech_type).length) as libc::c_int;
    (*name_buf).length =
        (tokIdLen.wrapping_add(mechOidLenLen as
                                   libc::c_uint).wrapping_add(mechOidTagLen as
                                                                  libc::c_uint).wrapping_add(mechOidDERLen
                                                                                                 as
                                                                                                 libc::c_uint).wrapping_add((*mech_type).length).wrapping_add(nameLenLen
                                                                                                                                                                  as
                                                                                                                                                                  libc::c_uint)
             as libc::c_ulong).wrapping_add(dispName.length);
    (*name_buf).value = gssalloc_malloc((*name_buf).length);
    if (*name_buf).value == 0 as *mut libc::c_void {
        (*name_buf).length = 0 as libc::c_int as size_t;
        gss_release_buffer(&mut status, &mut dispName);
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /* now create the name ..... */
    buf = (*name_buf).value as *mut libc::c_uchar;
    memset((*name_buf).value, 0 as libc::c_int, (*name_buf).length);
    memcpy(buf as *mut libc::c_void, tokId.as_ptr() as *const libc::c_void,
           tokIdLen as libc::c_ulong);
    buf = buf.offset(tokIdLen as isize);
    /* spec allows only 2 bytes for the mech oid length */
    mechOidLen =
        ((mechOidDERLen + mechOidTagLen) as
             libc::c_uint).wrapping_add((*mech_type).length) as libc::c_int;
    store_16_be(mechOidLen as libc::c_uint, buf as *mut libc::c_void);
    buf = buf.offset(2 as libc::c_int as isize);
    /*
     * DER Encoding of mech OID contains OID Tag (0x06), length and
     * mech OID value
     */
    let fresh8 = buf;
    buf = buf.offset(1);
    *fresh8 = 0x6 as libc::c_int as libc::c_uchar;
    if gssint_put_der_length((*mech_type).length, &mut buf,
                             (*name_buf).length.wrapping_sub(tokIdLen as
                                                                 libc::c_ulong).wrapping_sub(2
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong)
                                 as libc::c_uint) != 0 as libc::c_int {
        (*name_buf).length = 0 as libc::c_int as size_t;
        free((*name_buf).value);
        gss_release_buffer(&mut status, &mut dispName);
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    memcpy(buf as *mut libc::c_void, (*mech_type).elements,
           (*mech_type).length as libc::c_ulong);
    buf = buf.offset((*mech_type).length as isize);
    /* spec designates the next 4 bytes for the name length */
    store_32_be(dispName.length as libc::c_uint, buf as *mut libc::c_void);
    buf = buf.offset(4 as libc::c_int as isize);
    /* for the final ingredient - add the name from gss_display_name */
    memcpy(buf as *mut libc::c_void, dispName.value, dispName.length);
    /* release the buffer obtained from gss_display_name */
    gss_release_buffer(minor_status, &mut dispName);
    return 0 as libc::c_int as OM_uint32;
}
/*  gssint_export_internal_name */
#[no_mangle]
#[c2rust::src_loc = "536:1"]
pub unsafe extern "C" fn gssint_display_internal_name(mut minor_status:
                                                          *mut OM_uint32,
                                                      mut mech_type: gss_OID,
                                                      mut internal_name:
                                                          gss_name_t,
                                                      mut external_name:
                                                          gss_buffer_t,
                                                      mut name_type:
                                                          *mut gss_OID)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    let mut mech: gss_mechanism = 0 as *mut gss_config;
    mech = gssint_get_mechanism(mech_type as gss_const_OID);
    if !mech.is_null() {
        if (*mech).gss_display_name.is_some() {
            status =
                (*mech).gss_display_name.expect("non-null function pointer")(minor_status,
                                                                             internal_name,
                                                                             external_name,
                                                                             name_type);
            if status != 0 as libc::c_int as libc::c_uint {
                *minor_status =
                    gssint_mecherrmap_map(*minor_status,
                                          &mut (*mech).mech_type)
            }
        } else {
            status = (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        return status
    }
    return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "566:1"]
pub unsafe extern "C" fn gssint_release_internal_name(mut minor_status:
                                                          *mut OM_uint32,
                                                      mut mech_type: gss_OID,
                                                      mut internal_name:
                                                          *mut gss_name_t)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    let mut mech: gss_mechanism = 0 as *mut gss_config;
    mech = gssint_get_mechanism(mech_type as gss_const_OID);
    if !mech.is_null() {
        if (*mech).gss_release_name.is_some() {
            status =
                (*mech).gss_release_name.expect("non-null function pointer")(minor_status,
                                                                             internal_name);
            if status != 0 as libc::c_int as libc::c_uint {
                *minor_status =
                    gssint_mecherrmap_map(*minor_status,
                                          &mut (*mech).mech_type)
            }
        } else {
            status = (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        return status
    }
    return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "591:1"]
pub unsafe extern "C" fn gssint_delete_internal_sec_context(mut minor_status:
                                                                *mut OM_uint32,
                                                            mut mech_type:
                                                                gss_OID,
                                                            mut internal_ctx:
                                                                *mut gss_ctx_id_t,
                                                            mut output_token:
                                                                gss_buffer_t)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    let mut mech: gss_mechanism = 0 as *mut gss_config;
    mech = gssint_get_mechanism(mech_type as gss_const_OID);
    if !mech.is_null() {
        if (*mech).gss_delete_sec_context.is_some() {
            status =
                (*mech).gss_delete_sec_context.expect("non-null function pointer")(minor_status,
                                                                                   internal_ctx,
                                                                                   output_token)
        } else {
            status = (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        return status
    }
    return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
}
/*
 * This function converts an internal gssapi name to a union gssapi
 * name.  Note that internal_name should be considered "consumed" by
 * this call, whether or not we return an error.
 */
#[no_mangle]
#[c2rust::src_loc = "623:1"]
pub unsafe extern "C" fn gssint_convert_name_to_union_name(mut minor_status:
                                                               *mut OM_uint32,
                                                           mut mech:
                                                               gss_mechanism,
                                                           mut internal_name:
                                                               gss_name_t,
                                                           mut external_name:
                                                               *mut gss_name_t)
 -> OM_uint32 {
    let mut major_status: OM_uint32 = 0;
    let mut tmp: OM_uint32 = 0;
    let mut union_name: gss_union_name_t = 0 as *mut gss_name_struct;
    union_name =
        malloc(::std::mem::size_of::<gss_union_name_desc>() as libc::c_ulong)
            as gss_union_name_t;
    if union_name.is_null() {
        major_status =
            (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
        *minor_status = 12 as libc::c_int as OM_uint32;
        *minor_status = gssint_mecherrmap_map_errcode(*minor_status)
    } else {
        (*union_name).mech_type = 0 as gss_OID;
        (*union_name).mech_name = internal_name;
        (*union_name).name_type = 0 as gss_OID;
        (*union_name).external_name = 0 as gss_buffer_t;
        major_status =
            generic_gss_copy_oid(minor_status, &mut (*mech).mech_type,
                                 &mut (*union_name).mech_type);
        if major_status != 0 as libc::c_int as libc::c_uint {
            *minor_status = gssint_mecherrmap_map_errcode(*minor_status)
        } else {
            (*union_name).external_name =
                malloc(::std::mem::size_of::<gss_buffer_desc>() as
                           libc::c_ulong) as gss_buffer_t;
            if (*union_name).external_name.is_null() {
                major_status =
                    (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
            } else {
                (*(*union_name).external_name).length =
                    0 as libc::c_int as size_t;
                (*(*union_name).external_name).value = 0 as *mut libc::c_void;
                major_status =
                    (*mech).gss_display_name.expect("non-null function pointer")(minor_status,
                                                                                 internal_name,
                                                                                 (*union_name).external_name,
                                                                                 &mut (*union_name).name_type);
                if major_status != 0 as libc::c_int as libc::c_uint {
                    *minor_status =
                        gssint_mecherrmap_map(*minor_status,
                                              &mut (*mech).mech_type)
                } else {
                    (*union_name).loopback = union_name;
                    *external_name = union_name;
                    return 0 as libc::c_int as OM_uint32
                }
            }
        }
    }
    if !union_name.is_null() {
        if !(*union_name).external_name.is_null() {
            if !(*(*union_name).external_name).value.is_null() {
                free((*(*union_name).external_name).value);
            }
            free((*union_name).external_name as *mut libc::c_void);
        }
        if !(*union_name).name_type.is_null() {
            gss_release_oid(&mut tmp, &mut (*union_name).name_type);
        }
        if !(*union_name).mech_type.is_null() {
            gss_release_oid(&mut tmp, &mut (*union_name).mech_type);
        }
        free(union_name as *mut libc::c_void);
    }
    /*
     * do as the top comment says - since we are now owners of
     * internal_name, we must clean it up
     */
    if !internal_name.is_null() {
        gssint_release_internal_name(&mut tmp, &mut (*mech).mech_type,
                                     &mut internal_name);
    }
    return major_status;
}
/*
 * Glue routine for returning the mechanism-specific credential from a
 * external union credential.
 */
#[no_mangle]
#[c2rust::src_loc = "701:1"]
pub unsafe extern "C" fn gssint_get_mechanism_cred(mut union_cred:
                                                       gss_union_cred_t,
                                                   mut mech_type: gss_OID)
 -> gss_cred_id_t {
    let mut i: libc::c_int = 0;
    if union_cred.is_null() { return 0 as gss_cred_id_t }
    i = 0 as libc::c_int;
    while i < (*union_cred).count {
        if (*mech_type).length ==
               (*(*union_cred).mechs_array.offset(i as isize)).length &&
               memcmp((*mech_type).elements,
                      (*(*union_cred).mechs_array.offset(i as
                                                             isize)).elements,
                      (*mech_type).length as libc::c_ulong) ==
                   0 as libc::c_int {
            return *(*union_cred).cred_array.offset(i as isize)
        }
        i += 1
    }
    return 0 as gss_cred_id_t;
}
/*
 * Routine to create and copy the gss_buffer_desc structure.
 * Both space for the structure and the data is allocated.
 */
#[no_mangle]
#[c2rust::src_loc = "722:1"]
pub unsafe extern "C" fn gssint_create_copy_buffer(srcBuf: gss_buffer_t,
                                                   mut destBuf:
                                                       *mut gss_buffer_t,
                                                   mut addNullChar:
                                                       libc::c_int)
 -> OM_uint32 {
    let mut aBuf: gss_buffer_t = 0 as *mut gss_buffer_desc_struct;
    let mut len: libc::c_uint = 0;
    if destBuf.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    *destBuf = 0 as gss_buffer_t;
    aBuf =
        malloc(::std::mem::size_of::<gss_buffer_desc>() as libc::c_ulong) as
            gss_buffer_t;
    if aBuf.is_null() {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if addNullChar != 0 {
        len =
            (*srcBuf).length.wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_uint
    } else { len = (*srcBuf).length as libc::c_uint }
    (*aBuf).value = gssalloc_malloc(len as size_t);
    if (*aBuf).value.is_null() {
        free(aBuf as *mut libc::c_void);
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    memcpy((*aBuf).value, (*srcBuf).value, (*srcBuf).length);
    (*aBuf).length = (*srcBuf).length;
    *destBuf = aBuf;
    /* optionally add a NULL character */
    if addNullChar != 0 {
        *((*aBuf).value as *mut libc::c_char).offset((*aBuf).length as isize)
            = '\u{0}' as i32 as libc::c_char
    }
    return 0 as libc::c_int as OM_uint32;
}
/*
 * Rudimentary pointer validation macro to check whether the
 * "loopback" field of an opaque struct points back to itself.  This
 * field also catches some programming errors where an opaque pointer
 * is passed to a function expecting the address of the opaque
 * pointer.
 */
/* *******************************************************/
/* The Mechanism Dispatch Table -- a mechanism needs to */
/* define one of these and provide a function to return */
/* it to initialize the GSSAPI library		  */
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
/* kernel module name */
/* user library name */
/* mechanism string name */
/* optional mech parameters */
/* RTLD object handle for the mech */
/* mechanism oid */
/* mechanism initialization struct */
/* mechanism preference order */
/* free mech table */
/* interposer mechanism flag */
/* points to the interposer OID */
/* points to the interposer mech */
/* next element in the list */
/* *******************************************************/
/* Internal mechglue routines */
/* minor_status */
/* mech */
/* internal_name */
/* external_name */
/* union_cred */
/* mech_type */
/* src buffer */
/* destination buffer */
/* NULL terminate buffer ? */
/* ****** gssint_create_copy_buffer  ****** */
#[no_mangle]
#[c2rust::src_loc = "762:1"]
pub unsafe extern "C" fn gssint_create_union_context(mut minor:
                                                         *mut OM_uint32,
                                                     mut mech_oid:
                                                         gss_const_OID,
                                                     mut ctx_out:
                                                         *mut gss_union_ctx_id_t)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    let mut ctx: gss_union_ctx_id_t = 0 as *mut gss_union_ctx_id_struct;
    *ctx_out = 0 as gss_union_ctx_id_t;
    ctx =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<gss_union_ctx_id_struct>() as
                   libc::c_ulong) as gss_union_ctx_id_t;
    if ctx.is_null() {
        *minor = 12 as libc::c_int as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    status = generic_gss_copy_oid(minor, mech_oid, &mut (*ctx).mech_type);
    if status != 0 as libc::c_int as libc::c_uint {
        free(ctx as *mut libc::c_void);
        return status
    }
    (*ctx).loopback = ctx;
    (*ctx).internal_ctx_id = 0 as gss_ctx_id_t;
    *ctx_out = ctx;
    return 0 as libc::c_int as OM_uint32;
}
