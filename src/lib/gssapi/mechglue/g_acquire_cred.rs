use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:29"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:29"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:29"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/sys/types.h:29"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/mechglue/mglueP.h:29"]
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
    #[c2rust::src_loc = "60:1"]
    pub type gss_union_cred_t = *mut gss_cred_id_struct;
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
    #[c2rust::src_loc = "36:1"]
    pub type gss_union_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "60:1"]
    pub type gss_union_cred_desc = gss_cred_id_struct;
    use super::gssapi_h::{gss_OID, gss_buffer_t, gss_name_t, gss_cred_id_t,
                          gss_OID_desc, OM_uint32, gss_OID_set, gss_ctx_id_t,
                          gss_channel_bindings_t, gss_qop_t, gss_cred_usage_t,
                          gss_const_OID, gss_const_buffer_t,
                          gss_OID_desc_struct};
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
        #[c2rust::src_loc = "767:1"]
        pub fn gssint_make_public_oid_set(minor_status: *mut OM_uint32,
                                          oids: gss_OID, count: libc::c_int,
                                          public_set: *mut gss_OID_set)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "779:1"]
        pub fn gssint_release_internal_name(_: *mut OM_uint32, _: gss_OID,
                                            _: *mut gss_name_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "766:1"]
        pub fn gssint_get_public_oid(internal_oid: gss_const_OID) -> gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "773:1"]
        pub fn gssint_import_internal_name(_: *mut OM_uint32, _: gss_OID,
                                           _: gss_union_name_t,
                                           _: *mut gss_name_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "769:1"]
        pub fn gssint_get_mechanism(_: gss_const_OID) -> gss_mechanism;
        #[no_mangle]
        #[c2rust::src_loc = "792:1"]
        pub fn gssint_get_mechanism_cred(_: gss_union_cred_t, _: gss_OID)
         -> gss_cred_id_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:29"]
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
    pub type gss_OID_set_desc = gss_OID_set_desc_struct;
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
    /* Function Prototypes */
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
    /* mech_type (used to be const) */
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
    /* acceptor_cred_handle */
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
    /* New for V2 */
    /* minor_status */
    /* context_handle */
    /* qop_req */
    /* message_buffer */
    /* message_token */
    /* New for V2 */
    /* minor_status */
    /* context_handle */
    /* message_buffer */
    /* message_token */
    /* qop_state */
    /* New for V2 */
    /* minor_status */
    /* context_handle */
    /* conf_req_flag */
    /* qop_req */
    /* input_message_buffer */
    /* conf_state */
    /* output_message_buffer */
    /* New for V2 */
    /* minor_status */
    /* context_handle */
    /* input_message_buffer */
    /* output_message_buffer */
    /* conf_state */
    /* qop_state */
    /* minor_status */
    /* status_value */
    /* status_type */
    /* mech_type (used to be const) */
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
    /* input_name_type(used to be const) */
    /* output_name */
    /* minor_status */
    /* input_name */
    /* minor_status */
    /* buffer */
    /* minor_status */
    /* set */
    /* minor_status */
    /* cred_handle */
    /* name */
    /* lifetime */
    /* cred_usage */
    /* mechanisms */
    /* Last argument new for V2 */
    /* minor_status */
    /* context_handle */
    /* src_name */
    /* targ_name */
    /* lifetime_rec */
    /* mech_type */
    /* ctx_flags */
    /* locally_initiated */
    /* open */
    /* New for V2 */
    /* minor_status */
    /* context_handle */
    /* conf_req_flag */
    /* qop_req */
    /* req_output_size */
    /* max_input_size */
    /* New for V2 */
    /* minor_status */
    /* input_name */
    /* input_name_type */
    /* output_name */
    /* New for V2 */
    /* minor_status */
    /* input_name */
    /* desired_name_type */
    /* output_name */
    /* New for V2 */
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
    /* New for V2 */
    /* minor_status */
    /* cred_handle */
    /* mech_type */
    /* name */
    /* initiator_lifetime */
    /* acceptor_lifetime */
    /* cred_usage */
    /* New for V2 */
    /* minor_status */
    /* context_handle */
    /* interprocess_token */
    /* New for V2 */
    /* minor_status */
    /* interprocess_token */
    /* context_handle */
    /* New for V2 */
    /* minor_status */
    /* oid */
    /* New for V2 */
    /* minor_status */
    /* oid_set */
    /* New for V2 */
    /* minor_status */
    /* member_oid */
    /* oid_set */
    /* New for V2 */
    /* minor_status */
    /* member */
    /* set */
    /* present */
    /* New for V2 */
    /* minor_status */
    /* oid_str */
    /* oid */
    /* New for V2 */
    /* minor_status */
    /* oid */
    /* oid_str */
    /* New for V2 */
    /* minor_status */
    /* mechanism */
    /* name_types */
    /* New for V2 */
    /* minor_status */
    /* input_name */
    /* mech_types */
    /*
 * The following routines are obsolete variants of gss_get_mic, gss_wrap,
 * gss_verify_mic and gss_unwrap.  They should be provided by GSSAPI V2
 * implementations for backwards compatibility with V1 applications.  Distinct
 * entrypoints (as opposed to #defines) should be provided, to allow GSSAPI
 * V1 applications to link against GSSAPI V2 implementations.
 */
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
    /* New for V2 */
    /* minor_status */
    /* input_name */
    /* exported_name */
    /* New for V2 */
    /* minor_status */
    /* input_name */
    /* dest_name */
    /* New for V2 */
    /* minor_status */
    /* input_name */
    /* mech_type */
    /* output_name */
    /* RFC 4401 */
    /* minor_status */
    /* context */
    /* prf_key */
    /* prf_in */
    /* desired_output_len */
    /* prf_out */
    /* minor_status */
    /* input_cred_handle */
    /* input_usage */
    /* desired_mech */
    /* overwrite_cred */
    /* default_cred */
    /* elements_stored */
    /* cred_usage_stored */
    /* minor_status */
    /* cred_handle */
    /* mech_set */
    /* XXXX these are not part of the GSSAPI C bindings!  (but should be) */
    /* XXXX This is a necessary evil until the spec is fixed */
    /*
 * RFC 5587
 */
    #[c2rust::src_loc = "850:1"]
    pub type gss_const_OID = *const gss_OID_desc;
    #[c2rust::src_loc = "845:1"]
    pub type gss_const_buffer_t = *const gss_buffer_desc;
    #[c2rust::src_loc = "851:1"]
    pub type gss_const_OID_set = *const gss_OID_set_desc;
    use super::mglueP_h::{gss_name_struct, gss_cred_id_struct};
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "569:1"]
        pub fn gss_release_name(_: *mut OM_uint32, _: *mut gss_name_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "853:1"]
        pub fn gss_indicate_mechs_by_attrs(_: *mut OM_uint32,
                                           _: gss_const_OID_set,
                                           _: gss_const_OID_set,
                                           _: gss_const_OID_set,
                                           _: *mut gss_OID_set) -> OM_uint32;
        /* mechs */
        /* minor_status */
        /* mech */
        /* mech_attrs */
        /* known_mech_attrs */
        /* minor_status */
        /* mech_attr */
        /* name */
        /* short_desc */
        /* long_desc */
        #[no_mangle]
        #[c2rust::src_loc = "883:33"]
        pub static mut GSS_C_MA_NOT_DFLT_MECH: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "882:33"]
        pub static mut GSS_C_MA_DEPRECATED: gss_const_OID;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:29"]
pub mod gssapi_ext_h {
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
    #[c2rust::src_loc = "248:1"]
    pub type gss_iov_buffer_desc = gss_iov_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "248:16"]
    pub struct gss_iov_buffer_desc_struct {
        pub type_0: OM_uint32,
        pub buffer: gss_buffer_desc,
    }
    #[c2rust::src_loc = "488:1"]
    pub type gss_any_t = *mut gss_any;
    #[c2rust::src_loc = "134:1"]
    pub type gss_buffer_set_t = *mut gss_buffer_set_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "134:16"]
    pub struct gss_buffer_set_desc_struct {
        pub count: size_t,
        pub elements: *mut gss_buffer_desc,
    }
    use super::gssapi_h::{OM_uint32, gss_buffer_desc};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "488:16"]
        pub type gss_any;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:29"]
pub mod gssapiP_generic_h {
    use super::gssapi_h::{OM_uint32, gss_OID_set, gss_OID_desc};
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
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn generic_gss_release_oid_set(_: *mut OM_uint32,
                                           _: *mut gss_OID_set) -> OM_uint32;
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
        #[no_mangle]
        #[c2rust::src_loc = "263:1"]
        pub fn gssint_mecherrmap_map_errcode(errcode: OM_uint32) -> OM_uint32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src = "/usr/include/string.h:29"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
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
#[c2rust::header_src = "/usr/include/stdlib.h:29"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
pub use self::types_h::{__uint32_t, __ssize_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::stddef_h::size_t;
pub use self::sys_types_h::ssize_t;
pub use self::mglueP_h::{gss_name_struct, gss_cred_id_struct,
                         gss_union_cred_t, gss_mechanism, gss_config,
                         gss_union_name_t, gss_union_cred_desc,
                         gssint_make_public_oid_set,
                         gssint_release_internal_name, gssint_get_public_oid,
                         gssint_import_internal_name, gssint_get_mechanism,
                         gssint_get_mechanism_cred, gssint_select_mech_type};
pub use self::gssapi_h::{gss_name_t, gss_OID, gss_OID_desc_struct, OM_uint32,
                         gss_uint32, gss_buffer_t, gss_buffer_desc_struct,
                         gss_cred_id_t, gss_ctx_id_t, gss_OID_desc,
                         gss_OID_set_desc_struct, gss_OID_set_desc,
                         gss_OID_set, gss_buffer_desc,
                         gss_channel_bindings_struct, gss_channel_bindings_t,
                         gss_qop_t, gss_cred_usage_t, gss_const_OID,
                         gss_const_buffer_t, gss_const_OID_set,
                         gss_ctx_id_struct, gss_release_cred,
                         gss_release_name, gss_release_buffer,
                         gss_indicate_mechs_by_attrs, GSS_C_MA_NOT_DFLT_MECH,
                         GSS_C_MA_DEPRECATED};
pub use self::gssapi_ext_h::{gss_const_key_value_set_t,
                             gss_key_value_set_desc, gss_key_value_set_struct,
                             gss_key_value_element_desc,
                             gss_key_value_element_struct,
                             gss_iov_buffer_desc, gss_iov_buffer_desc_struct,
                             gss_any_t, gss_buffer_set_t,
                             gss_buffer_set_desc_struct, gss_any};
use self::gssapiP_generic_h::{generic_gss_release_oid_set,
                              gssint_mecherrmap_map,
                              gssint_mecherrmap_map_errcode};
use self::string_h::{memcpy, memcmp, memset};
use self::stdlib_h::{free, malloc, realloc, calloc};
/* #pragma ident	"@(#)g_acquire_cred.c	1.22	04/02/23 SMI" */
/*
 * Copyright 1996 by Sun Microsystems, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of Sun Microsystems not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. Sun Microsystems makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * SUN MICROSYSTEMS DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL SUN MICROSYSTEMS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 *  glue routine for gss_acquire_cred
 */
#[c2rust::src_loc = "38:1"]
unsafe extern "C" fn val_acq_cred_args(mut minor_status: *mut OM_uint32,
                                       mut desired_name: gss_name_t,
                                       mut time_req: OM_uint32,
                                       mut desired_mechs: gss_OID_set,
                                       mut cred_usage: libc::c_int,
                                       mut cred_store:
                                           gss_const_key_value_set_t,
                                       mut output_cred_handle:
                                           *mut gss_cred_id_t,
                                       mut actual_mechs: *mut gss_OID_set,
                                       mut time_rec: *mut OM_uint32)
 -> OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0 as libc::c_int as OM_uint32
    }
    if !output_cred_handle.is_null() {
        *output_cred_handle = 0 as gss_cred_id_t
    }
    if !actual_mechs.is_null() { *actual_mechs = 0 as gss_OID_set }
    if !time_rec.is_null() { *time_rec = 0 as libc::c_int as OM_uint32 }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    if output_cred_handle.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    if cred_usage != 2 as libc::c_int && cred_usage != 1 as libc::c_int &&
           cred_usage != 0 as libc::c_int {
        if !minor_status.is_null() {
            *minor_status = 22 as libc::c_int as OM_uint32;
            *minor_status = gssint_mecherrmap_map_errcode(*minor_status)
        }
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return 0 as libc::c_int as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "87:1"]
pub unsafe extern "C" fn gss_acquire_cred(mut minor_status: *mut OM_uint32,
                                          mut desired_name: gss_name_t,
                                          mut time_req: OM_uint32,
                                          mut desired_mechs: gss_OID_set,
                                          mut cred_usage: libc::c_int,
                                          mut output_cred_handle:
                                              *mut gss_cred_id_t,
                                          mut actual_mechs: *mut gss_OID_set,
                                          mut time_rec: *mut OM_uint32)
 -> OM_uint32 {
    return gss_acquire_cred_from(minor_status, desired_name, time_req,
                                 desired_mechs, cred_usage,
                                 0 as gss_const_key_value_set_t,
                                 output_cred_handle, actual_mechs, time_rec);
}
#[no_mangle]
#[c2rust::src_loc = "112:1"]
pub unsafe extern "C" fn gss_acquire_cred_from(mut minor_status:
                                                   *mut OM_uint32,
                                               mut desired_name: gss_name_t,
                                               mut time_req: OM_uint32,
                                               mut desired_mechs: gss_OID_set,
                                               mut cred_usage: libc::c_int,
                                               mut cred_store:
                                                   gss_const_key_value_set_t,
                                               mut output_cred_handle:
                                                   *mut gss_cred_id_t,
                                               mut actual_mechs:
                                                   *mut gss_OID_set,
                                               mut time_rec: *mut OM_uint32)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut major: OM_uint32 =
        (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    let mut tmpMinor: OM_uint32 = 0;
    let mut first_major: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut first_minor: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut initTimeOut: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut acceptTimeOut: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut outTime: OM_uint32 = 0xffffffff as libc::c_ulong as OM_uint32;
    let mut mechs: gss_OID_set = 0 as gss_OID_set;
    let mut except_attrs: gss_OID_set_desc =
        gss_OID_set_desc{count: 0, elements: 0 as *mut gss_OID_desc_struct,};
    let mut attr_oids: [gss_OID_desc; 2] =
        [gss_OID_desc{length: 0, elements: 0 as *mut libc::c_void,}; 2];
    let mut i: libc::c_uint = 0;
    let mut creds: gss_union_cred_t = 0 as gss_union_cred_t;
    major =
        val_acq_cred_args(minor_status, desired_name, time_req, desired_mechs,
                          cred_usage, cred_store, output_cred_handle,
                          actual_mechs, time_rec);
    if !(major != 0 as libc::c_int as libc::c_uint) {
        /*
     * if desired_mechs equals GSS_C_NULL_OID_SET, then try to
     * acquire credentials for all non-deprecated mechanisms.
     */
        if desired_mechs.is_null() {
            attr_oids[0 as libc::c_int as usize] = *GSS_C_MA_DEPRECATED;
            attr_oids[1 as libc::c_int as usize] = *GSS_C_MA_NOT_DFLT_MECH;
            except_attrs.count = 2 as libc::c_int as size_t;
            except_attrs.elements = attr_oids.as_mut_ptr();
            major =
                gss_indicate_mechs_by_attrs(minor_status,
                                            0 as gss_OID_set as
                                                gss_const_OID_set,
                                            &mut except_attrs as
                                                *mut gss_OID_set_desc as
                                                gss_const_OID_set,
                                            0 as gss_OID_set as
                                                gss_const_OID_set,
                                            &mut mechs);
            if major != 0 as libc::c_int as libc::c_uint {
                current_block = 13371929842352464343;
            } else { current_block = 12599329904712511516; }
        } else {
            mechs = desired_mechs;
            current_block = 12599329904712511516;
        }
        match current_block {
            13371929842352464343 => { }
            _ => {
                if (*mechs).count == 0 as libc::c_int as libc::c_ulong {
                    major =
                        (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
                } else {
                    /* allocate the output credential structure */
                    creds =
                        calloc(1 as libc::c_int as libc::c_ulong,
                               ::std::mem::size_of::<gss_union_cred_desc>() as
                                   libc::c_ulong) as gss_union_cred_t;
                    if creds.is_null() {
                        major =
                            (13 as libc::c_ulong as OM_uint32) <<
                                16 as libc::c_int;
                        *minor_status = 12 as libc::c_int as OM_uint32
                    } else {
                        (*creds).count = 0 as libc::c_int;
                        (*creds).loopback = creds;
                        /* for each requested mech attempt to obtain a credential */
                        i = 0 as libc::c_int as libc::c_uint; /* for */
                        major =
                            (16 as libc::c_ulong as OM_uint32) <<
                                16 as libc::c_int;
                        while (i as libc::c_ulong) < (*mechs).count {
                            major =
                                gss_add_cred_from(&mut tmpMinor,
                                                  creds as gss_cred_id_t,
                                                  desired_name,
                                                  &mut *(*mechs).elements.offset(i
                                                                                     as
                                                                                     isize),
                                                  cred_usage, time_req,
                                                  time_req, cred_store,
                                                  0 as *mut gss_cred_id_t,
                                                  0 as *mut gss_OID_set,
                                                  if !time_rec.is_null() {
                                                      &mut initTimeOut
                                                  } else {
                                                      0 as *mut OM_uint32
                                                  },
                                                  if !time_rec.is_null() {
                                                      &mut acceptTimeOut
                                                  } else {
                                                      0 as *mut OM_uint32
                                                  });
                            if major == 0 as libc::c_int as libc::c_uint {
                                /* update the credential's time */
                                if cred_usage == 2 as libc::c_int {
                                    if outTime > acceptTimeOut {
                                        outTime = acceptTimeOut
                                    }
                                } else if cred_usage == 1 as libc::c_int {
                                    if outTime > initTimeOut {
                                        outTime = initTimeOut
                                    }
                                } else if initTimeOut > acceptTimeOut {
                                    outTime =
                                        if outTime > acceptTimeOut {
                                            acceptTimeOut
                                        } else { outTime }
                                } else {
                                    outTime =
                                        if outTime > initTimeOut {
                                            initTimeOut
                                        } else { outTime }
                                }
                            } else if first_major ==
                                          0 as libc::c_int as libc::c_uint {
                                first_major = major;
                                first_minor = tmpMinor
                            }
                            i = i.wrapping_add(1)
                        }
                        /*
		 * time_rec is the lesser of the
		 * init/accept times
		 */
                        /* If we didn't get any creds, return the error status from the first mech
     * (which is often the preferred one). */
                        if (*creds).count < 1 as libc::c_int {
                            major = first_major;
                            *minor_status = first_minor
                        } else {
                            major = 0 as libc::c_int as OM_uint32;
                            /*
     * fill in output parameters
     * setup the actual mechs output parameter
     */
                            if !actual_mechs.is_null() {
                                major =
                                    gssint_make_public_oid_set(minor_status,
                                                               (*creds).mechs_array,
                                                               (*creds).count,
                                                               actual_mechs);
                                if major &
                                       ((0o377 as libc::c_ulong as OM_uint32)
                                            << 24 as libc::c_int |
                                            (0o377 as libc::c_ulong as
                                                 OM_uint32) <<
                                                16 as libc::c_int) != 0 {
                                    current_block = 13371929842352464343;
                                } else {
                                    current_block = 9627623479216730126;
                                }
                            } else { current_block = 9627623479216730126; }
                            match current_block {
                                13371929842352464343 => { }
                                _ => {
                                    if !time_rec.is_null() {
                                        *time_rec = outTime
                                    }
                                    *output_cred_handle =
                                        creds as gss_cred_id_t
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if major &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        gss_release_cred(&mut tmpMinor,
                         &mut creds as *mut gss_union_cred_t as
                             *mut gss_cred_id_t);
    }
    if desired_mechs.is_null() {
        generic_gss_release_oid_set(&mut tmpMinor, &mut mechs);
    }
    return major;
}
#[c2rust::src_loc = "256:1"]
unsafe extern "C" fn val_add_cred_args(mut minor_status: *mut OM_uint32,
                                       mut input_cred_handle: gss_cred_id_t,
                                       mut desired_name: gss_name_t,
                                       mut desired_mech: gss_OID,
                                       mut cred_usage: gss_cred_usage_t,
                                       mut cred_store:
                                           gss_const_key_value_set_t,
                                       mut initiator_time_req: OM_uint32,
                                       mut acceptor_time_req: OM_uint32,
                                       mut output_cred_handle:
                                           *mut gss_cred_id_t,
                                       mut actual_mechs: *mut gss_OID_set,
                                       mut initiator_time_rec: *mut OM_uint32,
                                       mut acceptor_time_rec: *mut OM_uint32)
 -> OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0 as libc::c_int as OM_uint32
    }
    if !output_cred_handle.is_null() {
        *output_cred_handle = 0 as gss_cred_id_t
    }
    if !actual_mechs.is_null() { *actual_mechs = 0 as gss_OID_set }
    if !acceptor_time_rec.is_null() {
        *acceptor_time_rec = 0 as libc::c_int as OM_uint32
    }
    if !initiator_time_rec.is_null() {
        *initiator_time_rec = 0 as libc::c_int as OM_uint32
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    if input_cred_handle.is_null() && output_cred_handle.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                   (7 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if cred_usage != 2 as libc::c_int && cred_usage != 1 as libc::c_int &&
           cred_usage != 0 as libc::c_int {
        if !minor_status.is_null() {
            *minor_status = 22 as libc::c_int as OM_uint32;
            *minor_status = gssint_mecherrmap_map_errcode(*minor_status)
        }
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return 0 as libc::c_int as OM_uint32;
}
/* Copy a mechanism credential (with the mechanism given by mech_oid) as
 * faithfully as possible. */
#[c2rust::src_loc = "313:1"]
unsafe extern "C" fn copy_mech_cred(mut minor_status: *mut OM_uint32,
                                    mut cred_in: gss_cred_id_t,
                                    mut mech_oid: gss_OID,
                                    mut cred_out: *mut gss_cred_id_t)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut mech: gss_mechanism = 0 as *mut gss_config;
    let mut buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut name: gss_name_t = 0 as *mut gss_name_struct;
    let mut life: OM_uint32 = 0;
    let mut usage: gss_cred_usage_t = 0;
    let mut oidset: gss_OID_set_desc =
        gss_OID_set_desc{count: 0, elements: 0 as *mut gss_OID_desc_struct,};
    mech = gssint_get_mechanism(mech_oid as gss_const_OID);
    if mech.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if (*mech).gss_export_cred.is_some() && (*mech).gss_import_cred.is_some()
       {
        status =
            (*mech).gss_export_cred.expect("non-null function pointer")(minor_status,
                                                                        cred_in,
                                                                        &mut buf);
        if status != 0 as libc::c_int as libc::c_uint { return status }
        status =
            (*mech).gss_import_cred.expect("non-null function pointer")(minor_status,
                                                                        &mut buf,
                                                                        cred_out);
        gss_release_buffer(&mut tmpmin, &mut buf);
    } else if (*mech).gss_inquire_cred.is_some() &&
                  (*mech).gss_acquire_cred.is_some() {
        status =
            (*mech).gss_inquire_cred.expect("non-null function pointer")(minor_status,
                                                                         cred_in,
                                                                         &mut name,
                                                                         &mut life,
                                                                         &mut usage,
                                                                         0 as
                                                                             *mut gss_OID_set);
        if status != 0 as libc::c_int as libc::c_uint { return status }
        oidset.count = 1 as libc::c_int as size_t;
        oidset.elements = gssint_get_public_oid(mech_oid as gss_const_OID);
        status =
            (*mech).gss_acquire_cred.expect("non-null function pointer")(minor_status,
                                                                         name,
                                                                         life,
                                                                         &mut oidset,
                                                                         usage,
                                                                         cred_out,
                                                                         0 as
                                                                             *mut gss_OID_set,
                                                                         0 as
                                                                             *mut OM_uint32);
        gss_release_name(&mut tmpmin, &mut name);
    } else {
        status = (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return status;
}
/* Copy a union credential from cred_in to *cred_out. */
#[c2rust::src_loc = "352:1"]
unsafe extern "C" fn copy_union_cred(mut minor_status: *mut OM_uint32,
                                     mut cred_in: gss_cred_id_t,
                                     mut cred_out: *mut gss_union_cred_t)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut status: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut cred: gss_union_cred_t = cred_in as gss_union_cred_t;
    let mut ncred: gss_union_cred_t = 0 as gss_union_cred_t;
    let mut tmpcred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut i: libc::c_int = 0;
    ncred =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<gss_cred_id_struct>() as libc::c_ulong)
            as gss_union_cred_t;
    if ncred.is_null() {
        current_block = 16626164785316884790;
    } else {
        (*ncred).mechs_array =
            calloc((*cred).count as libc::c_ulong,
                   ::std::mem::size_of::<gss_OID_desc_struct>() as
                       libc::c_ulong) as gss_OID;
        (*ncred).cred_array =
            calloc((*cred).count as libc::c_ulong,
                   ::std::mem::size_of::<gss_cred_id_t>() as libc::c_ulong) as
                *mut gss_cred_id_t;
        if (*ncred).mechs_array.is_null() || (*ncred).cred_array.is_null() {
            current_block = 16626164785316884790;
        } else {
            (*ncred).count = (*cred).count;
            i = 0 as libc::c_int;
            loop  {
                if !(i < (*cred).count) {
                    current_block = 2370887241019905314;
                    break ;
                }
                /* Copy this element's mechanism OID. */
                let ref mut fresh0 =
                    (*(*ncred).mechs_array.offset(i as isize)).elements;
                *fresh0 =
                    malloc((*(*cred).mechs_array.offset(i as isize)).length as
                               libc::c_ulong);
                if (*(*ncred).mechs_array.offset(i as
                                                     isize)).elements.is_null()
                   {
                    current_block = 16626164785316884790;
                    break ;
                }
                memcpy((*(*ncred).mechs_array.offset(i as isize)).elements,
                       (*(*cred).mechs_array.offset(i as isize)).elements,
                       (*(*cred).mechs_array.offset(i as isize)).length as
                           libc::c_ulong);
                (*(*ncred).mechs_array.offset(i as isize)).length =
                    (*(*cred).mechs_array.offset(i as isize)).length;
                /* Copy this element's mechanism cred. */
                status =
                    copy_mech_cred(minor_status,
                                   *(*cred).cred_array.offset(i as isize),
                                   &mut *(*cred).mechs_array.offset(i as
                                                                        isize),
                                   &mut *(*ncred).cred_array.offset(i as
                                                                        isize));
                if status != 0 as libc::c_int as libc::c_uint {
                    current_block = 9850252977296903552;
                    break ;
                }
                i += 1
            }
            match current_block {
                9850252977296903552 => { }
                16626164785316884790 => { }
                _ => {
                    (*ncred).loopback = ncred;
                    *cred_out = ncred;
                    return 0 as libc::c_int as OM_uint32
                }
            }
        }
    }
    match current_block {
        16626164785316884790 => {
            status = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
            *minor_status = 12 as libc::c_int as OM_uint32
        }
        _ => { }
    }
    tmpcred = ncred as gss_cred_id_t;
    gss_release_cred(&mut tmpmin, &mut tmpcred);
    return status;
}
/* V2 KRB5_CALLCONV */
#[no_mangle]
#[c2rust::src_loc = "399:1"]
pub unsafe extern "C" fn gss_add_cred(mut minor_status: *mut OM_uint32,
                                      mut input_cred_handle: gss_cred_id_t,
                                      mut desired_name: gss_name_t,
                                      mut desired_mech: gss_OID,
                                      mut cred_usage: gss_cred_usage_t,
                                      mut initiator_time_req: OM_uint32,
                                      mut acceptor_time_req: OM_uint32,
                                      mut output_cred_handle:
                                          *mut gss_cred_id_t,
                                      mut actual_mechs: *mut gss_OID_set,
                                      mut initiator_time_rec: *mut OM_uint32,
                                      mut acceptor_time_rec: *mut OM_uint32)
 -> OM_uint32 {
    return gss_add_cred_from(minor_status, input_cred_handle, desired_name,
                             desired_mech, cred_usage, initiator_time_req,
                             acceptor_time_req,
                             0 as gss_const_key_value_set_t,
                             output_cred_handle, actual_mechs,
                             initiator_time_rec, acceptor_time_rec);
}
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
#[no_mangle]
#[c2rust::src_loc = "424:1"]
pub unsafe extern "C" fn gss_add_cred_from(mut minor_status: *mut OM_uint32,
                                           mut input_cred_handle:
                                               gss_cred_id_t,
                                           mut desired_name: gss_name_t,
                                           mut desired_mech: gss_OID,
                                           mut cred_usage: gss_cred_usage_t,
                                           mut initiator_time_req: OM_uint32,
                                           mut acceptor_time_req: OM_uint32,
                                           mut cred_store:
                                               gss_const_key_value_set_t,
                                           mut output_cred_handle:
                                               *mut gss_cred_id_t,
                                           mut actual_mechs: *mut gss_OID_set,
                                           mut initiator_time_rec:
                                               *mut OM_uint32,
                                           mut acceptor_time_rec:
                                               *mut OM_uint32) -> OM_uint32 {
    let mut current_block: u64;
    let mut status: OM_uint32 = 0;
    let mut temp_minor_status: OM_uint32 = 0;
    let mut time_req: OM_uint32 = 0;
    let mut time_rec: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut time_recp: *mut OM_uint32 = 0 as *mut OM_uint32;
    let mut union_name: gss_union_name_t = 0 as *mut gss_name_struct;
    let mut union_cred: gss_union_cred_t = 0 as *mut gss_cred_id_struct;
    let mut internal_name: gss_name_t = 0 as gss_name_t;
    let mut allocated_name: gss_name_t = 0 as gss_name_t;
    let mut mech: gss_mechanism = 0 as *mut gss_config;
    let mut cred: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut tmpcred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut newptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut oidbuf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut target_mechs: gss_OID_set_desc =
        gss_OID_set_desc{count: 0, elements: 0 as *mut gss_OID_desc_struct,};
    let mut selected_mech: gss_OID = 0 as gss_OID;
    status =
        val_add_cred_args(minor_status, input_cred_handle, desired_name,
                          desired_mech, cred_usage, cred_store,
                          initiator_time_req, acceptor_time_req,
                          output_cred_handle, actual_mechs,
                          initiator_time_rec, acceptor_time_rec);
    if status != 0 as libc::c_int as libc::c_uint { return status }
    status =
        gssint_select_mech_type(minor_status, desired_mech as gss_const_OID,
                                &mut selected_mech);
    if status != 0 as libc::c_int as libc::c_uint { return status }
    mech = gssint_get_mechanism(selected_mech as gss_const_OID);
    if mech.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    } else {
        if (*mech).gss_acquire_cred.is_none() {
            return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    }
    if input_cred_handle.is_null() {
        /* Create a new credential handle. */
        union_cred =
            malloc(::std::mem::size_of::<gss_union_cred_desc>() as
                       libc::c_ulong) as gss_union_cred_t;
        if union_cred.is_null() {
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        memset(union_cred as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<gss_union_cred_desc>() as libc::c_ulong);
        (*union_cred).loopback = union_cred
    } else if output_cred_handle.is_null() {
        /* Add to the existing handle. */
        union_cred = input_cred_handle as gss_union_cred_t;
        if !gssint_get_mechanism_cred(union_cred, selected_mech).is_null() {
            return (17 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    } else {
        /* Create a new credential handle with the mechanism credentials of the
	 * input handle plus the acquired mechanism credential. */
        status =
            copy_union_cred(minor_status, input_cred_handle, &mut union_cred);
        if status != 0 as libc::c_int as libc::c_uint { return status }
    }
    /* We may need to create a mechanism specific name. */
    if !desired_name.is_null() {
        union_name = desired_name as gss_union_name_t;
        if !(*union_name).mech_type.is_null() &&
               ((*(*union_name).mech_type).length == (*selected_mech).length
                    &&
                    memcmp((*(*union_name).mech_type).elements,
                           (*selected_mech).elements,
                           (*(*union_name).mech_type).length as libc::c_ulong)
                        == 0 as libc::c_int) {
            internal_name = (*union_name).mech_name;
            current_block = 5529461102203738653;
        } else if gssint_import_internal_name(minor_status, selected_mech,
                                              union_name, &mut allocated_name)
                      != 0 as libc::c_int as libc::c_uint {
            status = (2 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
            current_block = 1609792147738124217;
        } else {
            internal_name = allocated_name;
            current_block = 5529461102203738653;
        }
    } else { current_block = 5529461102203738653; }
    match current_block {
        5529461102203738653 => {
            if cred_usage == 2 as libc::c_int {
                time_req = acceptor_time_req
            } else if cred_usage == 1 as libc::c_int {
                time_req = initiator_time_req
            } else if cred_usage == 0 as libc::c_int {
                time_req =
                    if acceptor_time_req > initiator_time_req {
                        acceptor_time_req
                    } else { initiator_time_req }
            } else { time_req = 0 as libc::c_int as OM_uint32 }
            target_mechs.count = 1 as libc::c_int as size_t;
            target_mechs.elements =
                gssint_get_public_oid(selected_mech as gss_const_OID);
            if target_mechs.elements.is_null() {
                status =
                    (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
            } else {
                if !initiator_time_rec.is_null() ||
                       !acceptor_time_rec.is_null() {
                    time_recp = &mut time_rec
                }
                if (*mech).gss_acquire_cred_from.is_some() {
                    status =
                        (*mech).gss_acquire_cred_from.expect("non-null function pointer")(minor_status,
                                                                                          internal_name,
                                                                                          time_req,
                                                                                          &mut target_mechs,
                                                                                          cred_usage,
                                                                                          cred_store,
                                                                                          &mut cred,
                                                                                          0
                                                                                              as
                                                                                              *mut gss_OID_set,
                                                                                          time_recp);
                    current_block = 5235537862154438448;
                } else if cred_store.is_null() {
                    status =
                        (*mech).gss_acquire_cred.expect("non-null function pointer")(minor_status,
                                                                                     internal_name,
                                                                                     time_req,
                                                                                     &mut target_mechs,
                                                                                     cred_usage,
                                                                                     &mut cred,
                                                                                     0
                                                                                         as
                                                                                         *mut gss_OID_set,
                                                                                     time_recp);
                    current_block = 5235537862154438448;
                } else {
                    status =
                        (16 as libc::c_ulong as OM_uint32) <<
                            16 as libc::c_int;
                    current_block = 1609792147738124217;
                }
                match current_block {
                    1609792147738124217 => { }
                    _ => {
                        if status != 0 as libc::c_int as libc::c_uint {
                            *minor_status =
                                gssint_mecherrmap_map(*minor_status,
                                                      &mut (*mech).mech_type)
                        } else {
                            /* Extend the arrays in the union cred. */
                            newptr =
                                realloc((*union_cred).mechs_array as
                                            *mut libc::c_void,
                                        (((*union_cred).count +
                                              1 as libc::c_int) as
                                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<gss_OID_desc>()
                                                                             as
                                                                             libc::c_ulong));
                            if newptr.is_null() {
                                status =
                                    (13 as libc::c_ulong as OM_uint32) <<
                                        16 as libc::c_int
                            } else {
                                (*union_cred).mechs_array = newptr as gss_OID;
                                newptr =
                                    realloc((*union_cred).cred_array as
                                                *mut libc::c_void,
                                            (((*union_cred).count +
                                                  1 as libc::c_int) as
                                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<gss_cred_id_t>()
                                                                                 as
                                                                                 libc::c_ulong));
                                if newptr.is_null() {
                                    status =
                                        (13 as libc::c_ulong as OM_uint32) <<
                                            16 as libc::c_int
                                } else {
                                    (*union_cred).cred_array =
                                        newptr as *mut gss_cred_id_t;
                                    if !acceptor_time_rec.is_null() {
                                        if cred_usage == 2 as libc::c_int ||
                                               cred_usage == 0 as libc::c_int
                                           {
                                            *acceptor_time_rec = time_rec
                                        }
                                    }
                                    if !initiator_time_rec.is_null() {
                                        if cred_usage == 1 as libc::c_int ||
                                               cred_usage == 0 as libc::c_int
                                           {
                                            *initiator_time_rec = time_rec
                                        }
                                    }
                                    oidbuf =
                                        malloc((*selected_mech).length as
                                                   libc::c_ulong);
                                    if !oidbuf.is_null() {
                                        let ref mut fresh1 =
                                            (*(*union_cred).mechs_array.offset((*union_cred).count
                                                                                   as
                                                                                   isize)).elements;
                                        *fresh1 = oidbuf;
                                        memcpy((*(*union_cred).mechs_array.offset((*union_cred).count
                                                                                      as
                                                                                      isize)).elements,
                                               (*selected_mech).elements,
                                               (*selected_mech).length as
                                                   libc::c_ulong);
                                        (*(*union_cred).mechs_array.offset((*union_cred).count
                                                                               as
                                                                               isize)).length
                                            = (*selected_mech).length;
                                        if !actual_mechs.is_null() {
                                            status =
                                                gssint_make_public_oid_set(minor_status,
                                                                           (*union_cred).mechs_array,
                                                                           (*union_cred).count
                                                                               +
                                                                               1
                                                                                   as
                                                                                   libc::c_int,
                                                                           actual_mechs);
                                            if status &
                                                   ((0o377 as libc::c_ulong as
                                                         OM_uint32) <<
                                                        24 as libc::c_int |
                                                        (0o377 as
                                                             libc::c_ulong as
                                                             OM_uint32) <<
                                                            16 as libc::c_int)
                                                   != 0 {
                                                current_block =
                                                    1609792147738124217;
                                            } else {
                                                current_block =
                                                    14294131666767243020;
                                            }
                                        } else {
                                            current_block =
                                                14294131666767243020;
                                        }
                                        match current_block {
                                            1609792147738124217 => { }
                                            _ => {
                                                let ref mut fresh2 =
                                                    *(*union_cred).cred_array.offset((*union_cred).count
                                                                                         as
                                                                                         isize);
                                                *fresh2 = cred;
                                                (*union_cred).count += 1;
                                                if !output_cred_handle.is_null()
                                                   {
                                                    *output_cred_handle =
                                                        union_cred as
                                                            gss_cred_id_t
                                                }
                                                /* We're done with the internal name. Free it if we allocated it. */
                                                if !allocated_name.is_null() {
                                                    gssint_release_internal_name(&mut temp_minor_status,
                                                                                 selected_mech,
                                                                                 &mut allocated_name);
                                                }
                                                return 0 as libc::c_int as
                                                           OM_uint32
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    if !cred.is_null() && (*mech).gss_release_cred.is_some() {
        (*mech).gss_release_cred.expect("non-null function pointer")(&mut temp_minor_status,
                                                                     &mut cred);
    }
    if !allocated_name.is_null() {
        gssint_release_internal_name(&mut temp_minor_status, selected_mech,
                                     &mut allocated_name);
    }
    if !output_cred_handle.is_null() && !union_cred.is_null() {
        tmpcred = union_cred;
        gss_release_cred(&mut temp_minor_status, &mut tmpcred);
    }
    free(oidbuf);
    return status;
}
