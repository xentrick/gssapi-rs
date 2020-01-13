use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:29"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = libc::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = libc::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "174:1"]
    pub type __blksize_t = libc::c_long;
    #[c2rust::src_loc = "179:1"]
    pub type __blkcnt_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:29"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
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
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:29"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:29"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__time_t, __syscall_slong_t};
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:29"]
pub mod thread_shared_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:16"]
    pub struct __pthread_internal_list {
        pub __prev: *mut __pthread_internal_list,
        pub __next: *mut __pthread_internal_list,
    }
    #[c2rust::src_loc = "82:1"]
    pub type __pthread_list_t = __pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:8"]
    pub struct __pthread_mutex_s {
        pub __lock: libc::c_int,
        pub __count: libc::c_uint,
        pub __owner: libc::c_int,
        pub __nusers: libc::c_uint,
        pub __kind: libc::c_int,
        pub __spins: libc::c_short,
        pub __elision: libc::c_short,
        pub __list: __pthread_list_t,
    }
}
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:29"]
pub mod pthreadtypes_h {
    #[c2rust::src_loc = "53:1"]
    pub type pthread_once_t = libc::c_int;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub union pthread_mutex_t {
        pub __data: __pthread_mutex_s,
        pub __size: [libc::c_char; 40],
        pub __align: libc::c_long,
    }
    use super::thread_shared_types_h::__pthread_mutex_s;
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
    #[c2rust::src_loc = "745:1"]
    pub type gss_mech_info = *mut gss_mech_config;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "745:16"]
    pub struct gss_mech_config {
        pub kmodName: *mut libc::c_char,
        pub uLibName: *mut libc::c_char,
        pub mechNameStr: *mut libc::c_char,
        pub optionStr: *mut libc::c_char,
        pub dl_handle: *mut libc::c_void,
        pub mech_type: gss_OID,
        pub mech: gss_mechanism,
        pub priority: libc::c_int,
        pub freeMech: libc::c_int,
        pub is_interposer: libc::c_int,
        pub int_mech_type: gss_OID,
        pub int_mech: gss_mechanism,
        pub next: *mut gss_mech_config,
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
    use super::gssapi_h::{gss_OID, gss_buffer_t, gss_name_t, gss_cred_id_t,
                          gss_OID_desc, OM_uint32, gss_OID_set, gss_ctx_id_t,
                          gss_channel_bindings_t, gss_qop_t, gss_cred_usage_t,
                          gss_const_OID, gss_const_buffer_t};
    use super::gssapi_ext_h::{gss_buffer_set_t, gss_iov_buffer_desc,
                              gss_any_t, gss_const_key_value_set_t};
    use super::sys_types_h::ssize_t;
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
    use super::mglueP_h::{gss_name_struct, gss_cred_id_struct};
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        /* Function Prototypes */
        #[no_mangle]
        #[c2rust::src_loc = "421:1"]
        pub fn gss_acquire_cred(_: *mut OM_uint32, _: gss_name_t,
                                _: OM_uint32, _: gss_OID_set,
                                _: gss_cred_usage_t, _: *mut gss_cred_id_t,
                                _: *mut gss_OID_set, _: *mut OM_uint32)
         -> OM_uint32;
        /* time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        /* cred_handle */
        #[no_mangle]
        #[c2rust::src_loc = "437:1"]
        pub fn gss_init_sec_context(_: *mut OM_uint32, _: gss_cred_id_t,
                                    _: *mut gss_ctx_id_t, _: gss_name_t,
                                    _: gss_OID, _: OM_uint32, _: OM_uint32,
                                    _: gss_channel_bindings_t,
                                    _: gss_buffer_t, _: *mut gss_OID,
                                    _: gss_buffer_t, _: *mut OM_uint32,
                                    _: *mut OM_uint32) -> OM_uint32;
        /* time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn gss_accept_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_cred_id_t, _: gss_buffer_t,
                                      _: gss_channel_bindings_t,
                                      _: *mut gss_name_t, _: *mut gss_OID,
                                      _: gss_buffer_t, _: *mut OM_uint32,
                                      _: *mut OM_uint32,
                                      _: *mut gss_cred_id_t) -> OM_uint32;
        /* delegated_cred_handle */
        #[no_mangle]
        #[c2rust::src_loc = "467:1"]
        pub fn gss_process_context_token(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: gss_buffer_t) -> OM_uint32;
        /* token_buffer */
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
        /* output_token */
        #[no_mangle]
        #[c2rust::src_loc = "481:1"]
        pub fn gss_context_time(_: *mut OM_uint32, _: gss_ctx_id_t,
                                _: *mut OM_uint32) -> OM_uint32;
        /* time_rec */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "489:1"]
        pub fn gss_get_mic(_: *mut OM_uint32, _: gss_ctx_id_t, _: gss_qop_t,
                           _: gss_buffer_t, _: gss_buffer_t) -> OM_uint32;
        /* message_token */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "499:1"]
        pub fn gss_verify_mic(_: *mut OM_uint32, _: gss_ctx_id_t,
                              _: gss_buffer_t, _: gss_buffer_t,
                              _: *mut gss_qop_t) -> OM_uint32;
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "508:1"]
        pub fn gss_wrap(_: *mut OM_uint32, _: gss_ctx_id_t, _: libc::c_int,
                        _: gss_qop_t, _: gss_buffer_t, _: *mut libc::c_int,
                        _: gss_buffer_t) -> OM_uint32;
        /* output_message_buffer */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "520:1"]
        pub fn gss_unwrap(_: *mut OM_uint32, _: gss_ctx_id_t, _: gss_buffer_t,
                          _: gss_buffer_t, _: *mut libc::c_int,
                          _: *mut gss_qop_t) -> OM_uint32;
        /* qop_state */
        #[no_mangle]
        #[c2rust::src_loc = "530:1"]
        pub fn gss_display_status(_: *mut OM_uint32, _: OM_uint32,
                                  _: libc::c_int, _: gss_OID,
                                  _: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
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
        #[no_mangle]
        #[c2rust::src_loc = "579:1"]
        pub fn gss_release_oid_set(_: *mut OM_uint32, _: *mut gss_OID_set)
         -> OM_uint32;
        /* minor_status */
        /* desired_mech_attrs */
        /* except_mech_attrs */
        /* critical_mech_attrs */
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
        #[c2rust::src_loc = "882:33"]
        pub static mut GSS_C_MA_DEPRECATED: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "692:1"]
        pub fn gss_test_oid_set_member(_: *mut OM_uint32, _: gss_OID,
                                       _: gss_OID_set, _: *mut libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "861:1"]
        pub fn gss_inquire_attrs_for_mech(_: *mut OM_uint32, _: gss_const_OID,
                                          _: *mut gss_OID_set,
                                          _: *mut gss_OID_set) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "546:1"]
        pub fn gss_compare_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_name_t, _: *mut libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "554:1"]
        pub fn gss_display_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_buffer_t, _: *mut gss_OID)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "562:1"]
        pub fn gss_import_name(_: *mut OM_uint32, _: gss_buffer_t, _: gss_OID,
                               _: *mut gss_name_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "569:1"]
        pub fn gss_release_name(_: *mut OM_uint32, _: *mut gss_name_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "584:1"]
        pub fn gss_inquire_cred(_: *mut OM_uint32, _: gss_cred_id_t,
                                _: *mut gss_name_t, _: *mut OM_uint32,
                                _: *mut gss_cred_usage_t, _: *mut gss_OID_set)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "594:1"]
        pub fn gss_inquire_context(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: *mut gss_name_t, _: *mut gss_name_t,
                                   _: *mut OM_uint32, _: *mut gss_OID,
                                   _: *mut OM_uint32, _: *mut libc::c_int,
                                   _: *mut libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "607:1"]
        pub fn gss_wrap_size_limit(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: libc::c_int, _: gss_qop_t, _: OM_uint32,
                                   _: *mut OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "633:1"]
        pub fn gss_add_cred(_: *mut OM_uint32, _: gss_cred_id_t,
                            _: gss_name_t, _: gss_OID, _: gss_cred_usage_t,
                            _: OM_uint32, _: OM_uint32, _: *mut gss_cred_id_t,
                            _: *mut gss_OID_set, _: *mut OM_uint32,
                            _: *mut OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "648:1"]
        pub fn gss_inquire_cred_by_mech(_: *mut OM_uint32, _: gss_cred_id_t,
                                        _: gss_OID, _: *mut gss_name_t,
                                        _: *mut OM_uint32, _: *mut OM_uint32,
                                        _: *mut gss_cred_usage_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "659:1"]
        pub fn gss_export_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "666:1"]
        pub fn gss_import_sec_context(_: *mut OM_uint32, _: gss_buffer_t,
                                      _: *mut gss_ctx_id_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "714:1"]
        pub fn gss_inquire_names_for_mech(_: *mut OM_uint32, _: gss_OID,
                                          _: *mut gss_OID_set) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "770:1"]
        pub fn gss_export_name(_: *mut OM_uint32, _: gss_name_t,
                               _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "777:1"]
        pub fn gss_duplicate_name(_: *mut OM_uint32, _: gss_name_t,
                                  _: *mut gss_name_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "796:1"]
        pub fn gss_pseudo_random(_: *mut OM_uint32, _: gss_ctx_id_t,
                                 _: libc::c_int, _: gss_buffer_t, _: ssize_t,
                                 _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "805:1"]
        pub fn gss_store_cred(_: *mut OM_uint32, _: gss_cred_id_t,
                              _: gss_cred_usage_t, _: gss_OID, _: OM_uint32,
                              _: OM_uint32, _: *mut gss_OID_set,
                              _: *mut gss_cred_usage_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "816:1"]
        pub fn gss_set_neg_mechs(_: *mut OM_uint32, _: gss_cred_id_t,
                                 _: gss_OID_set) -> OM_uint32;
        /*
 * RFC 5801
 */
        #[no_mangle]
        #[c2rust::src_loc = "907:1"]
        pub fn gss_inquire_saslname_for_mech(_: *mut OM_uint32, _: gss_OID,
                                             _: gss_buffer_t, _: gss_buffer_t,
                                             _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "916:1"]
        pub fn gss_inquire_mech_for_saslname(_: *mut OM_uint32,
                                             _: gss_buffer_t, _: *mut gss_OID)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:29"]
pub mod gssapi_ext_h {
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
    #[c2rust::src_loc = "134:1"]
    pub type gss_buffer_set_t = *mut gss_buffer_set_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "134:16"]
    pub struct gss_buffer_set_desc_struct {
        pub count: size_t,
        pub elements: *mut gss_buffer_desc,
    }
    use super::gssapi_h::{OM_uint32, gss_buffer_desc, gss_name_t,
                          gss_OID_desc, gss_const_OID, gss_buffer_desc_struct,
                          gss_buffer_t, gss_ctx_id_struct, gss_ctx_id_t,
                          gss_OID_desc_struct, gss_OID, gss_cred_id_t,
                          gss_qop_t, gss_OID_set_desc_struct, gss_OID_set,
                          gss_cred_usage_t, gss_const_buffer_t};
    use super::stddef_h::size_t;
    use super::mglueP_h::{gss_name_struct, gss_cred_id_struct};
    extern "C" {
        #[c2rust::src_loc = "488:16"]
        pub type gss_any;
        #[no_mangle]
        #[c2rust::src_loc = "57:1"]
        pub fn gss_localname(minor: *mut OM_uint32, name: gss_name_t,
                             mech_type: gss_const_OID,
                             localname: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "154:1"]
        pub fn gss_inquire_sec_context_by_oid(_: *mut OM_uint32,
                                              _: gss_ctx_id_t, _: gss_OID,
                                              _: *mut gss_buffer_set_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "160:1"]
        pub fn gss_inquire_cred_by_oid(_: *mut OM_uint32, _: gss_cred_id_t,
                                       _: gss_OID, _: *mut gss_buffer_set_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "166:1"]
        pub fn gss_set_sec_context_option(_: *mut OM_uint32,
                                          _: *mut gss_ctx_id_t, _: gss_OID,
                                          _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn gss_export_cred(_: *mut OM_uint32, _: gss_cred_id_t,
                               _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "180:1"]
        pub fn gss_import_cred(_: *mut OM_uint32, _: gss_buffer_t,
                               _: *mut gss_cred_id_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "197:1"]
        pub fn gssspi_mech_invoke(_: *mut OM_uint32, _: gss_OID, _: gss_OID,
                                  _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "207:1"]
        pub fn gss_wrap_aead(_: *mut OM_uint32, _: gss_ctx_id_t,
                             _: libc::c_int, _: gss_qop_t, _: gss_buffer_t,
                             _: gss_buffer_t, _: *mut libc::c_int,
                             _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "217:1"]
        pub fn gss_unwrap_aead(_: *mut OM_uint32, _: gss_ctx_id_t,
                               _: gss_buffer_t, _: gss_buffer_t,
                               _: gss_buffer_t, _: *mut libc::c_int,
                               _: *mut gss_qop_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "243:1"]
        pub fn gss_complete_auth_token(minor_status: *mut OM_uint32,
                                       context_handle: gss_ctx_id_t,
                                       input_message_buffer: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "307:1"]
        pub fn gss_wrap_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                            _: libc::c_int, _: gss_qop_t, _: *mut libc::c_int,
                            _: *mut gss_iov_buffer_desc, _: libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "328:1"]
        pub fn gss_unwrap_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                              _: *mut libc::c_int, _: *mut gss_qop_t,
                              _: *mut gss_iov_buffer_desc, _: libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "341:1"]
        pub fn gss_wrap_iov_length(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: libc::c_int, _: gss_qop_t,
                                   _: *mut libc::c_int,
                                   _: *mut gss_iov_buffer_desc,
                                   _: libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "403:1"]
        pub fn gss_acquire_cred_impersonate_name(_: *mut OM_uint32,
                                                 _: gss_cred_id_t,
                                                 _: gss_name_t, _: OM_uint32,
                                                 _: gss_OID_set,
                                                 _: gss_cred_usage_t,
                                                 _: *mut gss_cred_id_t,
                                                 _: *mut gss_OID_set,
                                                 _: *mut OM_uint32)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "415:1"]
        pub fn gss_add_cred_impersonate_name(_: *mut OM_uint32,
                                             _: gss_cred_id_t,
                                             _: gss_cred_id_t, _: gss_name_t,
                                             _: gss_OID, _: gss_cred_usage_t,
                                             _: OM_uint32, _: OM_uint32,
                                             _: *mut gss_cred_id_t,
                                             _: *mut gss_OID_set,
                                             _: *mut OM_uint32,
                                             _: *mut OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "436:1"]
        pub fn gss_display_name_ext(_: *mut OM_uint32, _: gss_name_t,
                                    _: gss_OID, _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "444:1"]
        pub fn gss_inquire_name(_: *mut OM_uint32, _: gss_name_t,
                                _: *mut libc::c_int, _: *mut gss_OID,
                                _: *mut gss_buffer_set_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn gss_get_name_attribute(_: *mut OM_uint32, _: gss_name_t,
                                      _: gss_buffer_t, _: *mut libc::c_int,
                                      _: *mut libc::c_int, _: gss_buffer_t,
                                      _: gss_buffer_t, _: *mut libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "465:1"]
        pub fn gss_set_name_attribute(_: *mut OM_uint32, _: gss_name_t,
                                      _: libc::c_int, _: gss_buffer_t,
                                      _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_name_attribute(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "481:1"]
        pub fn gss_export_name_composite(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "490:1"]
        pub fn gss_map_name_to_any(_: *mut OM_uint32, _: gss_name_t,
                                   _: libc::c_int, _: gss_buffer_t,
                                   _: *mut gss_any_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "499:1"]
        pub fn gss_release_any_name_mapping(_: *mut OM_uint32, _: gss_name_t,
                                            _: gss_buffer_t,
                                            _: *mut gss_any_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "545:1"]
        pub fn gss_acquire_cred_from(_: *mut OM_uint32, _: gss_name_t,
                                     _: OM_uint32, _: gss_OID_set,
                                     _: gss_cred_usage_t,
                                     _: gss_const_key_value_set_t,
                                     _: *mut gss_cred_id_t,
                                     _: *mut gss_OID_set, _: *mut OM_uint32)
         -> OM_uint32;
        /* acceptor_time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "572:1"]
        pub fn gss_store_cred_into(_: *mut OM_uint32, _: gss_cred_id_t,
                                   _: gss_cred_usage_t, _: gss_OID,
                                   _: OM_uint32, _: OM_uint32,
                                   _: gss_const_key_value_set_t,
                                   _: *mut gss_OID_set,
                                   _: *mut gss_cred_usage_t) -> OM_uint32;
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
        #[c2rust::src_loc = "598:1"]
        pub fn gssspi_query_meta_data(minor_status: *mut OM_uint32,
                                      mech_oid: gss_const_OID,
                                      cred_handle: gss_cred_id_t,
                                      context_handle: *mut gss_ctx_id_t,
                                      targ_name: gss_name_t,
                                      req_flags: OM_uint32,
                                      meta_data: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "608:1"]
        pub fn gssspi_exchange_meta_data(minor_status: *mut OM_uint32,
                                         mech_oid: gss_const_OID,
                                         cred_handle: gss_cred_id_t,
                                         context_handle: *mut gss_ctx_id_t,
                                         targ_name: gss_name_t,
                                         req_flags: OM_uint32,
                                         meta_data: gss_const_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "618:1"]
        pub fn gssspi_query_mechanism_info(minor_status: *mut OM_uint32,
                                           mech_oid: gss_const_OID,
                                           auth_scheme: *mut libc::c_uchar)
         -> OM_uint32;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:51"]
pub mod k5_plugin_h {
    use super::k5_err_h::errinfo;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2006 Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * This software is being provided to you, the LICENSEE, by the
 * Massachusetts Institute of Technology (M.I.T.) under the following
 * license.  By obtaining, using and/or copying this software, you agree
 * that you have read, understood, and will comply with these terms and
 * conditions:
 *
 * Export of this software from the United States of America may
 * require a specific license from the United States Government.
 * It is the responsibility of any person or organization contemplating
 * export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify and distribute
 * this software and its documentation for any purpose and without fee or
 * royalty is hereby granted, provided that you agree to comply with the
 * following copyright notice and statements, including the disclaimer, and
 * that the same appear on ALL copies of the software and documentation,
 * including modifications that you make for internal use or for
 * distribution:
 *
 * THIS SOFTWARE IS PROVIDED "AS IS", AND M.I.T. MAKES NO REPRESENTATIONS
 * OR WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
 * limitation, M.I.T. MAKES NO REPRESENTATIONS OR WARRANTIES OF
 * MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF
 * THE LICENSED SOFTWARE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY
 * PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
 *
 * The name of the Massachusetts Institute of Technology or M.I.T. may NOT
 * be used in advertising or publicity pertaining to distribution of the
 * software.  Title to copyright in this software and any associated
 * documentation shall at all times remain with M.I.T., and USER agrees to
 * preserve same.
 *
 * Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 */
        /* Just those definitions which are needed by util/support/plugins.c,
   which gets compiled before util/et is built, which happens before
   we can construct krb5.h, which is included by k5-int.h.

   So, no krb5 types.  */
        /*
 * Plugins normally export fixed symbol names, but when statically
 * linking plugins, we need a different symbol name for each plugin.
 * The first argument to PLUGIN_SYMBOL_NAME acts as the
 * differentiator, and is only used for static plugin linking.
 *
 * Although this macro (and thus this header file) are used in plugins
 * whose code lies inside the krb5 tree, plugins maintained separately
 * from the krb5 tree do not need it; they can just use the fixed
 * symbol name unconditionally.
 */
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn krb5int_open_plugin(_: *const libc::c_char,
                                   _: *mut *mut plugin_file_handle,
                                   _: *mut errinfo) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn krb5int_close_plugin(_: *mut plugin_file_handle);
        #[no_mangle]
        #[c2rust::src_loc = "98:1"]
        pub fn krb5int_get_plugin_func(_: *mut plugin_file_handle,
                                       _: *const libc::c_char,
                                       _:
                                           *mut Option<unsafe extern "C" fn()
                                                           -> ()>,
                                       _: *mut errinfo) -> libc::c_long;
    }
    /* K5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:54"]
pub mod krb5_h {
    #[c2rust::src_loc = "208:1"]
    pub type krb5_data = _krb5_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:16"]
    pub struct _krb5_data {
        pub magic: krb5_magic,
        pub length: libc::c_uint,
        pub data: *mut libc::c_char,
    }
    #[c2rust::src_loc = "206:1"]
    pub type krb5_magic = krb5_error_code;
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    use super::stdint_intn_h::int32_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:29"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:29"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:29"]
pub mod k5_thread_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-thread.h - Preliminary portable thread support */
/*
 * Copyright 2004,2005,2006,2007,2008 by the Massachusetts Institute of Technology.
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
    /* Interface (tentative):

     Mutex support:

     // Between these two, we should be able to do pure compile-time
     // and pure run-time initialization.
     //   POSIX:   partial initializer is PTHREAD_MUTEX_INITIALIZER,
     //            finish does nothing
     //   Windows: partial initializer is an invalid handle,
     //            finish does the real initialization work
     k5_mutex_t foo_mutex = K5_MUTEX_PARTIAL_INITIALIZER;
     int k5_mutex_finish_init(k5_mutex_t *);
     // for dynamic allocation
     int k5_mutex_init(k5_mutex_t *);
     // Must work for both kinds of alloc, even if it means adding flags.
     int k5_mutex_destroy(k5_mutex_t *);

     // As before.
     int k5_mutex_lock(k5_mutex_t *);
     int k5_mutex_unlock(k5_mutex_t *);

     In each library, one new function to finish the static mutex init,
     and any other library-wide initialization that might be desired.
     On POSIX, this function would be called via the second support
     function (see below).  On Windows, it would be called at library
     load time.  These functions, or functions they calls, should be the
     only places that k5_mutex_finish_init gets called.

     A second function or macro called at various possible "first" entry
     points which either calls pthread_once on the first function
     (POSIX), or checks some flag set by the first function (Windows),
     and possibly returns an error.  (In the non-threaded case, a simple
     flag can be used to avoid multiple invocations, and the mutexes
     don't need run-time initialization anyways.)

     A third function for library termination calls mutex_destroy on
     each mutex for the library.  This function would be called
     automatically at library unload time.  If it turns out to be needed
     at exit time for libraries that don't get unloaded, perhaps we
     should also use atexit().  Any static mutexes should be cleaned up
     with k5_mutex_destroy here.

     How does that second support function invoke the first support
     function only once?  Through something modelled on pthread_once
     that I haven't written up yet.  Probably:

     k5_once_t foo_once = K5_ONCE_INIT;
     k5_once(k5_once_t *, void (*)(void));

     For POSIX: Map onto pthread_once facility.
     For non-threaded case: A simple flag.
     For Windows: Not needed; library init code takes care of it.

     XXX: A general k5_once mechanism isn't possible for Windows,
     without faking it through named mutexes or mutexes initialized at
     startup.  I was only using it in one place outside these headers,
     so I'm dropping the general scheme.  Eventually the existing uses
     in k5-thread.h and k5-platform.h will be converted to pthread_once
     or static variables.


     Thread-specific data:

     // TSD keys are limited in number in gssapi/krb5/com_err; enumerate
     // them all.  This allows support code init to allocate the
     // necessary storage for pointers all at once, and avoids any
     // possible error in key creation.
     enum { ... } k5_key_t;
     // Register destructor function.  Called in library init code.
     int k5_key_register(k5_key_t, void (*destructor)(void *));
     // Returns NULL or data.
     void *k5_getspecific(k5_key_t);
     // Returns error if key out of bounds, or the pointer table can't
     // be allocated.  A call to k5_key_register must have happened first.
     // This may trigger the calling of pthread_setspecific on POSIX.
     int k5_setspecific(k5_key_t, void *);
     // Called in library termination code.
     // Trashes data in all threads, calling the registered destructor
     // (but calling it from the current thread).
     int k5_key_delete(k5_key_t);

     For the non-threaded version, the support code will have a static
     array indexed by k5_key_t values, and get/setspecific simply access
     the array elements.

     The TSD destructor table is global state, protected by a mutex if
     threads are enabled.


     Any actual external symbols will use the krb5int_ prefix.  The k5_
     names will be simple macros or inline functions to rename the
     external symbols, or slightly more complex ones to expand the
     implementation inline (e.g., map to POSIX versions and/or debug
     code using __FILE__ and the like).


     More to be added, perhaps.  */
    /* The mutex structure we use, k5_mutex_t, is defined to some
   OS-specific bits.  The use of multiple layers of typedefs are an
   artifact resulting from debugging code we once used, implemented as
   wrappers around the OS mutex scheme.

   The OS specific bits, in k5_os_mutex, break down into three primary
   implementations, POSIX threads, Windows threads, and no thread
   support.  However, the POSIX thread version is further subdivided:
   In one case, we can determine at run time whether the thread
   library is linked into the application, and use it only if it is
   present; in the other case, we cannot, and the thread library must
   be linked in always, but can be used unconditionally.  In the
   former case, the k5_os_mutex structure needs to hold both the POSIX
   and the non-threaded versions.

   The various k5_os_mutex_* operations are the OS-specific versions,
   applied to the OS-specific data, and k5_mutex_* uses k5_os_mutex_*
   to do the OS-specific parts of the work.  */
    /* Define the OS mutex bit.  */
    /* Empty inline functions avoid the "statement with no effect"
   warnings, and do better type-checking than functions that don't use
   their arguments.  */
    /* Values:
   2 - function has not been run
   3 - function has been run
   4 - function is being run -- deadlock detected */
    /* Weak reference support, etc.

   Linux: Stub mutex routines exist, but pthread_once does not.

   Solaris <10: In libc there's a pthread_once that doesn't seem to do
   anything.  Bleah.  But pthread_mutexattr_setrobust_np is defined
   only in libpthread.  However, some version of GNU libc (Red Hat's
   Fedora Core 5, reportedly) seems to have that function, but no
   declaration, so we'd have to declare it in order to test for its
   address.  We now have tests to see if pthread_once actually works,
   so stick with that for now.

   Solaris 10: The real thread support now lives in libc, and
   libpthread is just a filter object.  So we might as well use the
   real functions unconditionally.  Since we haven't got a test for
   this property yet, we use NO_WEAK_PTHREADS defined in aclocal.m4
   depending on the OS type.

   IRIX 6.5 stub pthread support in libc is really annoying.  The
   pthread_mutex_lock function returns ENOSYS for a program not linked
   against -lpthread.  No link-time failure, no weak symbols, etc.
   The C library doesn't provide pthread_once; we can use weak
   reference support for that.

   If weak references are not available, then for now, we assume that
   the pthread support routines will always be available -- either the
   real thing, or functional stubs that merely prohibit creating
   threads.

   If we find a platform with non-functional stubs and no weak
   references, we may have to resort to some hack like dlsym on the
   symbol tables of the current process.  */
    /* Can't rely on useful stubs -- see above regarding Solaris.  */
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "256:9"]
    pub struct k5_once_t {
        pub o: pthread_once_t,
        pub n: k5_os_nothread_once_t,
    }
    #[c2rust::src_loc = "188:1"]
    pub type k5_os_nothread_once_t = libc::c_uchar;
    #[inline]
    #[c2rust::src_loc = "379:1"]
    pub unsafe extern "C" fn k5_mutex_unlock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_unlock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_unlock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          388 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 35],
                                                    &[libc::c_char; 35]>(b"void k5_mutex_unlock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    #[inline]
    #[c2rust::src_loc = "367:1"]
    pub unsafe extern "C" fn k5_mutex_lock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_lock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_lock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          376 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 33],
                                                    &[libc::c_char; 33]>(b"void k5_mutex_lock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    #[inline]
    #[c2rust::src_loc = "360:1"]
    pub unsafe extern "C" fn k5_mutex_finish_init(mut m: *mut k5_mutex_t)
     -> libc::c_int {
        return 0 as libc::c_int;
    }
    use super::pthreadtypes_h::{pthread_mutex_t, pthread_once_t};
    use super::stdio_h::{fprintf, stderr};
    use super::string_h::strerror;
    use super::assert_h::__assert_fail;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "293:1"]
        pub fn k5_os_mutex_unlock(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "292:1"]
        pub fn k5_os_mutex_lock(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "262:1"]
        pub fn k5_once(once: *mut k5_once_t,
                       fn_0: Option<unsafe extern "C" fn() -> ()>)
         -> libc::c_int;
    }
    /* multiple inclusion? */
    /* In time, many of the definitions above should move into the support
   library, and this file should be greatly simplified.  For type
   definitions, that'll take some work, since other data structures
   incorporate mutexes directly, and our mutex type is dependent on
   configuration options and system attributes.  For most functions,
   though, it should be relatively easy.

   For now, plugins should use the exported functions, and not the
   above macros, and use krb5int_mutex_alloc for allocations.  */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:51"]
pub mod k5_err_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-err.h */
/*
 * Copyright 2006, 2007 Massachusetts Institute of Technology.
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
 *
 * Error-message handling
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct errinfo {
        pub code: libc::c_long,
        pub msg: *mut libc::c_char,
    }
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "66:1"]
        pub fn k5_clear_error(ep: *mut errinfo);
    }
    /* K5_ERR_H */
}
#[c2rust::header_src = "/usr/include/glob.h:45"]
pub mod glob_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:9"]
    pub struct glob_t {
        pub gl_pathc: __size_t,
        pub gl_pathv: *mut *mut libc::c_char,
        pub gl_offs: __size_t,
        pub gl_flags: libc::c_int,
        pub gl_closedir: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                    -> ()>,
        pub gl_readdir: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                   -> *mut dirent>,
        pub gl_opendir: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> *mut libc::c_void>,
        pub gl_lstat: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *mut stat)
                                 -> libc::c_int>,
        pub gl_stat: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: *mut stat)
                                -> libc::c_int>,
    }
    #[c2rust::src_loc = "27:1"]
    pub type __size_t = libc::c_ulong;
    use super::stat_h::stat;
    extern "C" {
        #[c2rust::src_loc = "93:12"]
        pub type dirent;
        #[no_mangle]
        #[c2rust::src_loc = "146:1"]
        pub fn glob(__pattern: *const libc::c_char, __flags: libc::c_int,
                    __errfunc:
                        Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: libc::c_int)
                                   -> libc::c_int>, __pglob: *mut glob_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "151:1"]
        pub fn globfree(__pglob: *mut glob_t);
    }
}
#[c2rust::header_src = "/usr/include/bits/stat.h:29"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: libc::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::types_h::{__dev_t, __ino_t, __nlink_t, __mode_t, __uid_t,
                         __gid_t, __off_t, __blksize_t, __blkcnt_t,
                         __syscall_slong_t};
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src = "/usr/include/ctype.h:42"]
pub mod ctype_h {
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:29"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "237:9"]
    pub struct k5_init_t {
        pub once: k5_once_t,
        pub error: libc::c_int,
        pub did_run: libc::c_int,
        pub fn_0: Option<unsafe extern "C" fn() -> ()>,
    }
    use super::k5_thread_h::k5_once_t;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:29"]
pub mod com_err_h {
    /*
 * Copyright 1988, Student Information Processing Board of the
 * Massachusetts Institute of Technology.
 *
 * Copyright 1995 by Cygnus Support.
 *
 * For copyright and distribution info, see the documentation supplied
 * with this package.
 */
    /* Header file for common error description library. */
    #[c2rust::src_loc = "26:1"]
    pub type errcode_t = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:8"]
    pub struct error_table {
        pub msgs: *const *const libc::c_char,
        pub base: libc::c_long,
        pub n_msgs: libc::c_uint,
    }
    extern "C" {
        /* Public interfaces */
        /*@observer@*//*@dependent@*/
        /*@modifies internalState@*/
        #[no_mangle]
        #[c2rust::src_loc = "57:1"]
        pub fn add_error_table(_: *const error_table) -> errcode_t;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/usr/include/string.h:29"]
pub mod string_h {
    use super::stddef_h::size_t;
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
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_alloc.h:29"]
pub mod gssapi_alloc_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* To the extent possible under law, Painless Security, LLC has waived
 * all copyright and related or neighboring rights to GSS-API Memory
 * Management Header. This work is published from: United States.
 */
    /* not _WIN32 or DEBUG_GSSALLOC */
    /* Normal Unix case, just use free/malloc/calloc/realloc. */
    #[inline]
    #[c2rust::src_loc = "93:1"]
    pub unsafe extern "C" fn gssalloc_free(mut value: *mut libc::c_void) {
        free(value);
    }
    use super::stdlib_h::free;
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
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "639:1"]
        pub fn secure_getenv(__name: *const libc::c_char)
         -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:29"]
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
#[c2rust::header_src = "/usr/include/stdio.h:29"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int,
                     __stream: *mut FILE) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:29"]
pub mod gssapiP_generic_h {
    use super::gssapi_h::{OM_uint32, gss_OID_set_desc, gss_OID_set, gss_OID,
                          gss_buffer_desc_struct, gss_buffer_t, gss_OID_desc};
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
        /*
 * Transfer contents of a k5buf to a gss_buffer and invalidate the source
 * On unix, this is a simple pointer copy
 * On windows, memory is reallocated and copied.
 */
        /*minor_status*/
        /*buffer_set*/
        /*minor_status*/
        /*member_buffer*/
        /*buffer_set*/
        /*minor_status*/
        /*buffer_set*/
        #[no_mangle]
        #[c2rust::src_loc = "316:1"]
        pub fn generic_gss_copy_oid_set(_: *mut OM_uint32,
                                        _: *const gss_OID_set_desc,
                                        _: *mut gss_OID_set) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "201:1"]
        pub fn generic_gss_release_oid(_: *mut OM_uint32, _: *mut gss_OID)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "236:1"]
        pub fn generic_gss_str_to_oid(_: *mut OM_uint32, _: gss_buffer_t,
                                      _: *mut gss_OID) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "258:1"]
        pub fn gssint_mecherrmap_init() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn gssint_mecherrmap_map(minor: OM_uint32,
                                     oid: *const gss_OID_desc) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn generic_gss_release_oid_set(_: *mut OM_uint32,
                                           _: *mut gss_OID_set) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "212:1"]
        pub fn generic_gss_create_empty_oid_set(_: *mut OM_uint32,
                                                _: *mut gss_OID_set)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "217:1"]
        pub fn generic_gss_add_oid_set_member(_: *mut OM_uint32,
                                              _: *const gss_OID_desc,
                                              _: *mut gss_OID_set)
         -> OM_uint32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src = "/usr/include/time.h:29"]
pub mod time_h {
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn time(__timer: *mut time_t) -> time_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapi_err_generic.h:29"]
pub mod gssapi_err_generic_h {
    use super::com_err_h::error_table;
    extern "C" {
        /*
 * et-h-gssapi_err_generic.h:
 * This file is automatically generated; please do not edit it.
 */
        #[no_mangle]
        #[c2rust::src_loc = "30:33"]
        pub static et_ggss_error_table: error_table;
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/sys/stat.h:34"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "259:1"]
        pub fn lstat(__file: *const libc::c_char, __buf: *mut stat)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "205:1"]
        pub fn stat(__file: *const libc::c_char, __buf: *mut stat)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:54"]
pub mod k5_int_h {
    #[inline]
    #[c2rust::src_loc = "656:1"]
    pub unsafe extern "C" fn zapfree(mut ptr: *mut libc::c_void,
                                     mut len: size_t) {
        if !ptr.is_null() { explicit_bzero(ptr, len); free(ptr); };
    }
    use super::stddef_h::size_t;
    use super::string_h::explicit_bzero;
    use super::stdlib_h::free;
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/krb5/gssapiP_krb5.h:54"]
pub mod gssapiP_krb5_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "1167:1"]
        pub fn gss_krb5int_lib_init() -> libc::c_int;
    }
    /* _GSSAPIP_KRB5_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/spnego/gssapiP_spnego.h:55"]
pub mod gssapiP_spnego_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "415:1"]
        pub fn gss_spnegoint_lib_init() -> libc::c_int;
    }
    /* _GSSAPIP_SPNEGO_H_ */
}
pub use self::types_h::{__int32_t, __uint32_t, __dev_t, __uid_t, __gid_t,
                        __ino_t, __mode_t, __nlink_t, __off_t, __off64_t,
                        __time_t, __blksize_t, __blkcnt_t, __ssize_t,
                        __syscall_slong_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::stddef_h::size_t;
pub use self::sys_types_h::ssize_t;
pub use self::time_t_h::time_t;
pub use self::struct_timespec_h::timespec;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::{pthread_once_t, pthread_mutex_t};
pub use self::mglueP_h::{gss_name_struct, gss_cred_id_struct, gss_mech_info,
                         gss_mech_config, gss_mechanism, gss_config};
pub use self::gssapi_h::{gss_name_t, gss_OID, gss_OID_desc_struct, OM_uint32,
                         gss_uint32, gss_buffer_t, gss_buffer_desc_struct,
                         gss_cred_id_t, gss_ctx_id_t, gss_OID_desc,
                         gss_OID_set_desc_struct, gss_OID_set_desc,
                         gss_OID_set, gss_buffer_desc,
                         gss_channel_bindings_struct, gss_channel_bindings_t,
                         gss_qop_t, gss_cred_usage_t, gss_const_OID,
                         gss_const_buffer_t, gss_ctx_id_struct,
                         gss_acquire_cred, gss_release_cred,
                         gss_init_sec_context, gss_accept_sec_context,
                         gss_process_context_token, gss_delete_sec_context,
                         gss_context_time, gss_get_mic, gss_verify_mic,
                         gss_wrap, gss_unwrap, gss_display_status,
                         gss_release_oid_set, GSS_C_MA_DEPRECATED,
                         gss_test_oid_set_member, gss_inquire_attrs_for_mech,
                         gss_compare_name, gss_display_name, gss_import_name,
                         gss_release_name, gss_inquire_cred,
                         gss_inquire_context, gss_wrap_size_limit,
                         gss_add_cred, gss_inquire_cred_by_mech,
                         gss_export_sec_context, gss_import_sec_context,
                         gss_inquire_names_for_mech, gss_export_name,
                         gss_duplicate_name, gss_pseudo_random,
                         gss_store_cred, gss_set_neg_mechs,
                         gss_inquire_saslname_for_mech,
                         gss_inquire_mech_for_saslname};
pub use self::gssapi_ext_h::{gss_iov_buffer_desc, gss_iov_buffer_desc_struct,
                             gss_const_key_value_set_t,
                             gss_key_value_set_desc, gss_key_value_set_struct,
                             gss_key_value_element_desc,
                             gss_key_value_element_struct, gss_any_t,
                             gss_buffer_set_t, gss_buffer_set_desc_struct,
                             gss_any, gss_localname,
                             gss_inquire_sec_context_by_oid,
                             gss_inquire_cred_by_oid,
                             gss_set_sec_context_option, gss_export_cred,
                             gss_import_cred, gssspi_mech_invoke,
                             gss_wrap_aead, gss_unwrap_aead,
                             gss_complete_auth_token, gss_wrap_iov,
                             gss_unwrap_iov, gss_wrap_iov_length,
                             gss_acquire_cred_impersonate_name,
                             gss_add_cred_impersonate_name,
                             gss_display_name_ext, gss_inquire_name,
                             gss_get_name_attribute, gss_set_name_attribute,
                             gss_delete_name_attribute,
                             gss_export_name_composite, gss_map_name_to_any,
                             gss_release_any_name_mapping,
                             gss_acquire_cred_from, gss_store_cred_into,
                             gssspi_query_meta_data,
                             gssspi_exchange_meta_data,
                             gssspi_query_mechanism_info};
use self::k5_plugin_h::{plugin_file_handle, krb5int_open_plugin,
                        krb5int_close_plugin, krb5int_get_plugin_func};
pub use self::krb5_h::{krb5_data, _krb5_data, krb5_magic, krb5_error_code,
                       krb5_int32};
pub use self::FILE_h::FILE;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::k5_thread_h::{k5_mutex_t, k5_os_mutex, k5_once_t,
                            k5_os_nothread_once_t, k5_mutex_unlock,
                            k5_mutex_lock, k5_mutex_finish_init,
                            k5_os_mutex_unlock, k5_os_mutex_lock, k5_once};
pub use self::k5_err_h::{errinfo, k5_clear_error};
pub use self::glob_h::{glob_t, __size_t, dirent, glob, globfree};
pub use self::stat_h::stat;
pub use self::ctype_h::{_ISspace, C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISxdigit, _ISdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc};
pub use self::k5_platform_h::k5_init_t;
pub use self::com_err_h::{errcode_t, error_table, add_error_table};
use self::string_h::{memcpy, memset, memcmp, strerror, strcmp, strdup, strlen,
                     explicit_bzero};
pub use self::gssapi_alloc_h::gssalloc_free;
use self::stdlib_h::{free, malloc, calloc, secure_getenv};
use self::assert_h::__assert_fail;
use self::stdio_h::{stderr, fprintf, fclose, snprintf, fgets, fopen};
use self::gssapiP_generic_h::{generic_gss_copy_oid_set,
                              generic_gss_release_oid, generic_gss_str_to_oid,
                              gssint_mecherrmap_init, gssint_mecherrmap_map,
                              generic_gss_release_oid_set,
                              generic_gss_create_empty_oid_set,
                              generic_gss_add_oid_set_member};
use self::time_h::time;
use self::gssapi_err_generic_h::et_ggss_error_table;
use self::sys_stat_h::{lstat, stat};
pub use self::k5_int_h::zapfree;
use self::gssapiP_krb5_h::gss_krb5int_lib_init;
use self::gssapiP_spnego_h::gss_spnegoint_lib_init;
/*
 * list of mechanism libraries and their entry points.
 * the list also maintains state of the mech libraries (loaded or not).
 */
#[c2rust::src_loc = "93:22"]
static mut g_mechList: gss_mech_info =
    0 as *const gss_mech_config as gss_mech_info;
#[c2rust::src_loc = "94:22"]
static mut g_mechListTail: gss_mech_info =
    0 as *const gss_mech_config as gss_mech_info;
#[c2rust::src_loc = "95:19"]
static mut g_mechListLock: k5_mutex_t =
    pthread_mutex_t{__data:
                        {
                            let mut init =
                                __pthread_mutex_s{__lock: 0 as libc::c_int,
                                                  __count:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __owner: 0 as libc::c_int,
                                                  __nusers:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __kind: 0 as libc::c_int,
                                                  __spins:
                                                      0 as libc::c_int as
                                                          libc::c_short,
                                                  __elision:
                                                      0 as libc::c_int as
                                                          libc::c_short,
                                                  __list:
                                                      {
                                                          let mut init =
                                                              __pthread_internal_list{__prev:
                                                                                          0
                                                                                              as
                                                                                              *const __pthread_internal_list
                                                                                              as
                                                                                              *mut __pthread_internal_list,
                                                                                      __next:
                                                                                          0
                                                                                              as
                                                                                              *const __pthread_internal_list
                                                                                              as
                                                                                              *mut __pthread_internal_list,};
                                                          init
                                                      },};
                            init
                        },};
#[c2rust::src_loc = "96:15"]
static mut g_confFileModTime: time_t = 0 as libc::c_int as time_t;
#[c2rust::src_loc = "97:15"]
static mut g_confLastCall: time_t = 0 as libc::c_int as time_t;
#[c2rust::src_loc = "99:25"]
static mut g_mechSet: gss_OID_set_desc =
    {
        let mut init =
            gss_OID_set_desc_struct{count: 0 as libc::c_int as size_t,
                                    elements:
                                        0 as *const gss_OID_desc_struct as
                                            gss_OID,};
        init
    };
#[c2rust::src_loc = "100:19"]
static mut g_mechSetLock: k5_mutex_t =
    pthread_mutex_t{__data:
                        {
                            let mut init =
                                __pthread_mutex_s{__lock: 0 as libc::c_int,
                                                  __count:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __owner: 0 as libc::c_int,
                                                  __nusers:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __kind: 0 as libc::c_int,
                                                  __spins:
                                                      0 as libc::c_int as
                                                          libc::c_short,
                                                  __elision:
                                                      0 as libc::c_int as
                                                          libc::c_short,
                                                  __list:
                                                      {
                                                          let mut init =
                                                              __pthread_internal_list{__prev:
                                                                                          0
                                                                                              as
                                                                                              *const __pthread_internal_list
                                                                                              as
                                                                                              *mut __pthread_internal_list,
                                                                                      __next:
                                                                                          0
                                                                                              as
                                                                                              *const __pthread_internal_list
                                                                                              as
                                                                                              *mut __pthread_internal_list,};
                                                          init
                                                      },};
                            init
                        },};
#[c2rust::src_loc = "102:1"]
static mut gssint_mechglue_init__once: k5_init_t =
    unsafe {
        {
            let mut init =
                k5_init_t{once:
                              {
                                  let mut init =
                                      k5_once_t{o: 0 as libc::c_int,
                                                n:
                                                    2 as libc::c_int as
                                                        k5_os_nothread_once_t,};
                                  init
                              },
                          error: 0 as libc::c_int,
                          did_run: 0 as libc::c_int,
                          fn_0:
                              Some(gssint_mechglue_init__aux as
                                       unsafe extern "C" fn() -> ()),};
            init
        }
    };
#[c2rust::src_loc = "102:1"]
unsafe extern "C" fn gssint_mechglue_init__aux() {
    gssint_mechglue_init__once.did_run = 1 as libc::c_int;
    gssint_mechglue_init__once.error = gssint_mechglue_init();
}
#[c2rust::src_loc = "105:1"]
unsafe extern "C" fn gssint_mechglue_init() -> libc::c_int {
    let mut err: libc::c_int = 0;
    add_error_table(&et_ggss_error_table);
    err = k5_mutex_finish_init(&mut g_mechSetLock);
    err = k5_mutex_finish_init(&mut g_mechListLock);
    err = gss_krb5int_lib_init();
    err = gss_spnegoint_lib_init();
    err = gssint_mecherrmap_init();
    return err;
}
#[no_mangle]
#[c2rust::src_loc = "153:1"]
pub unsafe extern "C" fn gssint_mechglue_initialize_library() -> libc::c_int {
    return ({
                let mut k5int_i: *mut k5_init_t =
                    &mut gssint_mechglue_init__once;
                let mut k5int_err: libc::c_int =
                    k5_once(&mut (*k5int_i).once, (*k5int_i).fn_0);
                if k5int_err != 0 {
                    k5int_err
                } else {
                    if (*k5int_i).did_run != 0 as libc::c_int {
                    } else {
                        __assert_fail(b"k5int_i->did_run != 0\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"g_initialize.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      156 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 45],
                                                                &[libc::c_char; 45]>(b"int gssint_mechglue_initialize_library(void)\x00")).as_ptr());
                    }
                    (*k5int_i).error
                }
            });
}
/*
 * function used to reclaim the memory used by a gss_OID structure.
 * This routine requires direct access to the mechList.
 */
#[no_mangle]
#[c2rust::src_loc = "163:1"]
pub unsafe extern "C" fn gss_release_oid(mut minor_status: *mut OM_uint32,
                                         mut oid: *mut gss_OID) -> OM_uint32 {
    let mut major: OM_uint32 = 0; /* while */
    let mut aMech: gss_mech_info = 0 as *mut gss_mech_config;
    if !minor_status.is_null() {
        *minor_status = 0 as libc::c_int as OM_uint32
    }
    if minor_status.is_null() || oid.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    *minor_status = gssint_mechglue_initialize_library() as OM_uint32;
    if *minor_status != 0 as libc::c_int as libc::c_uint {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    k5_mutex_lock(&mut g_mechListLock);
    aMech = g_mechList;
    while !aMech.is_null() {
        /*
		 * look through the loaded mechanism libraries for
		 * gss_internal_release_oid until one returns success.
		 * gss_internal_release_oid will only return success when
		 * the OID was recognized as an internal mechanism OID. if no
		 * mechanisms recognize the OID, then call the generic version.
		 */
        if !(*aMech).mech.is_null() &&
               (*(*aMech).mech).gss_internal_release_oid.is_some() {
            major =
                (*(*aMech).mech).gss_internal_release_oid.expect("non-null function pointer")(minor_status,
                                                                                              oid);
            if major == 0 as libc::c_int as libc::c_uint {
                k5_mutex_unlock(&mut g_mechListLock);
                return 0 as libc::c_int as OM_uint32
            }
            *minor_status =
                gssint_mecherrmap_map(*minor_status,
                                      &mut (*(*aMech).mech).mech_type)
        }
        aMech = (*aMech).next
    }
    k5_mutex_unlock(&mut g_mechListLock);
    return generic_gss_release_oid(minor_status, oid);
}
/* gss_release_oid */
/*
 * Wrapper around inquire_attrs_for_mech to determine whether a mechanism has
 * the deprecated attribute.  Must be called without g_mechSetLock since it
 * will call into the mechglue.
 */
#[c2rust::src_loc = "213:1"]
unsafe extern "C" fn is_deprecated(mut element: gss_OID) -> libc::c_int {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut mech_attrs: gss_OID_set = 0 as gss_OID_set;
    let mut deprecated: libc::c_int = 0 as libc::c_int;
    major =
        gss_inquire_attrs_for_mech(&mut minor, element as gss_const_OID,
                                   &mut mech_attrs, 0 as *mut gss_OID_set);
    if major == 0 as libc::c_int as libc::c_uint {
        gss_test_oid_set_member(&mut minor, GSS_C_MA_DEPRECATED as gss_OID,
                                mech_attrs, &mut deprecated);
    }
    if !mech_attrs.is_null() {
        gss_release_oid_set(&mut minor, &mut mech_attrs);
    }
    return deprecated;
}
/*
 * Removes mechs with the deprecated attribute from an OID set.  Must be
 * called without g_mechSetLock held since it calls into the mechglue.
 */
#[c2rust::src_loc = "236:1"]
unsafe extern "C" fn prune_deprecated(mut mech_set: gss_OID_set) {
    let mut i: OM_uint32 = 0;
    let mut j: OM_uint32 = 0;
    j = 0 as libc::c_int as OM_uint32;
    i = 0 as libc::c_int as OM_uint32;
    while (i as libc::c_ulong) < (*mech_set).count {
        if is_deprecated(&mut *(*mech_set).elements.offset(i as isize)) == 0 {
            let fresh0 = j;
            j = j.wrapping_add(1);
            *(*mech_set).elements.offset(fresh0 as isize) =
                *(*mech_set).elements.offset(i as isize)
        } else {
            gssalloc_free((*(*mech_set).elements.offset(i as
                                                            isize)).elements);
        }
        i = i.wrapping_add(1)
    }
    (*mech_set).count = j as size_t;
}
/*
 * this function will return an oid set indicating available mechanisms.
 * The set returned is based on configuration file entries and
 * NOT on the loaded mechanisms.  This function does not check if any
 * of these can actually be loaded.
 * Deprecated mechanisms will not be returned.
 * This routine needs direct access to the mechanism list.
 * To avoid reading the configuration file each call, we will save a
 * a mech oid set, and only update it once the file has changed.
 */
#[no_mangle]
#[c2rust::src_loc = "261:1"]
pub unsafe extern "C" fn gss_indicate_mechs(mut minorStatus: *mut OM_uint32,
                                            mut mechSet_out: *mut gss_OID_set)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    /* Initialize outputs. */
    if !minorStatus.is_null() { *minorStatus = 0 as libc::c_int as OM_uint32 }
    if !mechSet_out.is_null() { *mechSet_out = 0 as gss_OID_set }
    /* Validate arguments. */
    if minorStatus.is_null() || mechSet_out.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    *minorStatus = gssint_mechglue_initialize_library() as OM_uint32;
    if *minorStatus != 0 as libc::c_int as libc::c_uint {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if build_mechSet() != 0 {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /*
	 * need to lock the g_mechSet in case someone tries to update it while
	 * I'm copying it.
	 */
    k5_mutex_lock(&mut g_mechSetLock);
    status =
        generic_gss_copy_oid_set(minorStatus, &mut g_mechSet, mechSet_out);
    k5_mutex_unlock(&mut g_mechSetLock);
    if !(*mechSet_out).is_null() { prune_deprecated(*mechSet_out); }
    return status;
}
/* gss_indicate_mechs */
/* Call with g_mechSetLock held, or during final cleanup.  */
#[c2rust::src_loc = "303:1"]
unsafe extern "C" fn free_mechSet() {
    let mut i: libc::c_uint = 0;
    if g_mechSet.count != 0 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < g_mechSet.count {
            free((*g_mechSet.elements.offset(i as isize)).elements);
            i = i.wrapping_add(1)
        }
        free(g_mechSet.elements as *mut libc::c_void);
        g_mechSet.elements = 0 as gss_OID;
        g_mechSet.count = 0 as libc::c_int as size_t
    };
}
#[c2rust::src_loc = "317:1"]
unsafe extern "C" fn build_mechSet() -> OM_uint32 {
    let mut mList: gss_mech_info = 0 as *mut gss_mech_config;
    let mut i: size_t = 0;
    let mut count: size_t = 0;
    let mut curItem: gss_OID = 0 as *mut gss_OID_desc_struct;
    /*
	 * lock the mutex since we will be updating
	 * the mechList structure
	 * we need to keep the lock while we build the mechanism list
	 * since we are accessing parts of the mechList which could be
	 * modified.
	 */
    k5_mutex_lock(&mut g_mechListLock);
    updateMechList();
    /*
	 * we need to lock the mech set so that no one else will
	 * try to read it as we are re-creating it
	 */
    k5_mutex_lock(&mut g_mechSetLock);
    /* if the oid list already exists we must free it first */
    free_mechSet();
    /* determine how many elements to have in the list */
    mList = g_mechList;
    count = 0 as libc::c_int as size_t;
    while !mList.is_null() {
        count = count.wrapping_add(1);
        mList = (*mList).next
    }
    /* this should always be true, but.... */
    if count > 0 as libc::c_int as libc::c_ulong {
        g_mechSet.elements =
            calloc(count,
                   ::std::mem::size_of::<gss_OID_desc>() as libc::c_ulong) as
                gss_OID;
        if g_mechSet.elements.is_null() {
            k5_mutex_unlock(&mut g_mechSetLock);
            k5_mutex_unlock(&mut g_mechListLock);
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        memset(g_mechSet.elements as *mut libc::c_void, 0 as libc::c_int,
               count.wrapping_mul(::std::mem::size_of::<gss_OID_desc>() as
                                      libc::c_ulong));
        /* now copy each oid element */
        count = 0 as libc::c_int as size_t;
        mList = g_mechList;
        while !mList.is_null() {
            /* Don't expose interposer mechanisms. */
            if !((*mList).is_interposer != 0) {
                curItem =
                    &mut *g_mechSet.elements.offset(count as isize) as
                        *mut gss_OID_desc_struct;
                (*curItem).elements =
                    malloc((*(*mList).mech_type).length as libc::c_ulong);
                if (*curItem).elements.is_null() {
                    /*
				 * this is nasty - we must delete the
				 * part of the array already copied
				 */
                    i = 0 as libc::c_int as size_t;
                    while i < count {
                        free((*g_mechSet.elements.offset(i as
                                                             isize)).elements);
                        i = i.wrapping_add(1)
                    }
                    free(g_mechSet.elements as *mut libc::c_void);
                    g_mechSet.count = 0 as libc::c_int as size_t;
                    g_mechSet.elements = 0 as gss_OID;
                    k5_mutex_unlock(&mut g_mechSetLock);
                    k5_mutex_unlock(&mut g_mechListLock);
                    return (13 as libc::c_ulong as OM_uint32) <<
                               16 as libc::c_int
                }
                memcpy((*curItem).elements, (*(*mList).mech_type).elements,
                       (*(*mList).mech_type).length as libc::c_ulong);
                (*curItem).length = (*(*mList).mech_type).length;
                count = count.wrapping_add(1)
            }
            mList = (*mList).next
        }
        g_mechSet.count = count
    }
    k5_mutex_unlock(&mut g_mechSetLock);
    k5_mutex_unlock(&mut g_mechListLock);
    return 0 as libc::c_int as OM_uint32;
}
/*
 * this function has been added for use by modules that need to
 * know what (if any) optional parameters are supplied in the
 * config file (MECH_CONF).
 * It will return the option string for a specified mechanism.
 * caller is responsible for freeing the memory
 */
#[no_mangle]
#[c2rust::src_loc = "411:1"]
pub unsafe extern "C" fn gssint_get_modOptions(oid: gss_OID)
 -> *mut libc::c_char {
    let mut aMech: gss_mech_info = 0 as *mut gss_mech_config;
    let mut modOptions: *mut libc::c_char = 0 as *mut libc::c_char;
    if gssint_mechglue_initialize_library() != 0 as libc::c_int {
        return 0 as *mut libc::c_char
    }
    /* make sure we have fresh data */
    k5_mutex_lock(&mut g_mechListLock);
    updateMechList();
    aMech = searchMechList(oid as gss_const_OID);
    if aMech.is_null() || (*aMech).optionStr.is_null() {
        k5_mutex_unlock(&mut g_mechListLock);
        return 0 as *mut libc::c_char
    }
    if !(*aMech).optionStr.is_null() {
        modOptions = strdup((*aMech).optionStr)
    }
    k5_mutex_unlock(&mut g_mechListLock);
    return modOptions;
}
/* gssint_get_modOptions */
/* Return the mtime of filename or its eventual symlink target (if it is a
 * symlink), whichever is larger.  Return (time_t)-1 if lstat or stat fails. */
#[c2rust::src_loc = "440:1"]
unsafe extern "C" fn check_link_mtime(mut filename: *const libc::c_char,
                                      mut mtime_out: *mut time_t) -> time_t {
    let mut st1: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut st2: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    if lstat(filename, &mut st1) != 0 as libc::c_int {
        return -(1 as libc::c_int) as time_t
    }
    if !(st1.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
             0o120000 as libc::c_int as libc::c_uint) {
        return st1.st_mtim.tv_sec
    }
    if stat(filename, &mut st2) != 0 as libc::c_int {
        return -(1 as libc::c_int) as time_t
    }
    return if st1.st_mtim.tv_sec > st2.st_mtim.tv_sec {
               st1.st_mtim.tv_sec
           } else { st2.st_mtim.tv_sec };
}
/* Load pathname if it is newer than last.  Update *highest to the maximum of
 * its current value and pathname's mod time. */
#[c2rust::src_loc = "456:1"]
unsafe extern "C" fn load_if_changed(mut pathname: *const libc::c_char,
                                     mut last: time_t,
                                     mut highest: *mut time_t) {
    let mut mtime: time_t = 0;
    mtime = check_link_mtime(pathname, &mut mtime);
    if mtime == -(1 as libc::c_int) as time_t { return }
    if mtime > *highest { *highest = mtime }
    if mtime > last { loadConfigFile(pathname); };
}
/* Try to load any config files which have changed since the last call.  Config
 * files are MECH_CONF and any files matching MECH_CONF_PATTERN. */
#[c2rust::src_loc = "473:1"]
unsafe extern "C" fn loadConfigFiles() {
    let mut globbuf: glob_t =
        glob_t{gl_pathc: 0,
               gl_pathv: 0 as *mut *mut libc::c_char,
               gl_offs: 0,
               gl_flags: 0,
               gl_closedir: None,
               gl_readdir: None,
               gl_opendir: None,
               gl_lstat: None,
               gl_stat: None,};
    let mut highest: time_t = 0 as libc::c_int as time_t;
    let mut now: time_t = 0;
    let mut path: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    /* Don't glob and stat more than once per second. */
    if time(&mut now) == -(1 as libc::c_int) as time_t ||
           now == g_confLastCall {
        return
    }
    g_confLastCall = now;
    val =
        secure_getenv(b"GSS_MECH_CONFIG\x00" as *const u8 as
                          *const libc::c_char);
    if !val.is_null() {
        load_if_changed(val, g_confFileModTime, &mut g_confFileModTime);
        return
    }
    load_if_changed(b"/usr/local/etc/gss/mech\x00" as *const u8 as
                        *const libc::c_char, g_confFileModTime, &mut highest);
    memset(&mut globbuf as *mut glob_t as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<glob_t>() as libc::c_ulong);
    if glob(b"/usr/local/etc/gss/mech.d/*.conf\x00" as *const u8 as
                *const libc::c_char, 0 as libc::c_int, None, &mut globbuf) ==
           0 as libc::c_int {
        path = globbuf.gl_pathv;
        while !(*path).is_null() {
            load_if_changed(*path, g_confFileModTime, &mut highest);
            path = path.offset(1)
        }
    }
    globfree(&mut globbuf);
    g_confFileModTime = highest;
}
/*
 * determines if the mechList needs to be updated from file
 * and performs the update.
 * this functions must be called with a lock of g_mechListLock
 */
#[c2rust::src_loc = "510:1"]
unsafe extern "C" fn updateMechList() {
    let mut minfo: gss_mech_info = 0 as *mut gss_mech_config;
    /* _WIN32 */
    loadConfigFiles();
    /* !_WIN32 */
    /* Load any unloaded interposer mechanisms immediately, to make sure we
	 * interpose other mechanisms before they are used. */
    minfo = g_mechList;
    while !minfo.is_null() {
        if (*minfo).is_interposer != 0 && (*minfo).mech.is_null() {
            loadInterMech(minfo);
        }
        minfo = (*minfo).next
    };
}
/* updateMechList */
/* Update the mech list from system configuration if we have never done so.
 * Must be invoked with the g_mechListLock mutex held. */
#[c2rust::src_loc = "536:1"]
unsafe extern "C" fn initMechList() {
    static mut lazy_init: libc::c_int = 0 as libc::c_int;
    if lazy_init == 0 as libc::c_int {
        updateMechList();
        lazy_init = 1 as libc::c_int
    };
}
#[c2rust::src_loc = "547:1"]
unsafe extern "C" fn releaseMechInfo(mut pCf: *mut gss_mech_info) {
    let mut cf: gss_mech_info = 0 as *mut gss_mech_config;
    let mut minor_status: OM_uint32 = 0;
    if (*pCf).is_null() { return }
    cf = *pCf;
    if !(*cf).kmodName.is_null() {
        free((*cf).kmodName as *mut libc::c_void);
    }
    if !(*cf).uLibName.is_null() {
        free((*cf).uLibName as *mut libc::c_void);
    }
    if !(*cf).mechNameStr.is_null() {
        free((*cf).mechNameStr as *mut libc::c_void);
    }
    if !(*cf).optionStr.is_null() {
        free((*cf).optionStr as *mut libc::c_void);
    }
    if !(*cf).mech_type.is_null() &&
           (*cf).mech_type !=
               &mut (*(*cf).mech).mech_type as *mut gss_OID_desc {
        generic_gss_release_oid(&mut minor_status, &mut (*cf).mech_type);
    }
    if (*cf).freeMech != 0 {
        zapfree((*cf).mech as *mut libc::c_void,
                ::std::mem::size_of::<gss_config>() as libc::c_ulong);
    }
    if !(*cf).dl_handle.is_null() {
        krb5int_close_plugin((*cf).dl_handle as *mut plugin_file_handle);
    }
    if !(*cf).int_mech_type.is_null() {
        generic_gss_release_oid(&mut minor_status, &mut (*cf).int_mech_type);
    }
    memset(cf as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<gss_mech_config>() as libc::c_ulong);
    free(cf as *mut libc::c_void);
    *pCf = 0 as gss_mech_info;
}
/*
 * Register a mechanism.  Called with g_mechListLock held.
 */
#[no_mangle]
#[c2rust::src_loc = "587:1"]
pub unsafe extern "C" fn gssint_register_mechinfo(mut template: gss_mech_info)
 -> libc::c_int {
    let mut cf: gss_mech_info = 0 as *mut gss_mech_config;
    let mut new_cf: gss_mech_info = 0 as *mut gss_mech_config;
    new_cf =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<gss_mech_config>() as libc::c_ulong) as
            gss_mech_info;
    if new_cf.is_null() { return 12 as libc::c_int }
    (*new_cf).dl_handle = (*template).dl_handle;
    /* copy mech so we can rewrite canonical mechanism OID */
    (*new_cf).mech =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<gss_config>() as libc::c_ulong) as
            gss_mechanism;
    if (*new_cf).mech.is_null() {
        releaseMechInfo(&mut new_cf);
        return 12 as libc::c_int
    }
    *(*new_cf).mech = *(*template).mech;
    if !(*template).mech_type.is_null() {
        (*(*new_cf).mech).mech_type = *(*template).mech_type
    }
    (*new_cf).mech_type = &mut (*(*new_cf).mech).mech_type;
    (*new_cf).priority = (*template).priority;
    (*new_cf).freeMech = 1 as libc::c_int;
    (*new_cf).next = 0 as *mut gss_mech_config;
    if !(*template).kmodName.is_null() {
        (*new_cf).kmodName = strdup((*template).kmodName);
        if (*new_cf).kmodName.is_null() {
            releaseMechInfo(&mut new_cf);
            return 12 as libc::c_int
        }
    }
    if !(*template).uLibName.is_null() {
        (*new_cf).uLibName = strdup((*template).uLibName);
        if (*new_cf).uLibName.is_null() {
            releaseMechInfo(&mut new_cf);
            return 12 as libc::c_int
        }
    }
    if !(*template).mechNameStr.is_null() {
        (*new_cf).mechNameStr = strdup((*template).mechNameStr);
        if (*new_cf).mechNameStr.is_null() {
            releaseMechInfo(&mut new_cf);
            return 12 as libc::c_int
        }
    }
    if !(*template).optionStr.is_null() {
        (*new_cf).optionStr = strdup((*template).optionStr);
        if (*new_cf).optionStr.is_null() {
            releaseMechInfo(&mut new_cf);
            return 12 as libc::c_int
        }
    }
    if g_mechList.is_null() {
        g_mechList = new_cf;
        g_mechListTail = new_cf;
        return 0 as libc::c_int
    } else {
        if (*new_cf).priority < (*g_mechList).priority {
            (*new_cf).next = g_mechList;
            g_mechList = new_cf;
            return 0 as libc::c_int
        }
    }
    cf = g_mechList;
    while !cf.is_null() {
        if (*cf).next.is_null() || (*new_cf).priority < (*(*cf).next).priority
           {
            (*new_cf).next = (*cf).next;
            (*cf).next = new_cf;
            if g_mechListTail == cf { g_mechListTail = new_cf }
            break ;
        } else { cf = (*cf).next }
    }
    return 0 as libc::c_int;
}
/* _GSS_STATIC_LINK */
/*
 * If _symbol is undefined in the shared object but the shared object
 * is linked against the mechanism glue, it's possible for dlsym() to
 * return the mechanism glue implementation. Guard against that.
 */
#[c2rust::src_loc = "692:1"]
unsafe extern "C" fn build_dynamicMech(mut dl: *mut libc::c_void,
                                       mech_type: gss_OID) -> gss_mechanism {
    let mut mech: gss_mechanism = 0 as *mut gss_config;
    mech =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<gss_config>() as libc::c_ulong) as
            gss_mechanism;
    if mech.is_null() { return 0 as gss_mechanism }
    let mut errinfo: errinfo = errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_acquire_cred\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_acquire_cred as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo) != 0 || errinfo.code != 0 {
        (*mech).gss_acquire_cred = None;
        k5_clear_error(&mut errinfo);
    }
    if (*mech).gss_acquire_cred ==
           Some(gss_acquire_cred as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_name_t,
                                         _: OM_uint32, _: gss_OID_set,
                                         _: gss_cred_usage_t,
                                         _: *mut gss_cred_id_t,
                                         _: *mut gss_OID_set,
                                         _: *mut OM_uint32) -> OM_uint32) {
        (*mech).gss_acquire_cred = None
    }
    let mut errinfo_0: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_0 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_release_cred\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_release_cred as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_cred_id_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_0) != 0 || errinfo_0.code != 0 {
        (*mech).gss_release_cred = None;
        k5_clear_error(&mut errinfo_0);
    }
    if (*mech).gss_release_cred ==
           Some(gss_release_cred as
                    unsafe extern "C" fn(_: *mut OM_uint32,
                                         _: *mut gss_cred_id_t) -> OM_uint32)
       {
        (*mech).gss_release_cred = None
    }
    let mut errinfo_1: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_1 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_init_sec_context\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_init_sec_context as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        *mut gss_ctx_id_t,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        gss_channel_bindings_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut OM_uint32)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_1) != 0 || errinfo_1.code != 0 {
        (*mech).gss_init_sec_context = None;
        k5_clear_error(&mut errinfo_1);
    }
    if (*mech).gss_init_sec_context ==
           Some(gss_init_sec_context as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_cred_id_t,
                                         _: *mut gss_ctx_id_t, _: gss_name_t,
                                         _: gss_OID, _: OM_uint32,
                                         _: OM_uint32,
                                         _: gss_channel_bindings_t,
                                         _: gss_buffer_t, _: *mut gss_OID,
                                         _: gss_buffer_t, _: *mut OM_uint32,
                                         _: *mut OM_uint32) -> OM_uint32) {
        (*mech).gss_init_sec_context = None
    }
    let mut errinfo_2: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_2 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_accept_sec_context\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_accept_sec_context as
                                   *mut Option<unsafe extern "C" fn(_:
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_2) != 0 || errinfo_2.code != 0 {
        (*mech).gss_accept_sec_context = None;
        k5_clear_error(&mut errinfo_2);
    }
    if (*mech).gss_accept_sec_context ==
           Some(gss_accept_sec_context as
                    unsafe extern "C" fn(_: *mut OM_uint32,
                                         _: *mut gss_ctx_id_t,
                                         _: gss_cred_id_t, _: gss_buffer_t,
                                         _: gss_channel_bindings_t,
                                         _: *mut gss_name_t, _: *mut gss_OID,
                                         _: gss_buffer_t, _: *mut OM_uint32,
                                         _: *mut OM_uint32,
                                         _: *mut gss_cred_id_t) -> OM_uint32)
       {
        (*mech).gss_accept_sec_context = None
    }
    let mut errinfo_3: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_3 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_process_context_token\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gss_process_context_token as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_3) != 0 || errinfo_3.code != 0 {
        (*mech).gss_process_context_token = None;
        k5_clear_error(&mut errinfo_3);
    }
    if (*mech).gss_process_context_token ==
           Some(gss_process_context_token as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: gss_buffer_t) -> OM_uint32) {
        (*mech).gss_process_context_token = None
    }
    let mut errinfo_4: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_4 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_delete_sec_context\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_delete_sec_context as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_ctx_id_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_4) != 0 || errinfo_4.code != 0 {
        (*mech).gss_delete_sec_context = None;
        k5_clear_error(&mut errinfo_4);
    }
    if (*mech).gss_delete_sec_context ==
           Some(gss_delete_sec_context as
                    unsafe extern "C" fn(_: *mut OM_uint32,
                                         _: *mut gss_ctx_id_t,
                                         _: gss_buffer_t) -> OM_uint32) {
        (*mech).gss_delete_sec_context = None
    }
    let mut errinfo_5: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_5 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_context_time\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_context_time as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        *mut OM_uint32)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_5) != 0 || errinfo_5.code != 0 {
        (*mech).gss_context_time = None;
        k5_clear_error(&mut errinfo_5);
    }
    if (*mech).gss_context_time ==
           Some(gss_context_time as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: *mut OM_uint32) -> OM_uint32) {
        (*mech).gss_context_time = None
    }
    let mut errinfo_6: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_6 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_get_mic\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_get_mic as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        gss_qop_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_6) != 0 || errinfo_6.code != 0 {
        (*mech).gss_get_mic = None;
        k5_clear_error(&mut errinfo_6);
    }
    if (*mech).gss_get_mic ==
           Some(gss_get_mic as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: gss_qop_t, _: gss_buffer_t,
                                         _: gss_buffer_t) -> OM_uint32) {
        (*mech).gss_get_mic = None
    }
    let mut errinfo_7: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_7 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_verify_mic\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_verify_mic as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_qop_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_7) != 0 || errinfo_7.code != 0 {
        (*mech).gss_verify_mic = None;
        k5_clear_error(&mut errinfo_7);
    }
    if (*mech).gss_verify_mic ==
           Some(gss_verify_mic as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: gss_buffer_t, _: gss_buffer_t,
                                         _: *mut gss_qop_t) -> OM_uint32) {
        (*mech).gss_verify_mic = None
    }
    let mut errinfo_8: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_8 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_wrap\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_wrap as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_qop_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_8) != 0 || errinfo_8.code != 0 {
        (*mech).gss_wrap = None;
        k5_clear_error(&mut errinfo_8);
    }
    if (*mech).gss_wrap ==
           Some(gss_wrap as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: libc::c_int, _: gss_qop_t,
                                         _: gss_buffer_t, _: *mut libc::c_int,
                                         _: gss_buffer_t) -> OM_uint32) {
        (*mech).gss_wrap = None
    }
    let mut errinfo_9: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_9 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_unwrap\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_unwrap as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut gss_qop_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_9) != 0 || errinfo_9.code != 0 {
        (*mech).gss_unwrap = None;
        k5_clear_error(&mut errinfo_9);
    }
    if (*mech).gss_unwrap ==
           Some(gss_unwrap as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: gss_buffer_t, _: gss_buffer_t,
                                         _: *mut libc::c_int,
                                         _: *mut gss_qop_t) -> OM_uint32) {
        (*mech).gss_unwrap = None
    }
    let mut errinfo_10: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_10 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_display_status\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_display_status as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_10) != 0 || errinfo_10.code != 0 {
        (*mech).gss_display_status = None;
        k5_clear_error(&mut errinfo_10);
    }
    if (*mech).gss_display_status ==
           Some(gss_display_status as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: OM_uint32,
                                         _: libc::c_int, _: gss_OID,
                                         _: *mut OM_uint32, _: gss_buffer_t)
                        -> OM_uint32) {
        (*mech).gss_display_status = None
    }
    let mut errinfo_11: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_11 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_indicate_mechs\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_indicate_mechs as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_OID_set)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_11) != 0 || errinfo_11.code != 0 {
        (*mech).gss_indicate_mechs = None;
        k5_clear_error(&mut errinfo_11);
    }
    if (*mech).gss_indicate_mechs ==
           ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                              -> OM_uint32>,
                                   Option<unsafe extern "C" fn(_:
                                                                   *mut OM_uint32,
                                                               _:
                                                                   *mut gss_OID_set)
                                              ->
                                                  OM_uint32>>(Some(::std::mem::transmute::<unsafe extern "C" fn(_:
                                                                                                                    *mut OM_uint32,
                                                                                                                _:
                                                                                                                    *mut gss_OID_set)
                                                                                               ->
                                                                                                   OM_uint32,
                                                                                           unsafe extern "C" fn()
                                                                                               ->
                                                                                                   OM_uint32>(gss_indicate_mechs)))
       {
        (*mech).gss_indicate_mechs = None
    }
    let mut errinfo_12: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_12 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_compare_name\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_compare_name as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        *mut libc::c_int)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_12) != 0 || errinfo_12.code != 0 {
        (*mech).gss_compare_name = None;
        k5_clear_error(&mut errinfo_12);
    }
    if (*mech).gss_compare_name ==
           Some(gss_compare_name as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_name_t, _: *mut libc::c_int)
                        -> OM_uint32) {
        (*mech).gss_compare_name = None
    }
    let mut errinfo_13: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_13 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_display_name\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_display_name as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_OID)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_13) != 0 || errinfo_13.code != 0 {
        (*mech).gss_display_name = None;
        k5_clear_error(&mut errinfo_13);
    }
    if (*mech).gss_display_name ==
           Some(gss_display_name as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_buffer_t, _: *mut gss_OID)
                        -> OM_uint32) {
        (*mech).gss_display_name = None
    }
    let mut errinfo_14: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_14 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_import_name\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_import_name as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_name_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_14) != 0 || errinfo_14.code != 0 {
        (*mech).gss_import_name = None;
        k5_clear_error(&mut errinfo_14);
    }
    if (*mech).gss_import_name ==
           Some(gss_import_name as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_buffer_t,
                                         _: gss_OID, _: *mut gss_name_t)
                        -> OM_uint32) {
        (*mech).gss_import_name = None
    }
    let mut errinfo_15: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_15 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_release_name\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_release_name as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_name_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_15) != 0 || errinfo_15.code != 0 {
        (*mech).gss_release_name = None;
        k5_clear_error(&mut errinfo_15);
    }
    if (*mech).gss_release_name ==
           Some(gss_release_name as
                    unsafe extern "C" fn(_: *mut OM_uint32,
                                         _: *mut gss_name_t) -> OM_uint32) {
        (*mech).gss_release_name = None
    }
    let mut errinfo_16: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_16 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_inquire_cred\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_inquire_cred as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        *mut gss_name_t,
                                                                    _:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut gss_OID_set)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_16) != 0 || errinfo_16.code != 0 {
        (*mech).gss_inquire_cred = None;
        k5_clear_error(&mut errinfo_16);
    }
    if (*mech).gss_inquire_cred ==
           Some(gss_inquire_cred as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_cred_id_t,
                                         _: *mut gss_name_t,
                                         _: *mut OM_uint32,
                                         _: *mut gss_cred_usage_t,
                                         _: *mut gss_OID_set) -> OM_uint32) {
        (*mech).gss_inquire_cred = None
    }
    let mut errinfo_17: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_17 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_add_cred\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_add_cred as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_17) != 0 || errinfo_17.code != 0 {
        (*mech).gss_add_cred = None;
        k5_clear_error(&mut errinfo_17);
    }
    if (*mech).gss_add_cred ==
           Some(gss_add_cred as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_cred_id_t,
                                         _: gss_name_t, _: gss_OID,
                                         _: gss_cred_usage_t, _: OM_uint32,
                                         _: OM_uint32, _: *mut gss_cred_id_t,
                                         _: *mut gss_OID_set,
                                         _: *mut OM_uint32, _: *mut OM_uint32)
                        -> OM_uint32) {
        (*mech).gss_add_cred = None
    }
    let mut errinfo_18: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_18 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_export_sec_context\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_export_sec_context as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_ctx_id_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_18) != 0 || errinfo_18.code != 0 {
        (*mech).gss_export_sec_context = None;
        k5_clear_error(&mut errinfo_18);
    }
    if (*mech).gss_export_sec_context ==
           Some(gss_export_sec_context as
                    unsafe extern "C" fn(_: *mut OM_uint32,
                                         _: *mut gss_ctx_id_t,
                                         _: gss_buffer_t) -> OM_uint32) {
        (*mech).gss_export_sec_context = None
    }
    let mut errinfo_19: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_19 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_import_sec_context\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_import_sec_context as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_ctx_id_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_19) != 0 || errinfo_19.code != 0 {
        (*mech).gss_import_sec_context = None;
        k5_clear_error(&mut errinfo_19);
    }
    if (*mech).gss_import_sec_context ==
           Some(gss_import_sec_context as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_buffer_t,
                                         _: *mut gss_ctx_id_t) -> OM_uint32) {
        (*mech).gss_import_sec_context = None
    }
    let mut errinfo_20: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_20 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_inquire_cred_by_mech\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_inquire_cred_by_mech as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_name_t,
                                                                    _:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_cred_usage_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_20) != 0 || errinfo_20.code != 0 {
        (*mech).gss_inquire_cred_by_mech = None;
        k5_clear_error(&mut errinfo_20);
    }
    if (*mech).gss_inquire_cred_by_mech ==
           Some(gss_inquire_cred_by_mech as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_cred_id_t,
                                         _: gss_OID, _: *mut gss_name_t,
                                         _: *mut OM_uint32, _: *mut OM_uint32,
                                         _: *mut gss_cred_usage_t)
                        -> OM_uint32) {
        (*mech).gss_inquire_cred_by_mech = None
    }
    let mut errinfo_21: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_21 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_inquire_names_for_mech\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gss_inquire_names_for_mech as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_OID_set)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_21) != 0 || errinfo_21.code != 0 {
        (*mech).gss_inquire_names_for_mech = None;
        k5_clear_error(&mut errinfo_21);
    }
    if (*mech).gss_inquire_names_for_mech ==
           Some(gss_inquire_names_for_mech as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_OID,
                                         _: *mut gss_OID_set) -> OM_uint32) {
        (*mech).gss_inquire_names_for_mech = None
    }
    let mut errinfo_22: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_22 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_inquire_context\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_inquire_context as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        *mut gss_name_t,
                                                                    _:
                                                                        *mut gss_name_t,
                                                                    _:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_OID,
                                                                    _:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut libc::c_int)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_22) != 0 || errinfo_22.code != 0 {
        (*mech).gss_inquire_context = None;
        k5_clear_error(&mut errinfo_22);
    }
    if (*mech).gss_inquire_context ==
           Some(gss_inquire_context as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: *mut gss_name_t,
                                         _: *mut gss_name_t,
                                         _: *mut OM_uint32, _: *mut gss_OID,
                                         _: *mut OM_uint32,
                                         _: *mut libc::c_int,
                                         _: *mut libc::c_int) -> OM_uint32) {
        (*mech).gss_inquire_context = None
    }
    let mut errinfo_23: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_23 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_internal_release_oid\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_internal_release_oid as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_OID)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_23) != 0 || errinfo_23.code != 0 {
        (*mech).gss_internal_release_oid = None;
        k5_clear_error(&mut errinfo_23);
    }
    let mut errinfo_24: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_24 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_wrap_size_limit\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_wrap_size_limit as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_qop_t,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        *mut OM_uint32)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_24) != 0 || errinfo_24.code != 0 {
        (*mech).gss_wrap_size_limit = None;
        k5_clear_error(&mut errinfo_24);
    }
    if (*mech).gss_wrap_size_limit ==
           Some(gss_wrap_size_limit as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: libc::c_int, _: gss_qop_t,
                                         _: OM_uint32, _: *mut OM_uint32)
                        -> OM_uint32) {
        (*mech).gss_wrap_size_limit = None
    }
    let mut errinfo_25: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_25 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_localname\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_localname as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_const_OID,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_25) != 0 || errinfo_25.code != 0 {
        (*mech).gss_localname = None;
        k5_clear_error(&mut errinfo_25);
    }
    if (*mech).gss_localname ==
           Some(gss_localname as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_const_OID, _: gss_buffer_t)
                        -> OM_uint32) {
        (*mech).gss_localname = None
    }
    let mut errinfo_26: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_26 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssspi_authorize_localname\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gssspi_authorize_localname as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_const_buffer_t,
                                                                    _:
                                                                        gss_const_OID)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_26) != 0 || errinfo_26.code != 0 {
        (*mech).gssspi_authorize_localname = None;
        k5_clear_error(&mut errinfo_26);
    }
    let mut errinfo_27: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_27 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_export_name\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_export_name as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_27) != 0 || errinfo_27.code != 0 {
        (*mech).gss_export_name = None;
        k5_clear_error(&mut errinfo_27);
    }
    if (*mech).gss_export_name ==
           Some(gss_export_name as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_buffer_t) -> OM_uint32) {
        (*mech).gss_export_name = None
    }
    let mut errinfo_28: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_28 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_duplicate_name\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_duplicate_name as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        *mut gss_name_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_28) != 0 || errinfo_28.code != 0 {
        (*mech).gss_duplicate_name = None;
        k5_clear_error(&mut errinfo_28);
    }
    if (*mech).gss_duplicate_name ==
           Some(gss_duplicate_name as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_name_t,
                                         _: *mut gss_name_t) -> OM_uint32) {
        (*mech).gss_duplicate_name = None
    }
    let mut errinfo_29: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_29 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_store_cred\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_store_cred as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        gss_cred_usage_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        *mut gss_OID_set,
                                                                    _:
                                                                        *mut gss_cred_usage_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_29) != 0 || errinfo_29.code != 0 {
        (*mech).gss_store_cred = None;
        k5_clear_error(&mut errinfo_29);
    }
    if (*mech).gss_store_cred ==
           Some(gss_store_cred as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_cred_id_t,
                                         _: gss_cred_usage_t, _: gss_OID,
                                         _: OM_uint32, _: OM_uint32,
                                         _: *mut gss_OID_set,
                                         _: *mut gss_cred_usage_t)
                        -> OM_uint32) {
        (*mech).gss_store_cred = None
    }
    let mut errinfo_30: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_30 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_inquire_sec_context_by_oid\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gss_inquire_sec_context_by_oid as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_buffer_set_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_30) != 0 || errinfo_30.code != 0 {
        (*mech).gss_inquire_sec_context_by_oid = None;
        k5_clear_error(&mut errinfo_30);
    }
    if (*mech).gss_inquire_sec_context_by_oid ==
           Some(gss_inquire_sec_context_by_oid as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: gss_OID, _: *mut gss_buffer_set_t)
                        -> OM_uint32) {
        (*mech).gss_inquire_sec_context_by_oid = None
    }
    let mut errinfo_31: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_31 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_inquire_cred_by_oid\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_inquire_cred_by_oid as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_buffer_set_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_31) != 0 || errinfo_31.code != 0 {
        (*mech).gss_inquire_cred_by_oid = None;
        k5_clear_error(&mut errinfo_31);
    }
    if (*mech).gss_inquire_cred_by_oid ==
           Some(gss_inquire_cred_by_oid as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_cred_id_t,
                                         _: gss_OID, _: *mut gss_buffer_set_t)
                        -> OM_uint32) {
        (*mech).gss_inquire_cred_by_oid = None
    }
    let mut errinfo_32: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_32 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_set_sec_context_option\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gss_set_sec_context_option as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_ctx_id_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_32) != 0 || errinfo_32.code != 0 {
        (*mech).gss_set_sec_context_option = None;
        k5_clear_error(&mut errinfo_32);
    }
    if (*mech).gss_set_sec_context_option ==
           Some(gss_set_sec_context_option as
                    unsafe extern "C" fn(_: *mut OM_uint32,
                                         _: *mut gss_ctx_id_t, _: gss_OID,
                                         _: gss_buffer_t) -> OM_uint32) {
        (*mech).gss_set_sec_context_option = None
    }
    let mut errinfo_33: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_33 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssspi_set_cred_option\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gssspi_set_cred_option as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_cred_id_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_33) != 0 || errinfo_33.code != 0 {
        (*mech).gssspi_set_cred_option = None;
        k5_clear_error(&mut errinfo_33);
    }
    let mut errinfo_34: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_34 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssspi_mech_invoke\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gssspi_mech_invoke as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_34) != 0 || errinfo_34.code != 0 {
        (*mech).gssspi_mech_invoke = None;
        k5_clear_error(&mut errinfo_34);
    }
    if (*mech).gssspi_mech_invoke ==
           Some(gssspi_mech_invoke as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_OID,
                                         _: gss_OID, _: gss_buffer_t)
                        -> OM_uint32) {
        (*mech).gssspi_mech_invoke = None
    }
    let mut errinfo_35: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_35 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_wrap_aead\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_wrap_aead as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_qop_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_35) != 0 || errinfo_35.code != 0 {
        (*mech).gss_wrap_aead = None;
        k5_clear_error(&mut errinfo_35);
    }
    if (*mech).gss_wrap_aead ==
           Some(gss_wrap_aead as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: libc::c_int, _: gss_qop_t,
                                         _: gss_buffer_t, _: gss_buffer_t,
                                         _: *mut libc::c_int, _: gss_buffer_t)
                        -> OM_uint32) {
        (*mech).gss_wrap_aead = None
    }
    let mut errinfo_36: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_36 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_unwrap_aead\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_unwrap_aead as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut gss_qop_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_36) != 0 || errinfo_36.code != 0 {
        (*mech).gss_unwrap_aead = None;
        k5_clear_error(&mut errinfo_36);
    }
    if (*mech).gss_unwrap_aead ==
           Some(gss_unwrap_aead as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: gss_buffer_t, _: gss_buffer_t,
                                         _: gss_buffer_t, _: *mut libc::c_int,
                                         _: *mut gss_qop_t) -> OM_uint32) {
        (*mech).gss_unwrap_aead = None
    }
    let mut errinfo_37: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_37 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_wrap_iov\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_wrap_iov as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_qop_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut gss_iov_buffer_desc,
                                                                    _:
                                                                        libc::c_int)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_37) != 0 || errinfo_37.code != 0 {
        (*mech).gss_wrap_iov = None;
        k5_clear_error(&mut errinfo_37);
    }
    if (*mech).gss_wrap_iov ==
           Some(gss_wrap_iov as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: libc::c_int, _: gss_qop_t,
                                         _: *mut libc::c_int,
                                         _: *mut gss_iov_buffer_desc,
                                         _: libc::c_int) -> OM_uint32) {
        (*mech).gss_wrap_iov = None
    }
    let mut errinfo_38: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_38 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_unwrap_iov\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_unwrap_iov as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut gss_qop_t,
                                                                    _:
                                                                        *mut gss_iov_buffer_desc,
                                                                    _:
                                                                        libc::c_int)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_38) != 0 || errinfo_38.code != 0 {
        (*mech).gss_unwrap_iov = None;
        k5_clear_error(&mut errinfo_38);
    }
    if (*mech).gss_unwrap_iov ==
           Some(gss_unwrap_iov as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: *mut libc::c_int,
                                         _: *mut gss_qop_t,
                                         _: *mut gss_iov_buffer_desc,
                                         _: libc::c_int) -> OM_uint32) {
        (*mech).gss_unwrap_iov = None
    }
    let mut errinfo_39: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_39 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_wrap_iov_length\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_wrap_iov_length as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_qop_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut gss_iov_buffer_desc,
                                                                    _:
                                                                        libc::c_int)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_39) != 0 || errinfo_39.code != 0 {
        (*mech).gss_wrap_iov_length = None;
        k5_clear_error(&mut errinfo_39);
    }
    if (*mech).gss_wrap_iov_length ==
           Some(gss_wrap_iov_length as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: libc::c_int, _: gss_qop_t,
                                         _: *mut libc::c_int,
                                         _: *mut gss_iov_buffer_desc,
                                         _: libc::c_int) -> OM_uint32) {
        (*mech).gss_wrap_iov_length = None
    }
    let mut errinfo_40: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_40 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_complete_auth_token\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_complete_auth_token as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_40) != 0 || errinfo_40.code != 0 {
        (*mech).gss_complete_auth_token = None;
        k5_clear_error(&mut errinfo_40);
    }
    if (*mech).gss_complete_auth_token ==
           Some(gss_complete_auth_token as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: gss_buffer_t) -> OM_uint32) {
        (*mech).gss_complete_auth_token = None
    }
    /* Services4User (introduced in 1.8) */
    let mut errinfo_41: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_41 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_acquire_cred_impersonate_name\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gss_acquire_cred_impersonate_name
                                   as
                                   *mut Option<unsafe extern "C" fn(_:
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_41) != 0 || errinfo_41.code != 0 {
        (*mech).gss_acquire_cred_impersonate_name = None;
        k5_clear_error(&mut errinfo_41);
    }
    if (*mech).gss_acquire_cred_impersonate_name ==
           Some(gss_acquire_cred_impersonate_name as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_cred_id_t,
                                         _: gss_name_t, _: OM_uint32,
                                         _: gss_OID_set, _: gss_cred_usage_t,
                                         _: *mut gss_cred_id_t,
                                         _: *mut gss_OID_set,
                                         _: *mut OM_uint32) -> OM_uint32) {
        (*mech).gss_acquire_cred_impersonate_name = None
    }
    let mut errinfo_42: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_42 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_add_cred_impersonate_name\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gss_add_cred_impersonate_name as
                                   *mut Option<unsafe extern "C" fn(_:
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_42) != 0 || errinfo_42.code != 0 {
        (*mech).gss_add_cred_impersonate_name = None;
        k5_clear_error(&mut errinfo_42);
    }
    if (*mech).gss_add_cred_impersonate_name ==
           Some(gss_add_cred_impersonate_name as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_cred_id_t,
                                         _: gss_cred_id_t, _: gss_name_t,
                                         _: gss_OID, _: gss_cred_usage_t,
                                         _: OM_uint32, _: OM_uint32,
                                         _: *mut gss_cred_id_t,
                                         _: *mut gss_OID_set,
                                         _: *mut OM_uint32, _: *mut OM_uint32)
                        -> OM_uint32) {
        (*mech).gss_add_cred_impersonate_name = None
    }
    /* Naming extensions (introduced in 1.8) */
    let mut errinfo_43: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_43 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_display_name_ext\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_display_name_ext as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_43) != 0 || errinfo_43.code != 0 {
        (*mech).gss_display_name_ext = None;
        k5_clear_error(&mut errinfo_43);
    }
    if (*mech).gss_display_name_ext ==
           Some(gss_display_name_ext as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_OID, _: gss_buffer_t)
                        -> OM_uint32) {
        (*mech).gss_display_name_ext = None
    }
    let mut errinfo_44: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_44 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_inquire_name\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_inquire_name as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut gss_OID,
                                                                    _:
                                                                        *mut gss_buffer_set_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_44) != 0 || errinfo_44.code != 0 {
        (*mech).gss_inquire_name = None;
        k5_clear_error(&mut errinfo_44);
    }
    if (*mech).gss_inquire_name ==
           Some(gss_inquire_name as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_name_t,
                                         _: *mut libc::c_int, _: *mut gss_OID,
                                         _: *mut gss_buffer_set_t)
                        -> OM_uint32) {
        (*mech).gss_inquire_name = None
    }
    let mut errinfo_45: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_45 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_get_name_attribute\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_get_name_attribute as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_45) != 0 || errinfo_45.code != 0 {
        (*mech).gss_get_name_attribute = None;
        k5_clear_error(&mut errinfo_45);
    }
    if (*mech).gss_get_name_attribute ==
           Some(gss_get_name_attribute as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_buffer_t, _: *mut libc::c_int,
                                         _: *mut libc::c_int, _: gss_buffer_t,
                                         _: gss_buffer_t, _: *mut libc::c_int)
                        -> OM_uint32) {
        (*mech).gss_get_name_attribute = None
    }
    let mut errinfo_46: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_46 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_set_name_attribute\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_set_name_attribute as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_46) != 0 || errinfo_46.code != 0 {
        (*mech).gss_set_name_attribute = None;
        k5_clear_error(&mut errinfo_46);
    }
    if (*mech).gss_set_name_attribute ==
           Some(gss_set_name_attribute as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_name_t,
                                         _: libc::c_int, _: gss_buffer_t,
                                         _: gss_buffer_t) -> OM_uint32) {
        (*mech).gss_set_name_attribute = None
    }
    let mut errinfo_47: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_47 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_delete_name_attribute\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gss_delete_name_attribute as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_47) != 0 || errinfo_47.code != 0 {
        (*mech).gss_delete_name_attribute = None;
        k5_clear_error(&mut errinfo_47);
    }
    if (*mech).gss_delete_name_attribute ==
           Some(gss_delete_name_attribute as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_buffer_t) -> OM_uint32) {
        (*mech).gss_delete_name_attribute = None
    }
    let mut errinfo_48: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_48 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_export_name_composite\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gss_export_name_composite as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_48) != 0 || errinfo_48.code != 0 {
        (*mech).gss_export_name_composite = None;
        k5_clear_error(&mut errinfo_48);
    }
    if (*mech).gss_export_name_composite ==
           Some(gss_export_name_composite as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_buffer_t) -> OM_uint32) {
        (*mech).gss_export_name_composite = None
    }
    let mut errinfo_49: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_49 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_map_name_to_any\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_map_name_to_any as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_any_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_49) != 0 || errinfo_49.code != 0 {
        (*mech).gss_map_name_to_any = None;
        k5_clear_error(&mut errinfo_49);
    }
    if (*mech).gss_map_name_to_any ==
           Some(gss_map_name_to_any as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_name_t,
                                         _: libc::c_int, _: gss_buffer_t,
                                         _: *mut gss_any_t) -> OM_uint32) {
        (*mech).gss_map_name_to_any = None
    }
    let mut errinfo_50: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_50 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_release_any_name_mapping\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gss_release_any_name_mapping as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_any_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_50) != 0 || errinfo_50.code != 0 {
        (*mech).gss_release_any_name_mapping = None;
        k5_clear_error(&mut errinfo_50);
    }
    if (*mech).gss_release_any_name_mapping ==
           Some(gss_release_any_name_mapping as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_buffer_t, _: *mut gss_any_t)
                        -> OM_uint32) {
        (*mech).gss_release_any_name_mapping = None
    }
    /* RFC 4401 (introduced in 1.8) */
    let mut errinfo_51: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_51 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_pseudo_random\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_pseudo_random as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        ssize_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_51) != 0 || errinfo_51.code != 0 {
        (*mech).gss_pseudo_random = None;
        k5_clear_error(&mut errinfo_51);
    }
    if (*mech).gss_pseudo_random ==
           Some(gss_pseudo_random as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: libc::c_int, _: gss_buffer_t,
                                         _: ssize_t, _: gss_buffer_t)
                        -> OM_uint32) {
        (*mech).gss_pseudo_random = None
    }
    /* RFC 4178 (introduced in 1.8; gss_get_neg_mechs not implemented) */
    let mut errinfo_52: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_52 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_set_neg_mechs\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_set_neg_mechs as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        gss_OID_set)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_52) != 0 || errinfo_52.code != 0 {
        (*mech).gss_set_neg_mechs = None;
        k5_clear_error(&mut errinfo_52);
    }
    if (*mech).gss_set_neg_mechs ==
           Some(gss_set_neg_mechs as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_cred_id_t,
                                         _: gss_OID_set) -> OM_uint32) {
        (*mech).gss_set_neg_mechs = None
    }
    /* draft-ietf-sasl-gs2 */
    let mut errinfo_53: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_53 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_inquire_saslname_for_mech\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gss_inquire_saslname_for_mech as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_53) != 0 || errinfo_53.code != 0 {
        (*mech).gss_inquire_saslname_for_mech = None;
        k5_clear_error(&mut errinfo_53);
    }
    if (*mech).gss_inquire_saslname_for_mech ==
           Some(gss_inquire_saslname_for_mech as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_OID,
                                         _: gss_buffer_t, _: gss_buffer_t,
                                         _: gss_buffer_t) -> OM_uint32) {
        (*mech).gss_inquire_saslname_for_mech = None
    }
    let mut errinfo_54: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_54 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_inquire_mech_for_saslname\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gss_inquire_mech_for_saslname as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_OID)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_54) != 0 || errinfo_54.code != 0 {
        (*mech).gss_inquire_mech_for_saslname = None;
        k5_clear_error(&mut errinfo_54);
    }
    if (*mech).gss_inquire_mech_for_saslname ==
           Some(gss_inquire_mech_for_saslname as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_buffer_t,
                                         _: *mut gss_OID) -> OM_uint32) {
        (*mech).gss_inquire_mech_for_saslname = None
    }
    /* RFC 5587 */
    let mut errinfo_55: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_55 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_inquire_attrs_for_mech\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gss_inquire_attrs_for_mech as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_const_OID,
                                                                    _:
                                                                        *mut gss_OID_set,
                                                                    _:
                                                                        *mut gss_OID_set)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_55) != 0 || errinfo_55.code != 0 {
        (*mech).gss_inquire_attrs_for_mech = None;
        k5_clear_error(&mut errinfo_55);
    }
    if (*mech).gss_inquire_attrs_for_mech ==
           Some(gss_inquire_attrs_for_mech as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_const_OID,
                                         _: *mut gss_OID_set,
                                         _: *mut gss_OID_set) -> OM_uint32) {
        (*mech).gss_inquire_attrs_for_mech = None
    }
    let mut errinfo_56: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_56 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_acquire_cred_from\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_acquire_cred_from as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        gss_OID_set,
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_56) != 0 || errinfo_56.code != 0 {
        (*mech).gss_acquire_cred_from = None;
        k5_clear_error(&mut errinfo_56);
    }
    if (*mech).gss_acquire_cred_from ==
           Some(gss_acquire_cred_from as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_name_t,
                                         _: OM_uint32, _: gss_OID_set,
                                         _: gss_cred_usage_t,
                                         _: gss_const_key_value_set_t,
                                         _: *mut gss_cred_id_t,
                                         _: *mut gss_OID_set,
                                         _: *mut OM_uint32) -> OM_uint32) {
        (*mech).gss_acquire_cred_from = None
    }
    let mut errinfo_57: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_57 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_store_cred_into\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_store_cred_into as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        gss_cred_usage_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        gss_const_key_value_set_t,
                                                                    _:
                                                                        *mut gss_OID_set,
                                                                    _:
                                                                        *mut gss_cred_usage_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_57) != 0 || errinfo_57.code != 0 {
        (*mech).gss_store_cred_into = None;
        k5_clear_error(&mut errinfo_57);
    }
    if (*mech).gss_store_cred_into ==
           Some(gss_store_cred_into as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_cred_id_t,
                                         _: gss_cred_usage_t, _: gss_OID,
                                         _: OM_uint32, _: OM_uint32,
                                         _: gss_const_key_value_set_t,
                                         _: *mut gss_OID_set,
                                         _: *mut gss_cred_usage_t)
                        -> OM_uint32) {
        (*mech).gss_store_cred_into = None
    }
    let mut errinfo_58: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_58 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssspi_acquire_cred_with_password\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gssspi_acquire_cred_with_password
                                   as
                                   *mut Option<unsafe extern "C" fn(_:
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_58) != 0 || errinfo_58.code != 0 {
        (*mech).gssspi_acquire_cred_with_password = None;
        k5_clear_error(&mut errinfo_58);
    }
    let mut errinfo_59: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_59 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_export_cred\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_export_cred as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_59) != 0 || errinfo_59.code != 0 {
        (*mech).gss_export_cred = None;
        k5_clear_error(&mut errinfo_59);
    }
    if (*mech).gss_export_cred ==
           Some(gss_export_cred as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_cred_id_t,
                                         _: gss_buffer_t) -> OM_uint32) {
        (*mech).gss_export_cred = None
    }
    let mut errinfo_60: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_60 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gss_import_cred\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_import_cred as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_cred_id_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_60) != 0 || errinfo_60.code != 0 {
        (*mech).gss_import_cred = None;
        k5_clear_error(&mut errinfo_60);
    }
    if (*mech).gss_import_cred ==
           Some(gss_import_cred as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_buffer_t,
                                         _: *mut gss_cred_id_t) -> OM_uint32)
       {
        (*mech).gss_import_cred = None
    }
    let mut errinfo_61: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_61 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssspi_import_sec_context_by_mech\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gssspi_import_sec_context_by_mech
                                   as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_ctx_id_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_61) != 0 || errinfo_61.code != 0 {
        (*mech).gssspi_import_sec_context_by_mech = None;
        k5_clear_error(&mut errinfo_61);
    }
    let mut errinfo_62: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_62 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssspi_import_name_by_mech\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gssspi_import_name_by_mech as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_name_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_62) != 0 || errinfo_62.code != 0 {
        (*mech).gssspi_import_name_by_mech = None;
        k5_clear_error(&mut errinfo_62);
    }
    let mut errinfo_63: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_63 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssspi_import_cred_by_mech\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gssspi_import_cred_by_mech as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_cred_id_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_63) != 0 || errinfo_63.code != 0 {
        (*mech).gssspi_import_cred_by_mech = None;
        k5_clear_error(&mut errinfo_63);
    }
    /* draft-zhu-negoex */
    let mut errinfo_64: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_64 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssspi_query_meta_data\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gssspi_query_meta_data as
                                   *mut Option<unsafe extern "C" fn(_:
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
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_64) != 0 || errinfo_64.code != 0 {
        (*mech).gssspi_query_meta_data = None;
        k5_clear_error(&mut errinfo_64);
    }
    if (*mech).gssspi_query_meta_data ==
           Some(gssspi_query_meta_data as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_const_OID,
                                         _: gss_cred_id_t,
                                         _: *mut gss_ctx_id_t, _: gss_name_t,
                                         _: OM_uint32, _: gss_buffer_t)
                        -> OM_uint32) {
        (*mech).gssspi_query_meta_data = None
    }
    let mut errinfo_65: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_65 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssspi_exchange_meta_data\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gssspi_exchange_meta_data as
                                   *mut Option<unsafe extern "C" fn(_:
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_65) != 0 || errinfo_65.code != 0 {
        (*mech).gssspi_exchange_meta_data = None;
        k5_clear_error(&mut errinfo_65);
    }
    if (*mech).gssspi_exchange_meta_data ==
           Some(gssspi_exchange_meta_data as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_const_OID,
                                         _: gss_cred_id_t,
                                         _: *mut gss_ctx_id_t, _: gss_name_t,
                                         _: OM_uint32, _: gss_const_buffer_t)
                        -> OM_uint32) {
        (*mech).gssspi_exchange_meta_data = None
    }
    let mut errinfo_66: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_66 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssspi_query_mechanism_info\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gssspi_query_mechanism_info as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_const_OID,
                                                                    _:
                                                                        *mut libc::c_uchar)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_66) != 0 || errinfo_66.code != 0 {
        (*mech).gssspi_query_mechanism_info = None;
        k5_clear_error(&mut errinfo_66);
    }
    if (*mech).gssspi_query_mechanism_info ==
           Some(gssspi_query_mechanism_info as
                    unsafe extern "C" fn(_: *mut OM_uint32, _: gss_const_OID,
                                         _: *mut libc::c_uchar) -> OM_uint32)
       {
        (*mech).gssspi_query_mechanism_info = None
    }
    if !mech_type.is_null() {
    } else {
        __assert_fail(b"mech_type != GSS_C_NO_OID\x00" as *const u8 as
                          *const libc::c_char,
                      b"g_initialize.c\x00" as *const u8 as
                          *const libc::c_char,
                      778 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"gss_mechanism build_dynamicMech(void *, const gss_OID)\x00")).as_ptr());
    }
    (*mech).mech_type = *mech_type;
    return mech;
}
/* Build an interposer mechanism function table from dl. */
#[c2rust::src_loc = "800:1"]
unsafe extern "C" fn build_interMech(mut dl: *mut libc::c_void,
                                     mech_type: gss_OID) -> gss_mechanism {
    let mut mech: gss_mechanism = 0 as *mut gss_config;
    mech =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<gss_config>() as libc::c_ulong) as
            gss_mechanism;
    if mech.is_null() { return 0 as gss_mechanism }
    let mut errinfo: errinfo = errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_acquire_cred\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_acquire_cred as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo) != 0 || errinfo.code != 0 {
        (*mech).gss_acquire_cred = None;
        k5_clear_error(&mut errinfo);
    }
    let mut errinfo_0: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_0 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_release_cred\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_release_cred as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_cred_id_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_0) != 0 || errinfo_0.code != 0 {
        (*mech).gss_release_cred = None;
        k5_clear_error(&mut errinfo_0);
    }
    let mut errinfo_1: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_1 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_init_sec_context\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_init_sec_context as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        *mut gss_ctx_id_t,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        gss_channel_bindings_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut OM_uint32)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_1) != 0 || errinfo_1.code != 0 {
        (*mech).gss_init_sec_context = None;
        k5_clear_error(&mut errinfo_1);
    }
    let mut errinfo_2: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_2 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_accept_sec_context\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_accept_sec_context as
                                   *mut Option<unsafe extern "C" fn(_:
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_2) != 0 || errinfo_2.code != 0 {
        (*mech).gss_accept_sec_context = None;
        k5_clear_error(&mut errinfo_2);
    }
    let mut errinfo_3: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_3 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_process_context_token\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gss_process_context_token as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_3) != 0 || errinfo_3.code != 0 {
        (*mech).gss_process_context_token = None;
        k5_clear_error(&mut errinfo_3);
    }
    let mut errinfo_4: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_4 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_delete_sec_context\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_delete_sec_context as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_ctx_id_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_4) != 0 || errinfo_4.code != 0 {
        (*mech).gss_delete_sec_context = None;
        k5_clear_error(&mut errinfo_4);
    }
    let mut errinfo_5: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_5 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_context_time\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_context_time as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        *mut OM_uint32)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_5) != 0 || errinfo_5.code != 0 {
        (*mech).gss_context_time = None;
        k5_clear_error(&mut errinfo_5);
    }
    let mut errinfo_6: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_6 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_get_mic\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_get_mic as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        gss_qop_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_6) != 0 || errinfo_6.code != 0 {
        (*mech).gss_get_mic = None;
        k5_clear_error(&mut errinfo_6);
    }
    let mut errinfo_7: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_7 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_verify_mic\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_verify_mic as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_qop_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_7) != 0 || errinfo_7.code != 0 {
        (*mech).gss_verify_mic = None;
        k5_clear_error(&mut errinfo_7);
    }
    let mut errinfo_8: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_8 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_wrap\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_wrap as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_qop_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_8) != 0 || errinfo_8.code != 0 {
        (*mech).gss_wrap = None;
        k5_clear_error(&mut errinfo_8);
    }
    let mut errinfo_9: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_9 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_unwrap\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_unwrap as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut gss_qop_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_9) != 0 || errinfo_9.code != 0 {
        (*mech).gss_unwrap = None;
        k5_clear_error(&mut errinfo_9);
    }
    let mut errinfo_10: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_10 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_display_status\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_display_status as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_10) != 0 || errinfo_10.code != 0 {
        (*mech).gss_display_status = None;
        k5_clear_error(&mut errinfo_10);
    }
    let mut errinfo_11: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_11 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_indicate_mechs\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_indicate_mechs as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_OID_set)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_11) != 0 || errinfo_11.code != 0 {
        (*mech).gss_indicate_mechs = None;
        k5_clear_error(&mut errinfo_11);
    }
    let mut errinfo_12: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_12 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_compare_name\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_compare_name as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        *mut libc::c_int)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_12) != 0 || errinfo_12.code != 0 {
        (*mech).gss_compare_name = None;
        k5_clear_error(&mut errinfo_12);
    }
    let mut errinfo_13: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_13 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_display_name\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_display_name as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_OID)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_13) != 0 || errinfo_13.code != 0 {
        (*mech).gss_display_name = None;
        k5_clear_error(&mut errinfo_13);
    }
    let mut errinfo_14: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_14 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_import_name\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_import_name as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_name_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_14) != 0 || errinfo_14.code != 0 {
        (*mech).gss_import_name = None;
        k5_clear_error(&mut errinfo_14);
    }
    let mut errinfo_15: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_15 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_release_name\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_release_name as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_name_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_15) != 0 || errinfo_15.code != 0 {
        (*mech).gss_release_name = None;
        k5_clear_error(&mut errinfo_15);
    }
    let mut errinfo_16: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_16 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_inquire_cred\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_inquire_cred as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        *mut gss_name_t,
                                                                    _:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut gss_OID_set)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_16) != 0 || errinfo_16.code != 0 {
        (*mech).gss_inquire_cred = None;
        k5_clear_error(&mut errinfo_16);
    }
    let mut errinfo_17: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_17 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_add_cred\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_add_cred as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_17) != 0 || errinfo_17.code != 0 {
        (*mech).gss_add_cred = None;
        k5_clear_error(&mut errinfo_17);
    }
    let mut errinfo_18: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_18 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_export_sec_context\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_export_sec_context as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_ctx_id_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_18) != 0 || errinfo_18.code != 0 {
        (*mech).gss_export_sec_context = None;
        k5_clear_error(&mut errinfo_18);
    }
    let mut errinfo_19: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_19 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_import_sec_context\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_import_sec_context as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_ctx_id_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_19) != 0 || errinfo_19.code != 0 {
        (*mech).gss_import_sec_context = None;
        k5_clear_error(&mut errinfo_19);
    }
    let mut errinfo_20: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_20 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_inquire_cred_by_mech\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gss_inquire_cred_by_mech as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_name_t,
                                                                    _:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_cred_usage_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_20) != 0 || errinfo_20.code != 0 {
        (*mech).gss_inquire_cred_by_mech = None;
        k5_clear_error(&mut errinfo_20);
    }
    let mut errinfo_21: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_21 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_inquire_names_for_mech\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gss_inquire_names_for_mech as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_OID_set)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_21) != 0 || errinfo_21.code != 0 {
        (*mech).gss_inquire_names_for_mech = None;
        k5_clear_error(&mut errinfo_21);
    }
    let mut errinfo_22: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_22 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_inquire_context\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_inquire_context as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        *mut gss_name_t,
                                                                    _:
                                                                        *mut gss_name_t,
                                                                    _:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_OID,
                                                                    _:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut libc::c_int)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_22) != 0 || errinfo_22.code != 0 {
        (*mech).gss_inquire_context = None;
        k5_clear_error(&mut errinfo_22);
    }
    let mut errinfo_23: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_23 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_internal_release_oid\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gss_internal_release_oid as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_OID)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_23) != 0 || errinfo_23.code != 0 {
        (*mech).gss_internal_release_oid = None;
        k5_clear_error(&mut errinfo_23);
    }
    let mut errinfo_24: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_24 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_wrap_size_limit\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_wrap_size_limit as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_qop_t,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        *mut OM_uint32)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_24) != 0 || errinfo_24.code != 0 {
        (*mech).gss_wrap_size_limit = None;
        k5_clear_error(&mut errinfo_24);
    }
    let mut errinfo_25: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_25 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_localname\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_localname as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_const_OID,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_25) != 0 || errinfo_25.code != 0 {
        (*mech).gss_localname = None;
        k5_clear_error(&mut errinfo_25);
    }
    let mut errinfo_26: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_26 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_authorize_localname\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gssspi_authorize_localname as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_const_buffer_t,
                                                                    _:
                                                                        gss_const_OID)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_26) != 0 || errinfo_26.code != 0 {
        (*mech).gssspi_authorize_localname = None;
        k5_clear_error(&mut errinfo_26);
    }
    let mut errinfo_27: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_27 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_export_name\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_export_name as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_27) != 0 || errinfo_27.code != 0 {
        (*mech).gss_export_name = None;
        k5_clear_error(&mut errinfo_27);
    }
    let mut errinfo_28: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_28 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_duplicate_name\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_duplicate_name as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        *mut gss_name_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_28) != 0 || errinfo_28.code != 0 {
        (*mech).gss_duplicate_name = None;
        k5_clear_error(&mut errinfo_28);
    }
    let mut errinfo_29: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_29 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_store_cred\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_store_cred as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        gss_cred_usage_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        *mut gss_OID_set,
                                                                    _:
                                                                        *mut gss_cred_usage_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_29) != 0 || errinfo_29.code != 0 {
        (*mech).gss_store_cred = None;
        k5_clear_error(&mut errinfo_29);
    }
    let mut errinfo_30: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_30 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_inquire_sec_context_by_oid\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gss_inquire_sec_context_by_oid as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_buffer_set_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_30) != 0 || errinfo_30.code != 0 {
        (*mech).gss_inquire_sec_context_by_oid = None;
        k5_clear_error(&mut errinfo_30);
    }
    let mut errinfo_31: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_31 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_inquire_cred_by_oid\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_inquire_cred_by_oid as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_buffer_set_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_31) != 0 || errinfo_31.code != 0 {
        (*mech).gss_inquire_cred_by_oid = None;
        k5_clear_error(&mut errinfo_31);
    }
    let mut errinfo_32: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_32 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_set_sec_context_option\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gss_set_sec_context_option as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_ctx_id_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_32) != 0 || errinfo_32.code != 0 {
        (*mech).gss_set_sec_context_option = None;
        k5_clear_error(&mut errinfo_32);
    }
    let mut errinfo_33: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_33 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_set_cred_option\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gssspi_set_cred_option as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_cred_id_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_33) != 0 || errinfo_33.code != 0 {
        (*mech).gssspi_set_cred_option = None;
        k5_clear_error(&mut errinfo_33);
    }
    let mut errinfo_34: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_34 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_mech_invoke\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gssspi_mech_invoke as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_34) != 0 || errinfo_34.code != 0 {
        (*mech).gssspi_mech_invoke = None;
        k5_clear_error(&mut errinfo_34);
    }
    let mut errinfo_35: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_35 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_wrap_aead\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_wrap_aead as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_qop_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_35) != 0 || errinfo_35.code != 0 {
        (*mech).gss_wrap_aead = None;
        k5_clear_error(&mut errinfo_35);
    }
    let mut errinfo_36: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_36 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_unwrap_aead\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_unwrap_aead as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut gss_qop_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_36) != 0 || errinfo_36.code != 0 {
        (*mech).gss_unwrap_aead = None;
        k5_clear_error(&mut errinfo_36);
    }
    let mut errinfo_37: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_37 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_wrap_iov\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_wrap_iov as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_qop_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut gss_iov_buffer_desc,
                                                                    _:
                                                                        libc::c_int)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_37) != 0 || errinfo_37.code != 0 {
        (*mech).gss_wrap_iov = None;
        k5_clear_error(&mut errinfo_37);
    }
    let mut errinfo_38: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_38 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_unwrap_iov\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_unwrap_iov as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut gss_qop_t,
                                                                    _:
                                                                        *mut gss_iov_buffer_desc,
                                                                    _:
                                                                        libc::c_int)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_38) != 0 || errinfo_38.code != 0 {
        (*mech).gss_unwrap_iov = None;
        k5_clear_error(&mut errinfo_38);
    }
    let mut errinfo_39: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_39 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_wrap_iov_length\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_wrap_iov_length as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_qop_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut gss_iov_buffer_desc,
                                                                    _:
                                                                        libc::c_int)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_39) != 0 || errinfo_39.code != 0 {
        (*mech).gss_wrap_iov_length = None;
        k5_clear_error(&mut errinfo_39);
    }
    let mut errinfo_40: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_40 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_complete_auth_token\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_complete_auth_token as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_40) != 0 || errinfo_40.code != 0 {
        (*mech).gss_complete_auth_token = None;
        k5_clear_error(&mut errinfo_40);
    }
    /* Services4User (introduced in 1.8) */
    let mut errinfo_41: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_41 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_acquire_cred_impersonate_name\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gss_acquire_cred_impersonate_name
                                   as
                                   *mut Option<unsafe extern "C" fn(_:
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_41) != 0 || errinfo_41.code != 0 {
        (*mech).gss_acquire_cred_impersonate_name = None;
        k5_clear_error(&mut errinfo_41);
    }
    let mut errinfo_42: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_42 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_add_cred_impersonate_name\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gss_add_cred_impersonate_name as
                                   *mut Option<unsafe extern "C" fn(_:
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_42) != 0 || errinfo_42.code != 0 {
        (*mech).gss_add_cred_impersonate_name = None;
        k5_clear_error(&mut errinfo_42);
    }
    /* Naming extensions (introduced in 1.8) */
    let mut errinfo_43: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_43 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_display_name_ext\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_display_name_ext as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_43) != 0 || errinfo_43.code != 0 {
        (*mech).gss_display_name_ext = None;
        k5_clear_error(&mut errinfo_43);
    }
    let mut errinfo_44: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_44 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_inquire_name\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_inquire_name as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut gss_OID,
                                                                    _:
                                                                        *mut gss_buffer_set_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_44) != 0 || errinfo_44.code != 0 {
        (*mech).gss_inquire_name = None;
        k5_clear_error(&mut errinfo_44);
    }
    let mut errinfo_45: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_45 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_get_name_attribute\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_get_name_attribute as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_45) != 0 || errinfo_45.code != 0 {
        (*mech).gss_get_name_attribute = None;
        k5_clear_error(&mut errinfo_45);
    }
    let mut errinfo_46: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_46 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_set_name_attribute\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_set_name_attribute as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_46) != 0 || errinfo_46.code != 0 {
        (*mech).gss_set_name_attribute = None;
        k5_clear_error(&mut errinfo_46);
    }
    let mut errinfo_47: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_47 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_delete_name_attribute\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gss_delete_name_attribute as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_47) != 0 || errinfo_47.code != 0 {
        (*mech).gss_delete_name_attribute = None;
        k5_clear_error(&mut errinfo_47);
    }
    let mut errinfo_48: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_48 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_export_name_composite\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gss_export_name_composite as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_48) != 0 || errinfo_48.code != 0 {
        (*mech).gss_export_name_composite = None;
        k5_clear_error(&mut errinfo_48);
    }
    let mut errinfo_49: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_49 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_map_name_to_any\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_map_name_to_any as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_any_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_49) != 0 || errinfo_49.code != 0 {
        (*mech).gss_map_name_to_any = None;
        k5_clear_error(&mut errinfo_49);
    }
    let mut errinfo_50: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_50 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_release_any_name_mapping\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gss_release_any_name_mapping as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_any_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_50) != 0 || errinfo_50.code != 0 {
        (*mech).gss_release_any_name_mapping = None;
        k5_clear_error(&mut errinfo_50);
    }
    /* RFC 4401 (introduced in 1.8) */
    let mut errinfo_51: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_51 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_pseudo_random\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_pseudo_random as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_ctx_id_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        ssize_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_51) != 0 || errinfo_51.code != 0 {
        (*mech).gss_pseudo_random = None;
        k5_clear_error(&mut errinfo_51);
    }
    /* RFC 4178 (introduced in 1.8; get_neg_mechs not implemented) */
    let mut errinfo_52: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_52 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_set_neg_mechs\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_set_neg_mechs as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        gss_OID_set)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_52) != 0 || errinfo_52.code != 0 {
        (*mech).gss_set_neg_mechs = None;
        k5_clear_error(&mut errinfo_52);
    }
    /* draft-ietf-sasl-gs2 */
    let mut errinfo_53: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_53 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_inquire_saslname_for_mech\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gss_inquire_saslname_for_mech as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_53) != 0 || errinfo_53.code != 0 {
        (*mech).gss_inquire_saslname_for_mech = None;
        k5_clear_error(&mut errinfo_53);
    }
    let mut errinfo_54: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_54 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_inquire_mech_for_saslname\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gss_inquire_mech_for_saslname as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_OID)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_54) != 0 || errinfo_54.code != 0 {
        (*mech).gss_inquire_mech_for_saslname = None;
        k5_clear_error(&mut errinfo_54);
    }
    /* RFC 5587 */
    let mut errinfo_55: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_55 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_inquire_attrs_for_mech\x00" as *const u8
                                   as *const libc::c_char,
                               &mut (*mech).gss_inquire_attrs_for_mech as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_const_OID,
                                                                    _:
                                                                        *mut gss_OID_set,
                                                                    _:
                                                                        *mut gss_OID_set)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_55) != 0 || errinfo_55.code != 0 {
        (*mech).gss_inquire_attrs_for_mech = None;
        k5_clear_error(&mut errinfo_55);
    }
    let mut errinfo_56: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_56 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_acquire_cred_from\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_acquire_cred_from as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        gss_OID_set,
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_56) != 0 || errinfo_56.code != 0 {
        (*mech).gss_acquire_cred_from = None;
        k5_clear_error(&mut errinfo_56);
    }
    let mut errinfo_57: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_57 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_store_cred_into\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_store_cred_into as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        gss_cred_usage_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        OM_uint32,
                                                                    _:
                                                                        gss_const_key_value_set_t,
                                                                    _:
                                                                        *mut gss_OID_set,
                                                                    _:
                                                                        *mut gss_cred_usage_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_57) != 0 || errinfo_57.code != 0 {
        (*mech).gss_store_cred_into = None;
        k5_clear_error(&mut errinfo_57);
    }
    let mut errinfo_58: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_58 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_acquire_cred_with_password\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gssspi_acquire_cred_with_password
                                   as
                                   *mut Option<unsafe extern "C" fn(_:
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
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_58) != 0 || errinfo_58.code != 0 {
        (*mech).gssspi_acquire_cred_with_password = None;
        k5_clear_error(&mut errinfo_58);
    }
    let mut errinfo_59: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_59 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_export_cred\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_export_cred as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_cred_id_t,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_59) != 0 || errinfo_59.code != 0 {
        (*mech).gss_export_cred = None;
        k5_clear_error(&mut errinfo_59);
    }
    let mut errinfo_60: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_60 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_import_cred\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gss_import_cred as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_cred_id_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_60) != 0 || errinfo_60.code != 0 {
        (*mech).gss_import_cred = None;
        k5_clear_error(&mut errinfo_60);
    }
    let mut errinfo_61: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_61 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_import_sec_context_by_mech\x00" as
                                   *const u8 as *const libc::c_char,
                               &mut (*mech).gssspi_import_sec_context_by_mech
                                   as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_ctx_id_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_61) != 0 || errinfo_61.code != 0 {
        (*mech).gssspi_import_sec_context_by_mech = None;
        k5_clear_error(&mut errinfo_61);
    }
    let mut errinfo_62: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_62 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_import_name_by_mech\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gssspi_import_name_by_mech as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_name_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_62) != 0 || errinfo_62.code != 0 {
        (*mech).gssspi_import_name_by_mech = None;
        k5_clear_error(&mut errinfo_62);
    }
    let mut errinfo_63: errinfo =
        errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    memset(&mut errinfo_63 as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_get_plugin_func(dl as *mut plugin_file_handle,
                               b"gssi_import_cred_by_mech\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut (*mech).gssspi_import_cred_by_mech as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_cred_id_t)
                                                   -> OM_uint32> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo_63) != 0 || errinfo_63.code != 0 {
        (*mech).gssspi_import_cred_by_mech = None;
        k5_clear_error(&mut errinfo_63);
    }
    (*mech).mech_type = *mech_type;
    return mech;
}
/*
 * Concatenate an interposer mech OID and a real mech OID to create an
 * identifier for the interposed mech.  (The concatenation will not be a valid
 * DER OID encoding, but the OID is only used internally.)
 */
#[c2rust::src_loc = "891:1"]
unsafe extern "C" fn interposed_oid(mut pre: gss_OID, mut real: gss_OID)
 -> gss_OID {
    let mut o: gss_OID = 0 as *mut gss_OID_desc_struct;
    o =
        malloc(::std::mem::size_of::<gss_OID_desc>() as libc::c_ulong) as
            gss_OID;
    if o.is_null() { return 0 as gss_OID }
    (*o).length = (*pre).length.wrapping_add((*real).length);
    (*o).elements = malloc((*o).length as libc::c_ulong);
    if (*o).elements.is_null() {
        free(o as *mut libc::c_void);
        return 0 as gss_OID
    }
    memcpy((*o).elements, (*pre).elements, (*pre).length as libc::c_ulong);
    memcpy(((*o).elements as *mut libc::c_char).offset((*pre).length as isize)
               as *mut libc::c_void, (*real).elements,
           (*real).length as libc::c_ulong);
    return o;
}
#[c2rust::src_loc = "914:1"]
unsafe extern "C" fn loadInterMech(mut minfo: gss_mech_info) {
    let mut dl: *mut plugin_file_handle = 0 as *mut plugin_file_handle;
    let mut errinfo: errinfo = errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    let mut isym: Option<unsafe extern "C" fn(_: gss_OID) -> gss_OID_set> =
        None;
    let mut list: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut oid: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut min: OM_uint32 = 0;
    let mut mi: gss_mech_info = 0 as *mut gss_mech_config;
    let mut i: size_t = 0;
    memset(&mut errinfo as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_open_plugin((*minfo).uLibName, &mut dl, &mut errinfo) !=
           0 as libc::c_int as libc::c_long ||
           errinfo.code != 0 as libc::c_int as libc::c_long {
        return
    }
    if !(krb5int_get_plugin_func(dl,
                                 b"gss_mech_interposer\x00" as *const u8 as
                                     *const libc::c_char,
                                 &mut isym as
                                     *mut Option<unsafe extern "C" fn(_:
                                                                          gss_OID)
                                                     -> gss_OID_set> as
                                     *mut Option<unsafe extern "C" fn()
                                                     -> ()>, &mut errinfo) !=
             0 as libc::c_int as libc::c_long) {
        /* Get a list of mechs to interpose. */
        list =
            Some(isym.expect("non-null function pointer")).expect("non-null function pointer")((*minfo).mech_type);
        if !list.is_null() {
            (*minfo).mech =
                build_interMech(dl as *mut libc::c_void, (*minfo).mech_type);
            if !(*minfo).mech.is_null() {
                (*minfo).freeMech = 1 as libc::c_int;
                /* Add interposer fields for each interposed mech. */
                i = 0 as libc::c_int as size_t;
                while i < (*list).count {
                    /* Skip this mech if it doesn't exist or is already
		 * interposed. */
                    oid =
                        &mut *(*list).elements.offset(i as isize) as
                            *mut gss_OID_desc_struct;
                    mi = searchMechList(oid as gss_const_OID);
                    if !(mi.is_null() || !(*mi).int_mech_type.is_null()) {
                        /* Construct a special OID to represent the interposed mech. */
                        (*mi).int_mech_type =
                            interposed_oid((*minfo).mech_type, oid);
                        if !(*mi).int_mech_type.is_null() {
                            /* Save an alias to the interposer's function table. */
                            (*mi).int_mech = (*minfo).mech
                        }
                    }
                    i = i.wrapping_add(1)
                }
                gss_release_oid_set(&mut min, &mut list);
                (*minfo).dl_handle = dl as *mut libc::c_void;
                dl = 0 as *mut plugin_file_handle
            }
        }
    }
    if !dl.is_null() { krb5int_close_plugin(dl); }
    k5_clear_error(&mut errinfo);
}
/*
 * Determine the mechanism to use for a caller-specified mech OID.  For the
 * real mech OID of an interposed mech, return the interposed OID.  For an
 * interposed mech OID (which an interposer mech uses when re-entering the
 * mechglue), return the real mech OID.  The returned OID is an alias and
 * should not be modified or freed.
 */
#[no_mangle]
#[c2rust::src_loc = "992:1"]
pub unsafe extern "C" fn gssint_select_mech_type(mut minor: *mut OM_uint32,
                                                 mut oid: gss_const_OID,
                                                 mut selected_oid:
                                                     *mut gss_OID)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut minfo: gss_mech_info = 0 as *mut gss_mech_config;
    let mut status: OM_uint32 = 0;
    *selected_oid = 0 as gss_OID;
    if gssint_mechglue_initialize_library() != 0 as libc::c_int {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    k5_mutex_lock(&mut g_mechListLock);
    /* Read conf file at least once so that interposer plugins have a
	 * chance of getting initialized. */
    initMechList();
    minfo = g_mechList;
    if oid.is_null() { oid = (*minfo).mech_type as gss_const_OID }
    loop  {
        if minfo.is_null() { current_block = 6057473163062296781; break ; }
        if (*(*minfo).mech_type).length == (*oid).length &&
               memcmp((*(*minfo).mech_type).elements, (*oid).elements,
                      (*(*minfo).mech_type).length as libc::c_ulong) ==
                   0 as libc::c_int {
            if !(*minfo).int_mech_type.is_null() {
                *selected_oid = (*minfo).int_mech_type
            } else { *selected_oid = (*minfo).mech_type }
            status = 0 as libc::c_int as OM_uint32;
            current_block = 2349180396633407496;
            break ;
        } else if !(*minfo).int_mech_type.is_null() &&
                      ((*(*minfo).int_mech_type).length == (*oid).length &&
                           memcmp((*(*minfo).int_mech_type).elements,
                                  (*oid).elements,
                                  (*(*minfo).int_mech_type).length as
                                      libc::c_ulong) == 0 as libc::c_int) {
            *selected_oid = (*minfo).mech_type;
            status = 0 as libc::c_int as OM_uint32;
            current_block = 2349180396633407496;
            break ;
        } else { minfo = (*minfo).next }
    }
    match current_block {
        6057473163062296781 => {
            status = (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        _ => { }
    }
    k5_mutex_unlock(&mut g_mechListLock);
    return status;
}
/* If oid is an interposed OID, return the corresponding real mech OID.  If
 * it's a real mech OID, return it unmodified.  Otherwised return null. */
#[no_mangle]
#[c2rust::src_loc = "1038:1"]
pub unsafe extern "C" fn gssint_get_public_oid(mut oid: gss_const_OID)
 -> gss_OID {
    let mut minfo: gss_mech_info = 0 as *mut gss_mech_config;
    let mut public_oid: gss_OID = 0 as gss_OID;
    /* if oid is null -> then get default which is the first in the list */
    if oid.is_null() { return 0 as gss_OID }
    if gssint_mechglue_initialize_library() != 0 as libc::c_int {
        return 0 as gss_OID
    }
    k5_mutex_lock(&mut g_mechListLock);
    minfo = g_mechList;
    while !minfo.is_null() {
        if !((*minfo).is_interposer != 0) {
            if (*(*minfo).mech_type).length == (*oid).length &&
                   memcmp((*(*minfo).mech_type).elements, (*oid).elements,
                          (*(*minfo).mech_type).length as libc::c_ulong) ==
                       0 as libc::c_int ||
                   !(*minfo).int_mech_type.is_null() &&
                       ((*(*minfo).int_mech_type).length == (*oid).length &&
                            memcmp((*(*minfo).int_mech_type).elements,
                                   (*oid).elements,
                                   (*(*minfo).int_mech_type).length as
                                       libc::c_ulong) == 0 as libc::c_int) {
                public_oid = (*minfo).mech_type;
                break ;
            }
        }
        minfo = (*minfo).next
    }
    k5_mutex_unlock(&mut g_mechListLock);
    return public_oid;
}
/* Translate a vector of oids (as from a union cred struct) into a set of
 * public OIDs using gssint_get_public_oid. */
#[no_mangle]
#[c2rust::src_loc = "1070:1"]
pub unsafe extern "C" fn gssint_make_public_oid_set(mut minor_status:
                                                        *mut OM_uint32,
                                                    mut oids: gss_OID,
                                                    mut count: libc::c_int,
                                                    mut public_set:
                                                        *mut gss_OID_set)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut set: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut public_oid: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut i: libc::c_int = 0;
    *public_set = 0 as gss_OID_set;
    status = generic_gss_create_empty_oid_set(minor_status, &mut set);
    if status &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        return status
    }
    i = 0 as libc::c_int;
    while i < count {
        public_oid =
            gssint_get_public_oid(&mut *oids.offset(i as isize) as
                                      *mut gss_OID_desc_struct as
                                      gss_const_OID);
        if !public_oid.is_null() {
            status =
                generic_gss_add_oid_set_member(minor_status,
                                               public_oid as
                                                   *const gss_OID_desc,
                                               &mut set);
            if status &
                   ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
                        |
                        (0o377 as libc::c_ulong as OM_uint32) <<
                            16 as libc::c_int) != 0 {
                generic_gss_release_oid_set(&mut tmpmin, &mut set);
                return status
            }
        }
        i += 1
    }
    *public_set = set;
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
/*
 * Register a mechanism.  Called with g_mechListLock held.
 */
/*
 * given the mechanism type, return the mechanism structure
 * containing the mechanism library entry points.
 * will return NULL if mech type is not found
 * This function will also trigger the loading of the mechanism
 * module if it has not been already loaded.
 */
#[no_mangle]
#[c2rust::src_loc = "1112:1"]
pub unsafe extern "C" fn gssint_get_mechanism(mut oid: gss_const_OID)
 -> gss_mechanism {
    let mut aMech: gss_mech_info = 0 as *mut gss_mech_config;
    let mut sym: Option<unsafe extern "C" fn(_: gss_OID) -> gss_mechanism> =
        None;
    let mut dl: *mut plugin_file_handle = 0 as *mut plugin_file_handle;
    let mut errinfo: errinfo = errinfo{code: 0, msg: 0 as *mut libc::c_char,};
    if gssint_mechglue_initialize_library() != 0 as libc::c_int {
        return 0 as gss_mechanism
    }
    k5_mutex_lock(&mut g_mechListLock);
    /* Check if the mechanism is already loaded. */
    aMech = g_mechList;
    if oid.is_null() { oid = (*aMech).mech_type as gss_const_OID }
    while !aMech.is_null() {
        if (*(*aMech).mech_type).length == (*oid).length &&
               memcmp((*(*aMech).mech_type).elements, (*oid).elements,
                      (*(*aMech).mech_type).length as libc::c_ulong) ==
                   0 as libc::c_int && !(*aMech).mech.is_null() {
            k5_mutex_unlock(&mut g_mechListLock);
            return (*aMech).mech
        } else {
            if !(*aMech).int_mech_type.is_null() &&
                   ((*(*aMech).int_mech_type).length == (*oid).length &&
                        memcmp((*(*aMech).int_mech_type).elements,
                               (*oid).elements,
                               (*(*aMech).int_mech_type).length as
                                   libc::c_ulong) == 0 as libc::c_int) {
                k5_mutex_unlock(&mut g_mechListLock);
                return (*aMech).int_mech
            }
        }
        aMech = (*aMech).next
    }
    /*
	 * might need to re-read the configuration file before loading
	 * the mechanism to ensure we have the latest info.
	 */
    updateMechList();
    aMech = searchMechList(oid);
    /* is the mechanism present in the list ? */
    if aMech.is_null() {
        k5_mutex_unlock(&mut g_mechListLock);
        return 0 as *mut libc::c_void as gss_mechanism
    }
    /* has another thread loaded the mech */
    if !(*aMech).mech.is_null() {
        k5_mutex_unlock(&mut g_mechListLock);
        return (*aMech).mech
    }
    memset(&mut errinfo as *mut errinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<errinfo>() as libc::c_ulong);
    if krb5int_open_plugin((*aMech).uLibName, &mut dl, &mut errinfo) !=
           0 as libc::c_int as libc::c_long ||
           errinfo.code != 0 as libc::c_int as libc::c_long {
        k5_clear_error(&mut errinfo);
        k5_mutex_unlock(&mut g_mechListLock);
        return 0 as *mut libc::c_void as gss_mechanism
    }
    if krb5int_get_plugin_func(dl,
                               b"gss_mech_initialize\x00" as *const u8 as
                                   *const libc::c_char,
                               &mut sym as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        gss_OID)
                                                   -> gss_mechanism> as
                                   *mut Option<unsafe extern "C" fn() -> ()>,
                               &mut errinfo) ==
           0 as libc::c_int as libc::c_long {
        /* Call the symbol to get the mechanism table */
        (*aMech).mech =
            Some(sym.expect("non-null function pointer")).expect("non-null function pointer")((*aMech).mech_type)
    } else {
        /* Try dynamic dispatch table */
        k5_clear_error(&mut errinfo);
        (*aMech).mech =
            build_dynamicMech(dl as *mut libc::c_void, (*aMech).mech_type);
        (*aMech).freeMech = 1 as libc::c_int
    }
    if (*aMech).mech.is_null() {
        krb5int_close_plugin(dl);
        k5_mutex_unlock(&mut g_mechListLock);
        return 0 as *mut libc::c_void as gss_mechanism
    }
    (*aMech).dl_handle = dl as *mut libc::c_void;
    k5_mutex_unlock(&mut g_mechListLock);
    return (*aMech).mech;
}
/* gssint_get_mechanism */
/*
 * this routine is used for searching the list of mechanism data.
 *
 * this needs to be called with g_mechListLock held.
 */
#[c2rust::src_loc = "1197:1"]
unsafe extern "C" fn searchMechList(mut oid: gss_const_OID) -> gss_mech_info {
    let mut aMech: gss_mech_info = g_mechList;
    /* if oid is null -> then get default which is the first in the list */
    if oid.is_null() { return aMech }
    while !aMech.is_null() {
        if (*(*aMech).mech_type).length == (*oid).length &&
               memcmp((*(*aMech).mech_type).elements, (*oid).elements,
                      (*(*aMech).mech_type).length as libc::c_ulong) ==
                   0 as libc::c_int {
            return aMech
        }
        aMech = (*aMech).next
    }
    /* none found */
    return 0 as *mut libc::c_void as gss_mech_info;
}
/* searchMechList */
/* Return the first non-whitespace character starting from str. */
#[c2rust::src_loc = "1216:1"]
unsafe extern "C" fn skip_whitespace(mut str: *mut libc::c_char)
 -> *mut libc::c_char {
    while *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as
              libc::c_int &
              _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        str = str.offset(1)
    }
    return str;
}
/* Truncate str at the first whitespace character and return the first
 * non-whitespace character after that point. */
#[c2rust::src_loc = "1226:1"]
unsafe extern "C" fn delimit_ws(mut str: *mut libc::c_char)
 -> *mut libc::c_char {
    while *str as libc::c_int != '\u{0}' as i32 &&
              *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as
                  libc::c_int &
                  _ISspace as libc::c_int as libc::c_ushort as libc::c_int ==
                  0 {
        str = str.offset(1)
    }
    if *str as libc::c_int != '\u{0}' as i32 {
        let fresh1 = str;
        str = str.offset(1);
        *fresh1 = '\u{0}' as i32 as libc::c_char
    }
    return skip_whitespace(str);
}
/* Truncate str at the first occurrence of delimiter and return the first
 * non-whitespace character after that point. */
#[c2rust::src_loc = "1238:1"]
unsafe extern "C" fn delimit(mut str: *mut libc::c_char,
                             mut delimiter: libc::c_char)
 -> *mut libc::c_char {
    while *str as libc::c_int != '\u{0}' as i32 &&
              *str as libc::c_int != delimiter as libc::c_int {
        str = str.offset(1)
    }
    if *str as libc::c_int != '\u{0}' as i32 {
        let fresh2 = str;
        str = str.offset(1);
        *fresh2 = '\u{0}' as i32 as libc::c_char
    }
    return skip_whitespace(str);
}
/*
 * loads the configuration file
 * this is called while having a mutex lock on the mechanism list
 * entries for libraries that have been loaded can't be modified
 * mechNameStr and mech_type fields are not updated during updates
 */
#[c2rust::src_loc = "1254:1"]
unsafe extern "C" fn loadConfigFile(mut fileName: *const libc::c_char) {
    let mut sharedLib: *mut libc::c_char = 0 as *mut libc::c_char; /* while */
    let mut kernMod: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut modOptions: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut modType: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer: [libc::c_char; 8192] = [0; 8192];
    let mut oidStr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut confFile: *mut FILE = 0 as *mut FILE;
    confFile = fopen(fileName, b"r\x00" as *const u8 as *const libc::c_char);
    if confFile.is_null() { return }
    memset(buffer.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong);
    while !fgets(buffer.as_mut_ptr(), 8192 as libc::c_int, confFile).is_null()
          {
        /* ignore lines beginning with # */
        if *buffer.as_mut_ptr() as libc::c_int == '#' as i32 { continue ; }
        /* Parse out the name, oid, and shared library path. */
        oidStr = buffer.as_mut_ptr();
        oid = delimit_ws(oidStr);
        if *oid as libc::c_int == '\u{0}' as i32 { continue ; }
        sharedLib = delimit_ws(oid);
        if *sharedLib as libc::c_int == '\u{0}' as i32 { continue ; }
        next = delimit_ws(sharedLib);
        /* Parse out the kernel module name if present. */
        if *next as libc::c_int != '\u{0}' as i32 &&
               *next as libc::c_int != '[' as i32 &&
               *next as libc::c_int != '<' as i32 {
            kernMod = next;
            next = delimit_ws(kernMod)
        } else { kernMod = 0 as *mut libc::c_char }
        /* Parse out the module options if present. */
        if *next as libc::c_int == '[' as i32 {
            modOptions = next.offset(1 as libc::c_int as isize);
            next = delimit(modOptions, ']' as i32 as libc::c_char)
        } else { modOptions = 0 as *mut libc::c_char }
        /* Parse out the module type if present. */
        if *next as libc::c_int == '<' as i32 {
            modType = next.offset(1 as libc::c_int as isize);
            delimit(modType, '>' as i32 as libc::c_char);
        } else { modType = 0 as *mut libc::c_char }
        addConfigEntry(oidStr, oid, sharedLib, kernMod, modOptions, modType);
    }
    fclose(confFile);
}
/* Local functions */
/* loadConfigFile */
#[c2rust::src_loc = "1473:1"]
unsafe extern "C" fn addConfigEntry(mut oidStr: *const libc::c_char,
                                    mut oid: *const libc::c_char,
                                    mut sharedLib: *const libc::c_char,
                                    mut kernMod: *const libc::c_char,
                                    mut modOptions: *const libc::c_char,
                                    mut modType: *const libc::c_char) {
    let mut sharedPath: [libc::c_char; 8212] = [0; 8212];
    let mut tmpStr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mechOid: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut aMech: gss_mech_info = 0 as *mut gss_mech_config;
    let mut tmp: gss_mech_info = 0 as *mut gss_mech_config;
    let mut minor: OM_uint32 = 0;
    let mut oidBuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    if oid.is_null() || oidStr.is_null() { return }
    /*
	 * check if an entry for this oid already exists
	 * if it does, and the library is already loaded then
	 * we can't modify it, so skip it
	 */
    oidBuf.value = oid as *mut libc::c_void;
    oidBuf.length = strlen(oid);
    if generic_gss_str_to_oid(&mut minor, &mut oidBuf, &mut mechOid) !=
           0 as libc::c_int as libc::c_uint {
        return
    }
    aMech = searchMechList(mechOid as gss_const_OID);
    if !aMech.is_null() && !(*aMech).mech.is_null() {
        generic_gss_release_oid(&mut minor, &mut mechOid);
        return
    }
    /*
	 * If that's all, then this is a corrupt entry. Skip it.
	 */
    if *sharedLib == 0 {
        generic_gss_release_oid(&mut minor, &mut mechOid);
        return
    }
    if *sharedLib.offset(0 as libc::c_int as isize) as libc::c_int ==
           '/' as i32 {
        snprintf(sharedPath.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 8212]>() as
                     libc::c_ulong,
                 b"%s\x00" as *const u8 as *const libc::c_char, sharedLib);
    } else {
        snprintf(sharedPath.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 8212]>() as
                     libc::c_ulong,
                 b"%s%s\x00" as *const u8 as *const libc::c_char,
                 b"/usr/local/lib/gss/\x00" as *const u8 as
                     *const libc::c_char, sharedLib);
    }
    /*
	 * are we creating a new mechanism entry or
	 * just modifying existing (non loaded) mechanism entry
	 */
    if !aMech.is_null() {
        /*
		 * delete any old values and set new
		 * mechNameStr and mech_type are not modified
		 */
        if !(*aMech).kmodName.is_null() {
            free((*aMech).kmodName as *mut libc::c_void);
            (*aMech).kmodName = 0 as *mut libc::c_char
        }
        if !(*aMech).optionStr.is_null() {
            free((*aMech).optionStr as *mut libc::c_void);
            (*aMech).optionStr = 0 as *mut libc::c_char
        }
        tmpStr = strdup(sharedPath.as_mut_ptr());
        if !tmpStr.is_null() {
            if !(*aMech).uLibName.is_null() {
                free((*aMech).uLibName as *mut libc::c_void);
            }
            (*aMech).uLibName = tmpStr
        }
        if !kernMod.is_null() {
            /* this is an optional parameter */
            (*aMech).kmodName = strdup(kernMod)
        }
        if !modOptions.is_null() {
            /* optional module options */
            (*aMech).optionStr = strdup(modOptions)
        }
        /* the oid is already set */
        generic_gss_release_oid(&mut minor, &mut mechOid);
        return
    }
    /* adding a new entry */
    aMech =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<gss_mech_config>() as libc::c_ulong) as
            gss_mech_info;
    if aMech.is_null() {
        generic_gss_release_oid(&mut minor, &mut mechOid);
        return
    }
    (*aMech).mech_type = mechOid;
    (*aMech).uLibName = strdup(sharedPath.as_mut_ptr());
    (*aMech).mechNameStr = strdup(oidStr);
    (*aMech).freeMech = 0 as libc::c_int;
    /* check if any memory allocations failed - bad news */
    if (*aMech).uLibName.is_null() || (*aMech).mechNameStr.is_null() {
        if !(*aMech).uLibName.is_null() {
            free((*aMech).uLibName as *mut libc::c_void);
        }
        if !(*aMech).mechNameStr.is_null() {
            free((*aMech).mechNameStr as *mut libc::c_void);
        }
        generic_gss_release_oid(&mut minor, &mut mechOid);
        free(aMech as *mut libc::c_void);
        return
    }
    if !kernMod.is_null() {
        /* this is an optional parameter */
        (*aMech).kmodName = strdup(kernMod)
    }
    if !modOptions.is_null() { (*aMech).optionStr = strdup(modOptions) }
    if !modType.is_null() &&
           strcmp(modType,
                  b"interposer\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        (*aMech).is_interposer = 1 as libc::c_int
    }
    /*
	 * add the new entry to the end of the list - make sure
	 * that only complete entries are added because other
	 * threads might currently be searching the list.
	 */
    tmp = g_mechListTail;
    g_mechListTail = aMech;
    if !tmp.is_null() { (*tmp).next = aMech }
    if g_mechList.is_null() { g_mechList = aMech };
}
