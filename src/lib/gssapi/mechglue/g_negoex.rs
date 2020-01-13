use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:39"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:39"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:39"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/sys/types.h:39"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/mechglue/mglueP.h:39"]
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
    #[c2rust::src_loc = "60:1"]
    pub type gss_union_cred_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "36:1"]
    pub type gss_union_name_t = *mut gss_name_struct;
    use super::gssapi_h::{gss_OID, gss_buffer_t, gss_name_t, gss_cred_id_t,
                          gss_OID_desc, OM_uint32, gss_OID_set, gss_ctx_id_t,
                          gss_channel_bindings_t, gss_qop_t, gss_cred_usage_t,
                          gss_const_OID, gss_const_buffer_t,
                          gss_OID_desc_struct, gss_buffer_desc_struct};
    use super::gssapi_ext_h::{gss_buffer_set_t, gss_iov_buffer_desc,
                              gss_any_t, gss_const_key_value_set_t};
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "780:1"]
        pub fn gssint_delete_internal_sec_context(_: *mut OM_uint32,
                                                  _: gss_OID,
                                                  _: *mut gss_ctx_id_t,
                                                  _: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "779:1"]
        pub fn gssint_release_internal_name(_: *mut OM_uint32, _: gss_OID,
                                            _: *mut gss_name_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "803:1"]
        pub fn gssint_create_union_context(minor: *mut OM_uint32,
                                           _: gss_const_OID,
                                           _: *mut gss_union_ctx_id_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "773:1"]
        pub fn gssint_import_internal_name(_: *mut OM_uint32, _: gss_OID,
                                           _: gss_union_name_t,
                                           _: *mut gss_name_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "792:1"]
        pub fn gssint_get_mechanism_cred(_: gss_union_cred_t, _: gss_OID)
         -> gss_cred_id_t;
        #[no_mangle]
        #[c2rust::src_loc = "769:1"]
        pub fn gssint_get_mechanism(_: gss_const_OID) -> gss_mechanism;
        #[no_mangle]
        #[c2rust::src_loc = "766:1"]
        pub fn gssint_get_public_oid(internal_oid: gss_const_OID) -> gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "764:1"]
        pub fn gssint_select_mech_type(minor: *mut OM_uint32,
                                       in_oid: gss_const_OID,
                                       selected_oid: *mut gss_OID)
         -> OM_uint32;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:39"]
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
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:39"]
pub mod gssapi_ext_h {
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
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src = "/usr/include/assert.h:39"]
pub mod assert_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:39"]
pub mod gssapiP_generic_h {
    use super::gssapi_h::{OM_uint32, gss_OID_desc};
    extern "C" {
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
 * $Id$
 */
        /* * helper macros **/
        /* this code knows that an int on the wire is 32 bits.  The type of
   num should be at least this big, or the extra shifts may do weird
   things */
        /* * malloc wrappers; these may actually do something later */
        /* * helper functions **/
        /* hide names from applications, especially glib applications */
        /* flags for g_verify_token_header() */
        /* * declarations of internal name mechanism functions **/
        /* minor_status */
        /* buffer */
        /* minor_status */
        /* set */
        /* minor_status */
        /* set */
        /* minor_status */
        /* oid */
        /* new_oid */
        /* minor_status */
        /* oid_set */
        /* minor_status */
        /* member_oid */
        /* oid_set */
        /* minor_status */
        /* member */
        /* set */
        /* present */
        /* minor_status */
        /* oid */
        /* oid_str */
        /* minor_status */
        /* oid_str */
        /* oid */
        /* minor_status */
        /* prefix */
        /* prefix_len */
        /* suffix */
        /* oid */
        /* minor_status */
        /*prefix */
        /* prefix_len */
        /* oid */
        /* suffix */
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn gssint_mecherrmap_map(minor: OM_uint32,
                                     oid: *const gss_OID_desc) -> OM_uint32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src = "/usr/include/string.h:39"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
pub use self::types_h::{__uint32_t, __ssize_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::stddef_h::size_t;
pub use self::sys_types_h::ssize_t;
pub use self::mglueP_h::{gss_name_struct, gss_cred_id_struct, gss_mechanism,
                         gss_config, gss_union_ctx_id_t,
                         gss_union_ctx_id_struct, gss_union_cred_t,
                         gss_union_name_t, gssint_delete_internal_sec_context,
                         gssint_release_internal_name,
                         gssint_create_union_context,
                         gssint_import_internal_name,
                         gssint_get_mechanism_cred, gssint_get_mechanism,
                         gssint_get_public_oid, gssint_select_mech_type};
pub use self::gssapi_h::{gss_name_t, gss_OID, gss_OID_desc_struct, OM_uint32,
                         gss_uint32, gss_buffer_t, gss_buffer_desc_struct,
                         gss_cred_id_t, gss_ctx_id_t, gss_OID_desc,
                         gss_OID_set_desc_struct, gss_OID_set,
                         gss_buffer_desc, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_qop_t, gss_cred_usage_t,
                         gss_const_buffer_t, gss_const_OID,
                         gss_ctx_id_struct};
pub use self::gssapi_ext_h::{gss_buffer_set_desc_struct, gss_buffer_set_t,
                             gss_iov_buffer_desc_struct, gss_iov_buffer_desc,
                             gss_any_t, gss_key_value_element_struct,
                             gss_key_value_element_desc,
                             gss_key_value_set_struct, gss_key_value_set_desc,
                             gss_const_key_value_set_t, gss_any};
use self::assert_h::__assert_fail;
use self::gssapiP_generic_h::gssint_mecherrmap_map;
use self::string_h::{memcmp, memset};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2011 by the Massachusetts Institute of Technology.
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
/*
 * This file contains dispatch functions for the three GSSAPI extensions
 * described in draft-zhu-negoex-04, renamed to use the gssspi_ prefix.  Since
 * the only caller of these functions is SPNEGO, argument validation is
 * omitted.
 */
#[no_mangle]
#[c2rust::src_loc = "41:1"]
pub unsafe extern "C" fn gssspi_query_meta_data(mut minor_status:
                                                    *mut OM_uint32,
                                                mut mech_oid: gss_const_OID,
                                                mut cred_handle:
                                                    gss_cred_id_t,
                                                mut context_handle:
                                                    *mut gss_ctx_id_t,
                                                targ_name: gss_name_t,
                                                mut req_flags: OM_uint32,
                                                mut meta_data: gss_buffer_t)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut status: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut ctx: gss_union_ctx_id_t = *context_handle as gss_union_ctx_id_t;
    let mut cred: gss_union_cred_t = cred_handle as gss_union_cred_t;
    let mut union_name: gss_union_name_t = targ_name as gss_union_name_t;
    let mut mech: gss_mechanism = 0 as *mut gss_config;
    let mut selected_mech: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut public_mech: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut internal_cred: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut internal_name: gss_name_t = 0 as gss_name_t;
    let mut imported_name: gss_name_t = 0 as gss_name_t;
    let mut new_ctx: gss_ctx_id_t = 0 as gss_ctx_id_t;
    let mut internal_ctx: *mut gss_ctx_id_t = 0 as *mut gss_ctx_id_t;
    *minor_status = 0 as libc::c_int as OM_uint32;
    (*meta_data).length = 0 as libc::c_int as size_t;
    (*meta_data).value = 0 as *mut libc::c_void;
    status =
        gssint_select_mech_type(minor_status, mech_oid, &mut selected_mech);
    if status != 0 as libc::c_int as libc::c_uint { return status }
    public_mech = gssint_get_public_oid(selected_mech as gss_const_OID);
    mech = gssint_get_mechanism(selected_mech as gss_const_OID);
    if mech.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if (*mech).gssspi_query_meta_data.is_none() {
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if !cred.is_null() {
        internal_cred = gssint_get_mechanism_cred(cred, selected_mech);
        if internal_cred.is_null() {
            return (7 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    }
    if !union_name.is_null() {
        if !(*union_name).mech_type.is_null() &&
               ((*(*union_name).mech_type).length == (*selected_mech).length
                    &&
                    memcmp((*(*union_name).mech_type).elements,
                           (*selected_mech).elements,
                           (*(*union_name).mech_type).length as libc::c_ulong)
                        == 0 as libc::c_int) {
            internal_name = (*union_name).mech_name;
            current_block = 15125582407903384992;
        } else {
            status =
                gssint_import_internal_name(minor_status, selected_mech,
                                            union_name, &mut imported_name);
            if status != 0 as libc::c_int as libc::c_uint {
                current_block = 1135352847978496863;
            } else {
                internal_name = imported_name;
                current_block = 15125582407903384992;
            }
        }
    } else { current_block = 15125582407903384992; }
    match current_block {
        15125582407903384992 => {
            internal_ctx =
                if !ctx.is_null() {
                    &mut (*ctx).internal_ctx_id
                } else { &mut new_ctx };
            status =
                (*mech).gssspi_query_meta_data.expect("non-null function pointer")(minor_status,
                                                                                   public_mech
                                                                                       as
                                                                                       gss_const_OID,
                                                                                   internal_cred,
                                                                                   internal_ctx,
                                                                                   internal_name,
                                                                                   req_flags,
                                                                                   meta_data);
            if status != 0 as libc::c_int as libc::c_uint {
                *minor_status =
                    gssint_mecherrmap_map(*minor_status,
                                          &mut (*mech).mech_type)
            } else if !new_ctx.is_null() {
                if ctx.is_null() {
                } else {
                    __assert_fail(b"ctx == NULL\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"g_negoex.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  102 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 135],
                                                            &[libc::c_char; 135]>(b"OM_uint32 gssspi_query_meta_data(OM_uint32 *, gss_const_OID, gss_cred_id_t, gss_ctx_id_t *, const gss_name_t, OM_uint32, gss_buffer_t)\x00")).as_ptr());
                }
                status =
                    gssint_create_union_context(minor_status,
                                                selected_mech as
                                                    gss_const_OID, &mut ctx);
                if !(status != 0 as libc::c_int as libc::c_uint) {
                    (*ctx).internal_ctx_id = new_ctx;
                    new_ctx = 0 as gss_ctx_id_t;
                    *context_handle = ctx as gss_ctx_id_t
                }
            }
        }
        _ => { }
    }
    if !imported_name.is_null() {
        gssint_release_internal_name(&mut minor, selected_mech,
                                     &mut imported_name);
    }
    if !new_ctx.is_null() {
        gssint_delete_internal_sec_context(&mut minor, &mut (*mech).mech_type,
                                           &mut new_ctx, 0 as gss_buffer_t);
    }
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "125:1"]
pub unsafe extern "C" fn gssspi_exchange_meta_data(mut minor_status:
                                                       *mut OM_uint32,
                                                   mut mech_oid:
                                                       gss_const_OID,
                                                   mut cred_handle:
                                                       gss_cred_id_t,
                                                   mut context_handle:
                                                       *mut gss_ctx_id_t,
                                                   targ_name: gss_name_t,
                                                   mut req_flags: OM_uint32,
                                                   mut meta_data:
                                                       gss_const_buffer_t)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut ctx: gss_union_ctx_id_t = *context_handle as gss_union_ctx_id_t;
    let mut cred: gss_union_cred_t = cred_handle as gss_union_cred_t;
    let mut union_name: gss_union_name_t = targ_name as gss_union_name_t;
    let mut mech: gss_mechanism = 0 as *mut gss_config;
    let mut selected_mech: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut public_mech: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut internal_cred: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut internal_name: gss_name_t = 0 as gss_name_t;
    let mut imported_name: gss_name_t = 0 as gss_name_t;
    let mut new_ctx: gss_ctx_id_t = 0 as gss_ctx_id_t;
    let mut internal_ctx: *mut gss_ctx_id_t = 0 as *mut gss_ctx_id_t;
    *minor_status = 0 as libc::c_int as OM_uint32;
    status =
        gssint_select_mech_type(minor_status, mech_oid, &mut selected_mech);
    if status != 0 as libc::c_int as libc::c_uint { return status }
    public_mech = gssint_get_public_oid(selected_mech as gss_const_OID);
    mech = gssint_get_mechanism(selected_mech as gss_const_OID);
    if mech.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if (*mech).gssspi_exchange_meta_data.is_none() {
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if !cred.is_null() {
        internal_cred = gssint_get_mechanism_cred(cred, selected_mech);
        if internal_cred.is_null() {
            return (7 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    }
    if !union_name.is_null() {
        if !(*union_name).mech_type.is_null() &&
               ((*(*union_name).mech_type).length == (*selected_mech).length
                    &&
                    memcmp((*(*union_name).mech_type).elements,
                           (*selected_mech).elements,
                           (*(*union_name).mech_type).length as libc::c_ulong)
                        == 0 as libc::c_int) {
            internal_name = (*union_name).mech_name
        } else {
            status =
                gssint_import_internal_name(minor_status, selected_mech,
                                            union_name, &mut imported_name);
            if status &
                   ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
                        |
                        (0o377 as libc::c_ulong as OM_uint32) <<
                            16 as libc::c_int) != 0 {
                return status
            }
            internal_name = imported_name
        }
    }
    internal_ctx =
        if !ctx.is_null() {
            &mut (*ctx).internal_ctx_id
        } else { &mut new_ctx };
    status =
        (*mech).gssspi_exchange_meta_data.expect("non-null function pointer")(minor_status,
                                                                              public_mech
                                                                                  as
                                                                                  gss_const_OID,
                                                                              internal_cred,
                                                                              internal_ctx,
                                                                              internal_name,
                                                                              req_flags,
                                                                              meta_data);
    if status != 0 as libc::c_int as libc::c_uint {
        *minor_status =
            gssint_mecherrmap_map(*minor_status, &mut (*mech).mech_type)
    } else if !new_ctx.is_null() {
        if ctx.is_null() {
        } else {
            __assert_fail(b"ctx == NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"g_negoex.c\x00" as *const u8 as
                              *const libc::c_char,
                          186 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 144],
                                                    &[libc::c_char; 144]>(b"OM_uint32 gssspi_exchange_meta_data(OM_uint32 *, gss_const_OID, gss_cred_id_t, gss_ctx_id_t *, const gss_name_t, OM_uint32, gss_const_buffer_t)\x00")).as_ptr());
        }
        status =
            gssint_create_union_context(minor_status,
                                        selected_mech as gss_const_OID,
                                        &mut ctx);
        if !(status != 0 as libc::c_int as libc::c_uint) {
            (*ctx).internal_ctx_id = new_ctx;
            new_ctx = 0 as gss_ctx_id_t;
            *context_handle = ctx as gss_ctx_id_t
        }
    }
    if !imported_name.is_null() {
        gssint_release_internal_name(&mut minor, selected_mech,
                                     &mut imported_name);
    }
    if !new_ctx.is_null() {
        gssint_delete_internal_sec_context(&mut minor, &mut (*mech).mech_type,
                                           &mut new_ctx, 0 as gss_buffer_t);
    }
    return status;
}
/* If the mech created a context, wrap it in a union context. */
/* If the mech created a context, wrap it in a union context. */
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
/* __cplusplus */
/*
 * Solaris extensions
 */
/* *
 * Provides a platform-specific name for a GSSAPI name as interpreted by a
 * given mechanism.
 *
 * @param [out] minor      Minor status code
 * @param [in] name        The gss name resulting from accept_sec_context
 * @param [in] mech_type   The mechanism that will be asked to map @a name to a
 *                         local name
 * @param [out] localname  Caller-allocated buffer to be filled in with the
 *                         local name on success
 */
/* *
 * Determine whether a mechanism name is authorized to act as a username.
 *
 * @param [in] name      Mechanism name
 * @param [in] username  System username
 *
 * This is a simple wrapper around gss_authorize_localname().  It only supports
 * system usernames as local names, and cannot distinguish between lack of
 * authorization and other errors.
 *
 * @retval 1 @a name is authorized to act as @a username
 * @retval 0 @a name is not authorized or an error occurred
 */
/* *
 *  Determine whether a mechanism name is authorized to act as a local name.
 *
 * @param [out] minor  Minor status code
 * @param [in] name    Mechanism name
 * @param [in] user    Local name
 *
 * @a name is a mechanism name, typically the result of a completed
 * gss_accept_sec_context().  @a user is an internal name representing a local
 * name, such as a name imported by gss_import_name() with an @a
 * input_name_type of @c GSS_C_NT_USER_NAME.
 *
 * @return Return GSS_S_COMPLETE if @a name is authorized to act as @a user,
 * GSS_S_UNAUTHORIZED if not, or an appropriate GSS error code if an error
 * occurred.
 *
 * @sa gss_userok
 */
/* minor_status */
/* desired_name */
/* password */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* input_cred_handle */
/* desired_name */
/* desired_mech */
/* password */
/* cred_usage */
/* initiator_time_req */
/* acceptor_time_req */
/* output_cred_handle */
/* actual_mechs */
/* initiator_time_rec */
/* acceptor_time_rec */
/*
 * GGF extensions
 */
/*minor_status*/
/*buffer_set*/
/*minor_status*/
/*member_buffer*/
/*buffer_set*/
/*minor_status*/
/*buffer_set*/
/*minor_status*/
/*context_handle*/
/*desired_object*/
/*data_set*/
/*minor_status*/
/*cred_handle*/
/*desired_object*/
/*data_set*/
/*minor_status*/
/*cred_handle*/
/*desired_object*/
/*value*/
/*
 * Export import cred extensions from GGF, but using Heimdal's signatures
 */
/* minor_status */
/* cred_handle */
/* token */
/* minor_status */
/* token */
/* cred_handle */
/*
 * Heimdal extension
 */
/*minor_status*/
/*cred*/
/*desired_object*/
/*value*/
/*
 * Call the given method on the given mechanism
 */
/*minor_status*/
/*desired_mech*/
/*desired_object*/
/*value*/
/*
 * AEAD extensions
 */
/*minor_status*/
/*context_handle*/
/*conf_req_flag*/
/*qop_req*/
/*input_assoc_buffer*/
/*input_payload_buffer*/
/*conf_state*/
/*output_message_buffer*/
/*minor_status*/
/*context_handle*/
/*input_message_buffer*/
/*input_assoc_buffer*/
/*output_payload_buffer*/
/*conf_state*/
/*qop_state*/
/*
 * SSPI extensions
 */
/*
 * Returns a buffer set with the first member containing the
 * session key for SSPI compatibility. The optional second
 * member contains an OID identifying the session key type.
 */
/* Packet data */
/* Mechanism header */
/* Mechanism specific parameters */
/* Mechanism trailer */
/* Padding */
/* Complete wrap token */
/* Sign only packet data */
/* MIC token destination */
/* indicates GSS should allocate */
/* indicates caller should free */
/*
 * Sign and optionally encrypt a sequence of buffers. The buffers
 * shall be ordered HEADER | DATA | PADDING | TRAILER. Suitable
 * space for the header, padding and trailer should be provided
 * by calling gss_wrap_iov_length(), or the ALLOCATE flag should
 * be set on those buffers.
 *
 * Encryption is in-place. SIGN_ONLY buffers are untouched. Only
 * a single PADDING buffer should be provided. The order of the
 * buffers in memory does not matter. Buffers in the IOV should
 * be arranged in the order above, and in the case of multiple
 * DATA buffers the sender and receiver should agree on the
 * order.
 *
 * With GSS_C_DCE_STYLE it is acceptable to not provide PADDING
 * and TRAILER, but the caller must guarantee the plaintext data
 * being encrypted is correctly padded, otherwise an error will
 * be returned.
 *
 * While applications that have knowledge of the underlying
 * cryptosystem may request a specific configuration of data
 * buffers, the only generally supported configurations are:
 *
 *  HEADER | DATA | PADDING | TRAILER
 *
 * which will emit GSS_Wrap() compatible tokens, and:
 *
 *  HEADER | SIGN_ONLY | DATA | PADDING | TRAILER
 *
 * for AEAD.
 *
 * The typical (special cased) usage for DCE is as follows:
 *
 *  SIGN_ONLY_1 | DATA | SIGN_ONLY_2 | HEADER
 */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/*
 * Verify and optionally decrypt a sequence of buffers. To process
 * a GSS-API message without separate buffer, pass STREAM | DATA.
 * Upon return DATA will contain the decrypted or integrity
 * protected message. Only a single DATA buffer may be provided
 * with this usage. DATA by default will point into STREAM, but if
 * the ALLOCATE flag is set a copy will be returned.
 *
 * Otherwise, decryption is in-place. SIGN_ONLY buffers are
 * untouched.
 */
/* minor_status */
/* context_handle */
/* conf_state */
/* qop_state */
/* iov */
/* iov_count */
/*
 * Query HEADER, PADDING and TRAILER buffer lengths. DATA buffers
 * should be provided so the correct padding length can be determined.
 */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/*
 * Produce a GSSAPI MIC token for a sequence of buffers.  All SIGN_ONLY and
 * DATA buffers will be signed, in the order they appear.  One MIC_TOKEN buffer
 * must be included for the result.  Suitable space should be provided for the
 * MIC_TOKEN buffer by calling gss_get_mic_iov_length, or the ALLOCATE flag
 * should be set on that buffer.  If the ALLOCATE flag is used, use
 * gss_release_iov_buffer to free the allocated buffer within the iov list when
 * it is no longer needed.
 */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/*
 * Query the MIC_TOKEN buffer length within the iov list.
 */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/*
 * Verify the MIC_TOKEN buffer within the iov list against the SIGN_ONLY and
 * DATA buffers in the order they appear.  Return values are the same as for
 * gss_verify_mic.
 */
/* minor_status */
/* context_handle */
/* qop_state */
/* iov */
/* iov_count */
/*
 * Release buffers that have the ALLOCATED flag set.
 */
/* minor_status */
/* iov */
/* iov_count */
/*
 * Protocol transition
 */
/* minor_status */
/* impersonator_cred_handle */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
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
/*
 * Naming extensions
 */
/* minor_status */
/* name */
/* display_as_name_type */
/* display_name */
/* minor_status */
/* name */
/* name_is_MN */
/* MN_mech */
/* attrs */
/* minor_status */
/* name */
/* attr */
/* authenticated */
/* complete */
/* value */
/* display_value */
/* more */
/* minor_status */
/* name */
/* complete */
/* attr */
/* value */
/* minor_status */
/* name */
/* attr */
/* minor_status */
/* name */
/* exp_composite_name */
/* minor_status */
/* name */
/* authenticated */
/* type_id */
/* output */
/* minor_status */
/* name */
/* type_id */
/* input */
/* draft-josefsson-gss-capsulate */
/* input_token */
/* token_oid */
/* output_token */
/* input_token */
/* token_oid */
/* output_token */
/* first_oid */
/* second_oid */
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
/* minor_status */
/* input_cred_handle */
/* desired_name */
/* desired_mech */
/* cred_usage */
/* initiator_time_req */
/* acceptor_time_req */
/* cred_store */
/* output_cred_handle */
/* actual_mechs */
/* initiator_time_rec */
/* acceptor_time_rec */
/* minor_status */
/* input_cred_handle */
/* input_usage */
/* desired_mech */
/* overwrite_cred */
/* default_cred */
/* cred_store */
/* elements_stored */
/* cred_usage_stored */
/*
 * A mech can make itself negotiable via NegoEx (draft-zhu-negoex) by
 * implementing the following three SPIs, and also implementing
 * gss_inquire_sec_context_by_oid() and answering the GSS_C_INQ_NEGOEX_KEY and
 * GSS_C_INQ_NEGOEX_VERIFY_KEY OIDs.  The answer must be in two buffers: the
 * first contains the key contents, and the second contains the key enctype as
 * a four-byte little-endian integer.
 *
 * By default, NegoEx mechanisms will not be directly negotiated via SPNEGO.
 * If direct SPNEGO negotiation is required for interoperability, implement
 * gss_inquire_attrs_for_mech() and assert the GSS_C_MA_NEGOEX_AND_SPNEGO
 * attribute (along with any applicable RFC 5587 attributes).
 */
#[no_mangle]
#[c2rust::src_loc = "209:1"]
pub unsafe extern "C" fn gssspi_query_mechanism_info(mut minor_status:
                                                         *mut OM_uint32,
                                                     mut mech_oid:
                                                         gss_const_OID,
                                                     mut auth_scheme:
                                                         *mut libc::c_uchar)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    let mut selected_mech: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut public_mech: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut mech: gss_mechanism = 0 as *mut gss_config;
    *minor_status = 0 as libc::c_int as OM_uint32;
    memset(auth_scheme as *mut libc::c_void, 0 as libc::c_int,
           16 as libc::c_int as libc::c_ulong);
    status =
        gssint_select_mech_type(minor_status, mech_oid, &mut selected_mech);
    if status != 0 as libc::c_int as libc::c_uint { return status }
    public_mech = gssint_get_public_oid(selected_mech as gss_const_OID);
    mech = gssint_get_mechanism(selected_mech as gss_const_OID);
    if mech.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if (*mech).gssspi_query_mechanism_info.is_none() {
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    status =
        (*mech).gssspi_query_mechanism_info.expect("non-null function pointer")(minor_status,
                                                                                public_mech
                                                                                    as
                                                                                    gss_const_OID,
                                                                                auth_scheme);
    if status &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        *minor_status =
            gssint_mecherrmap_map(*minor_status, &mut (*mech).mech_type)
    }
    return status;
}
