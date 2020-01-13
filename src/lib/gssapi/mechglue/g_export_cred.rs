use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/sys/types.h:33"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/mechglue/mglueP.h:33"]
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
    #[c2rust::src_loc = "60:1"]
    pub type gss_union_cred_t = *mut gss_cred_id_struct;
    use super::gssapi_h::{gss_OID, gss_buffer_t, gss_name_t, gss_cred_id_t,
                          gss_OID_desc, OM_uint32, gss_OID_set, gss_ctx_id_t,
                          gss_channel_bindings_t, gss_qop_t, gss_cred_usage_t,
                          gss_const_OID, gss_const_buffer_t};
    use super::gssapi_ext_h::{gss_buffer_set_t, gss_iov_buffer_desc,
                              gss_any_t, gss_const_key_value_set_t};
    use super::sys_types_h::ssize_t;
    extern "C" {
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
        #[no_mangle]
        #[c2rust::src_loc = "769:1"]
        pub fn gssint_get_mechanism(_: gss_const_OID) -> gss_mechanism;
        #[no_mangle]
        #[c2rust::src_loc = "766:1"]
        pub fn gssint_get_public_oid(internal_oid: gss_const_OID) -> gss_OID;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:33"]
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
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:33"]
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
    #[c2rust::src_loc = "248:1"]
    pub type gss_iov_buffer_desc = gss_iov_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "248:16"]
    pub struct gss_iov_buffer_desc_struct {
        pub type_0: OM_uint32,
        pub buffer: gss_buffer_desc,
    }
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
    #[c2rust::src_loc = "541:1"]
    pub type gss_const_key_value_set_t = *const gss_key_value_set_desc;
    #[c2rust::src_loc = "540:1"]
    pub type gss_key_value_set_desc = gss_key_value_set_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "536:8"]
    pub struct gss_key_value_set_struct {
        pub count: OM_uint32,
        pub elements: *mut gss_key_value_element_desc,
    }
    #[c2rust::src_loc = "534:1"]
    pub type gss_key_value_element_desc = gss_key_value_element_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "530:8"]
    pub struct gss_key_value_element_struct {
        pub key: *const libc::c_char,
        pub value: *const libc::c_char,
    }
    #[c2rust::src_loc = "488:1"]
    pub type gss_any_t = *mut gss_any;
    use super::stddef_h::size_t;
    use super::gssapi_h::{gss_buffer_desc, OM_uint32};
    extern "C" {
        #[c2rust::src_loc = "488:16"]
        pub type gss_any;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:33"]
pub mod k5_buf_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-buf.h - k5buf interface declarations */
/*
 * Copyright 2008 Massachusetts Institute of Technology.
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
    /*
 * The k5buf module is intended to allow multi-step string construction in a
 * fixed or dynamic buffer without the need to check for a failure at each step
 * (and without aborting on malloc failure).  If an allocation failure occurs
 * or the fixed buffer runs out of room, the buffer will be set to an error
 * state which can be detected with k5_buf_status.  Data in a buffer is
 * terminated with a zero byte so that it can be used as a C string.
 *
 * k5buf structures are usually stack-allocated.  Do not put k5buf structure
 * pointers into public APIs.  It is okay to reference the data and len fields
 * of a buffer (they will be NULL/0 if the buffer is in an error state), but do
 * not change them.
 */
    /* Buffer type values */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct k5buf {
        pub buftype: k5buftype,
        pub data: *mut libc::c_void,
        pub space: size_t,
        pub len: size_t,
    }
    #[c2rust::src_loc = "48:1"]
    pub type k5buftype = libc::c_uint;
    #[c2rust::src_loc = "48:59"]
    pub const K5BUF_DYNAMIC_ZAP: k5buftype = 3;
    #[c2rust::src_loc = "48:44"]
    pub const K5BUF_DYNAMIC: k5buftype = 2;
    #[c2rust::src_loc = "48:31"]
    pub const K5BUF_FIXED: k5buftype = 1;
    #[c2rust::src_loc = "48:18"]
    pub const K5BUF_ERROR: k5buftype = 0;
    #[inline]
    #[c2rust::src_loc = "129:1"]
    pub unsafe extern "C" fn k5_buf_add_uint32_be(mut buf: *mut k5buf,
                                                  mut val: uint32_t) {
        let mut p: *mut libc::c_void =
            k5_buf_get_space(buf, 4 as libc::c_int as size_t);
        if !p.is_null() { store_32_be(val, p); };
    }
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint32_t;
    use super::k5_platform_h::store_32_be;
    extern "C" {
        /* Initialize a k5buf using a fixed-sized, existing buffer.  SPACE must be
 * more than zero, or an assertion failure will result. */
        /* Initialize a k5buf using an internally allocated dynamic buffer. */
        /* Initialize a k5buf using an internally allocated dynamic buffer, zeroing
 * memory when reallocating or freeing. */
        /* Add a C string to BUF. */
        /* Add a counted series of bytes to BUF. */
        /* Add sprintf-style formatted data to BUF. */
        /* Add sprintf-style formatted data to BUF, with a va_list.  The value of ap is
 * undefined after the call. */
        /* Extend the length of buf by len and return a pointer to the reserved space,
 * to be filled in by the caller.  Return NULL on error. */
        /* Truncate BUF.  LEN must be between 0 and the existing buffer
 * length, or an assertion failure will result. */
        /* Return ENOMEM if buf is in an error state, 0 otherwise. */
        /*
 * Free the storage used in the dynamic buffer BUF.  The caller may choose to
 * take responsibility for freeing the data pointer instead of using this
 * function.  If BUF is a fixed buffer, an assertion failure will result.
 * Freeing a buffer in the error state, a buffer initialized with EMPTY_K5BUF,
 * or a zeroed k5buf structure is a no-op.
 */
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn k5_buf_free(buf: *mut k5buf);
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn k5_buf_status(buf: *mut k5buf) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn k5_buf_add_len(buf: *mut k5buf, data: *const libc::c_void,
                              len: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "93:1"]
        pub fn k5_buf_get_space(buf: *mut k5buf, len: size_t)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn k5_buf_init_dynamic(buf: *mut k5buf);
    }
    /* K5_BUF_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:33"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "574:5"]
    pub struct C2RustUnnamed {
        pub i: uint32_t,
    }
    /* -*- mode: c; indent-tabs-mode: nil -*- */
/* include/k5-platform.h */
/*
 * Copyright 2003, 2004, 2005, 2007, 2008, 2009 Massachusetts Institute of Technology.
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
    /*
 * Some platform-dependent definitions to sync up the C support level.
 * Some to a C99-ish level, some related utility code.
 *
 * Currently:
 * + [u]int{8,16,32}_t types
 * + 64-bit types and load/store code
 * + SIZE_MAX
 * + shared library init/fini hooks
 * + consistent getpwnam/getpwuid interfaces
 * + va_copy fudged if not provided
 * + strlcpy/strlcat
 * + fnmatch
 * + [v]asprintf
 * + strerror_r
 * + mkstemp
 * + zap (support function and macro)
 * + constant time memory comparison
 * + path manipulation
 * + _, N_, dgettext, bindtextdomain (for localization)
 * + getopt_long
 * + secure_getenv
 * + fetching filenames from a directory
 */
    /* This attribute prevents unused function warnings in gcc and clang. */
    /* Initialization and finalization function support for libraries.

   At top level, before the functions are defined or even declared:
   MAKE_INIT_FUNCTION(init_fn);
   MAKE_FINI_FUNCTION(fini_fn);
   Then:
   int init_fn(void) { ... }
   void fini_fn(void) { if (INITIALIZER_RAN(init_fn)) ... }
   In code, in the same file:
   err = CALL_INIT_FUNCTION(init_fn);

   To trigger or verify the initializer invocation from another file,
   a helper function must be created.

   This model handles both the load-time execution (Windows) and
   delayed execution (pthread_once) approaches, and should be able to
   guarantee in both cases that the init function is run once, in one
   thread, before other stuff in the library is done; furthermore, the
   finalization code should only run if the initialization code did.
   (Maybe I could've made the "if INITIALIZER_RAN" test implicit, via
   another function hidden in macros, but this is hairy enough
   already.)

   The init_fn and fini_fn names should be chosen such that any
   exported names staring with those names, and optionally followed by
   additional characters, fits in with any namespace constraints on
   the library in question.


   There's also PROGRAM_EXITING() currently always defined as zero.
   If there's some trivial way to find out if the fini function is
   being called because the program that the library is linked into is
   exiting, we can just skip all the work because the resources are
   about to be freed up anyways.  Generally this is likely to be the
   same as distinguishing whether the library was loaded dynamically
   while the program was running, or loaded as part of program
   startup.  On most platforms, I don't think we can distinguish these
   cases easily, and it's probably not worth expending any significant
   effort.  (Note in particular that atexit() won't do, because if the
   library is explicitly loaded and unloaded, it would have to be able
   to deregister the atexit callback function.  Also, the system limit
   on atexit callbacks may be small.)


   Implementation outline:

   Windows: MAKE_FINI_FUNCTION creates a symbol with a magic name that
   is sought at library build time, and code is added to invoke the
   function when the library is unloaded.  MAKE_INIT_FUNCTION does
   likewise, but the function is invoked when the library is loaded,
   and an extra variable is declared to hold an error code and a "yes
   the initializer ran" flag.  CALL_INIT_FUNCTION blows up if the flag
   isn't set, otherwise returns the error code.

   UNIX: MAKE_INIT_FUNCTION creates and initializes a variable with a
   name derived from the function name, containing a k5_once_t
   (pthread_once_t or int), an error code, and a pointer to the
   function.  The function itself is declared static, but the
   associated variable has external linkage.  CALL_INIT_FUNCTION
   ensures thath the function is called exactly once (pthread_once or
   just check the flag) and returns the stored error code (or the
   pthread_once error).

   (That's the basic idea.  With some debugging assert() calls and
   such, it's a bit more complicated.  And we also need to handle
   doing the pthread test at run time on systems where that works, so
   we use the k5_once_t stuff instead.)

   UNIX, with compiler support: MAKE_FINI_FUNCTION declares the
   function as a destructor, and the run time linker support or
   whatever will cause it to be invoked when the library is unloaded,
   the program ends, etc.

   UNIX, with linker support: MAKE_FINI_FUNCTION creates a symbol with
   a magic name that is sought at library build time, and linker
   options are used to mark it as a finalization function for the
   library.  The symbol must be exported.

   UNIX, no library finalization support: The finalization function
   never runs, and we leak memory.  Tough.

   DELAY_INITIALIZER will be defined by the configure script if we
   want to use k5_once instead of load-time initialization.  That'll
   be the preferred method on most systems except Windows, where we
   have to initialize some mutexes.




   For maximum flexibility in defining the macros, the function name
   parameter should be a simple name, not even a macro defined as
   another name.  The function should have a unique name, and should
   conform to whatever namespace is used by the library in question.
   (We do have export lists, but (1) they're not used for all
   platforms, and (2) they're not used for static libraries.)

   If the macro expansion needs the function to have been declared, it
   must include a declaration.  If it is not necessary for the symbol
   name to be exported from the object file, the macro should declare
   it as "static".  Hence the signature must exactly match "void
   foo(void)".  (ANSI C allows a static declaration followed by a
   non-static one; the result is internal linkage.)  The macro
   expansion has to come before the function, because gcc apparently
   won't act on "__attribute__((constructor))" if it comes after the
   function definition.

   This is going to be compiler- and environment-specific, and may
   require some support at library build time, and/or "asm"
   statements.  But through macro expansion and auxiliary functions,
   we should be able to handle most things except #pragma.

   It's okay for this code to require that the library be built
   with the same compiler and compiler options throughout, but
   we shouldn't require that the library and application use the
   same compiler.

   For static libraries, we don't really care about cleanup too much,
   since it's all memory handling and mutex allocation which will all
   be cleaned up when the program exits.  Thus, it's okay if gcc-built
   static libraries don't play nicely with cc-built executables when
   it comes to static constructors, just as long as it doesn't cause
   linking to fail.

   For dynamic libraries on UNIX, we'll use pthread_once-type support
   to do delayed initialization, so if finalization can't be made to
   work, we'll only have memory leaks in a load/use/unload cycle.  If
   anyone (like, say, the OS vendor) complains about this, they can
   tell us how to get a shared library finalization function invoked
   automatically.

   Currently there's --disable-delayed-initialization for preventing
   the initialization from being delayed on UNIX, but that's mainly
   just for testing the linker options for initialization, and will
   probably be removed at some point.  */
    /* Helper macros.  */
    /* XXX Should test USE_LINKER_INIT_OPTION early, and if it's set,
   always provide a function by the expected name, even if we're
   delaying initialization.  */
    /* Run the initialization code during program execution, at the latest
   possible moment.  This means multiple threads may be active.  */
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
    #[c2rust::src_loc = "567:1"]
    pub unsafe extern "C" fn store_32_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = __bswap_32(val);
    }
    use super::stdint_uintn_h::uint32_t;
    use super::byteswap_h::__bswap_32;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/bits/byteswap.h:33"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
        return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
                   (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
                   (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
                   (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
    }
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:33"]
pub mod gssapiP_generic_h {
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
    /*
 * Transfer contents of a k5buf to a gss_buffer and invalidate the source
 * On unix, this is a simple pointer copy
 * On windows, memory is reallocated and copied.
 */
    #[inline]
    #[c2rust::src_loc = "270:1"]
    pub unsafe extern "C" fn k5buf_to_gss(mut minor: *mut OM_uint32,
                                          mut input_k5buf: *mut k5buf,
                                          mut output_buffer: gss_buffer_t)
     -> OM_uint32 {
        let mut status: OM_uint32 = 0 as libc::c_int as OM_uint32;
        if k5_buf_status(input_k5buf) != 0 as libc::c_int {
            *minor = 12 as libc::c_int as OM_uint32;
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        (*output_buffer).length = (*input_k5buf).len;
        (*output_buffer).value = (*input_k5buf).data;
        memset(input_k5buf as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<k5buf>() as libc::c_ulong);
        return status;
    }
    use super::gssapi_h::{OM_uint32, gss_buffer_desc_struct, gss_buffer_t,
                          gss_OID_desc};
    use super::k5_buf_h::{k5buf, k5_buf_status};
    use super::stddef_h::size_t;
    use super::string_h::memset;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn gssint_mecherrmap_map(minor: OM_uint32,
                                     oid: *const gss_OID_desc) -> OM_uint32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
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
                         gss_config, gss_union_cred_t, gssint_get_mechanism,
                         gssint_get_public_oid};
pub use self::gssapi_h::{gss_name_t, gss_OID, gss_OID_desc_struct, OM_uint32,
                         gss_uint32, gss_buffer_t, gss_buffer_desc_struct,
                         gss_cred_id_t, gss_ctx_id_t, gss_OID_desc,
                         gss_OID_set_desc_struct, gss_OID_set,
                         gss_buffer_desc, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_qop_t, gss_cred_usage_t,
                         gss_const_buffer_t, gss_const_OID, gss_ctx_id_struct,
                         gss_release_buffer};
pub use self::gssapi_ext_h::{gss_buffer_set_desc_struct, gss_buffer_set_t,
                             gss_iov_buffer_desc, gss_iov_buffer_desc_struct,
                             gss_const_key_value_set_t,
                             gss_key_value_set_desc, gss_key_value_set_struct,
                             gss_key_value_element_desc,
                             gss_key_value_element_struct, gss_any_t,
                             gss_any};
pub use self::k5_buf_h::{k5buf, k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5_buf_add_uint32_be,
                         k5_buf_free, k5_buf_status, k5_buf_add_len,
                         k5_buf_get_space, k5_buf_init_dynamic};
pub use self::k5_platform_h::{C2RustUnnamed, store_32_be};
pub use self::byteswap_h::__bswap_32;
pub use self::gssapiP_generic_h::{k5buf_to_gss, gssint_mecherrmap_map};
use self::string_h::memset;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/mechglue/g_export_cred.c - gss_export_cred definition */
/*
 * Copyright (C) 2012 by the Massachusetts Institute of Technology.
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
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn val_exp_cred_args(mut minor_status: *mut OM_uint32,
                                       mut cred_handle: gss_cred_id_t,
                                       mut token: gss_buffer_t) -> OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0 as libc::c_int as OM_uint32
    }
    if !token.is_null() {
        (*token).length = 0 as libc::c_int as size_t;
        (*token).value = 0 as *mut libc::c_void
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    if cred_handle.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                   (7 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if token.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    return 0 as libc::c_int as OM_uint32;
}
/*
 * Export import cred extensions from GGF, but using Heimdal's signatures
 */
#[no_mangle]
#[c2rust::src_loc = "58:1"]
pub unsafe extern "C" fn gss_export_cred(mut minor_status: *mut OM_uint32,
                                         mut cred_handle: gss_cred_id_t,
                                         mut token: gss_buffer_t)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut status: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut cred: gss_union_cred_t = 0 as *mut gss_cred_id_struct;
    let mut mech_oid: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut public_oid: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut mech: gss_mechanism = 0 as *mut gss_config;
    let mut mech_token: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    let mut i: libc::c_int = 0;
    status = val_exp_cred_args(minor_status, cred_handle, token);
    if status != 0 as libc::c_int as libc::c_uint { return status }
    k5_buf_init_dynamic(&mut buf);
    cred = cred_handle as gss_union_cred_t;
    i = 0 as libc::c_int;
    loop  {
        if !(i < (*cred).count) {
            current_block = 4068382217303356765;
            break ;
        }
        /* Get an export token for this mechanism. */
        mech_oid =
            &mut *(*cred).mechs_array.offset(i as isize) as
                *mut gss_OID_desc_struct;
        public_oid = gssint_get_public_oid(mech_oid as gss_const_OID);
        mech = gssint_get_mechanism(mech_oid as gss_const_OID);
        if public_oid.is_null() || mech.is_null() {
            status = (10 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
            current_block = 15146010626366536021;
            break ;
        } else if (*mech).gss_export_cred.is_none() {
            status = (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
            current_block = 15146010626366536021;
            break ;
        } else {
            status =
                (*mech).gss_export_cred.expect("non-null function pointer")(minor_status,
                                                                            *(*cred).cred_array.offset(i
                                                                                                           as
                                                                                                           isize),
                                                                            &mut mech_token);
            if status != 0 as libc::c_int as libc::c_uint {
                *minor_status =
                    gssint_mecherrmap_map(*minor_status,
                                          &mut (*mech).mech_type);
                current_block = 15146010626366536021;
                break ;
            } else {
                /* Append the mech OID and token to buf. */
                k5_buf_add_uint32_be(&mut buf, (*public_oid).length);
                k5_buf_add_len(&mut buf, (*public_oid).elements,
                               (*public_oid).length as size_t);
                k5_buf_add_uint32_be(&mut buf, mech_token.length as uint32_t);
                k5_buf_add_len(&mut buf, mech_token.value, mech_token.length);
                gss_release_buffer(&mut tmpmin, &mut mech_token);
                i += 1
            }
        }
    }
    match current_block {
        4068382217303356765 => {
            return k5buf_to_gss(minor_status, &mut buf, token)
        }
        _ => { k5_buf_free(&mut buf); return status }
    };
}
