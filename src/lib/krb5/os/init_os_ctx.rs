use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:29"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:29"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:29"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:29"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:29"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::__uint64_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:29"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    use super::pthreadtypes_h::pthread_mutex_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/util/profile/prof_int.h:32"]
pub mod prof_int_h {
    /*
 * This structure defines the high-level, user visible profile_t
 * object, which is used as a handle by users who need to query some
 * configuration file(s)
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "93:8"]
    pub struct _profile_t {
        pub magic: prf_magic_t,
        pub first_file: prf_file_t,
        pub vt: *mut profile_vtable,
        pub cbdata: *mut libc::c_void,
        pub lib_handle: prf_lib_handle_t,
    }
    #[c2rust::src_loc = "86:1"]
    pub type prf_lib_handle_t = *mut _prf_lib_handle_t;
    /*
 * The profile flags
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "80:8"]
    pub struct _prf_lib_handle_t {
        pub lock: k5_mutex_t,
        pub refcount: libc::c_int,
        pub plugin_handle: *mut plugin_file_handle,
    }
    #[c2rust::src_loc = "71:1"]
    pub type prf_file_t = *mut _prf_file_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "65:8"]
    pub struct _prf_file_t {
        pub magic: prf_magic_t,
        pub data: *mut _prf_data_t,
        pub next: *mut _prf_file_t,
    }
    /*
 * This is the structure which stores the profile information for a
 * particular configuration file.
 *
 * Locking strategy:
 * - filespec, fslen are fixed after creation
 * - refcount and next should only be tweaked with the global lock held
 * - other fields can be tweaked after grabbing the in-struct lock
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:8"]
    pub struct _prf_data_t {
        pub magic: prf_magic_t,
        pub lock: k5_mutex_t,
        pub root: *mut profile_node,
        pub last_stat: time_t,
        pub timestamp: time_t,
        pub frac_ts: libc::c_ulong,
        pub flags: libc::c_int,
        pub upd_serial: libc::c_int,
        pub fslen: size_t,
        pub pad: C2RustUnnamed,
        pub refcount: libc::c_int,
        pub next: *mut _prf_data_t,
        pub filespec: [libc::c_char; 15],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:2"]
    pub union C2RustUnnamed {
        pub d: libc::c_double,
        pub p: *mut libc::c_void,
        pub ll: uint64_t,
        pub m: k5_mutex_t,
    }
    /*
 * prof-int.h
 */
    #[c2rust::src_loc = "19:1"]
    pub type prf_magic_t = libc::c_long;
    use super::profile_h::{profile_vtable, profile_t};
    use super::k5_thread_h::k5_mutex_t;
    use super::k5_plugin_h::plugin_file_handle;
    use super::time_t_h::time_t;
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint64_t;
    use super::com_err_h::errcode_t;
    extern "C" {
        #[c2rust::src_loc = "33:9"]
        pub type profile_node;
        /* prof_file.c */
        #[no_mangle]
        #[c2rust::src_loc = "211:1"]
        pub fn profile_copy(_: profile_t, _: *mut profile_t) -> errcode_t;
    }
    /* prof_set.c -- included from profile.h */
    /* Others included from profile.h */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:29"]
pub mod k5_plugin_h {
    /* opaque */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:8"]
    pub struct plugin_dir_handle {
        pub files: *mut *mut plugin_file_handle,
    }
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
        #[c2rust::src_loc = "106:1"]
        pub fn krb5int_close_plugin_dirs(_: *mut plugin_dir_handle);
    }
    /* K5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:29"]
pub mod profile_h {
    /*
 * profile.h
 */
    /* Used by profile_init_flags(). */
    /* Allow module declaration */
    /*
 * Used by the profile iterator in prof_get.c
 */
    /* __cplusplus */
    /* path as C string */
    /* list of : separated paths, C string */
    /* path as C string */
    /* list of : separated paths, C string */
    /*
 * profile_init_vtable allows a caller to create a profile-compatible object
 * with a different back end.
 */
    /*
 * Mandatory: Look up all of the relations for names, placing the resulting
 * values in *ret_values.  If no relations exist, return PROF_NO_RELATION, or
 * PROF_NO_SECTION to indicate that one of the intermediate names does not
 * exist as a section.  The list will be freed with free_values.
 */
    /* Mandatory: Free a list of strings returned by get_values. */
    /* Optional: Release any data associated with the profile. */
    /*
 * Optional (mandatory if cleanup is defined): Generate a new cbdata pointer
 * for a copy of the profile.  If not implemented, the new profile will receive
 * the same cbdata pointer as the old one.
 */
    /*
 * Optional: Create an iterator handle.
 *
 * If flags contains PROFILE_ITER_LIST_SECTION, iterate over all of the
 * relations and sections within names.  Otherwise, iterate over the relation
 * values for names, or produce a single section result if names is a section.
 *
 * If flags contains PROFILE_ITER_SECTIONS_ONLY, produce only sections.
 *
 * If flags contains PROFILE_ITER_RELATIONS_ONLY, produce only relations.
 */
    /*
 * Optional (mandatory if iterator_create is defined): Produce the next
 * relation or section in an iteration.  If producing a section result, set
 * *ret_value to NULL.  The returned strings will be freed with free_string.
 */
    /*
 * Optional (mandatory if iterator_create is defined): Free the memory for an
 * iterator.
 */
    /* Optional (mandatory if iterator is defined): Free a string value. */
    /*
 * Optional: Determine if a profile is writable.  If not implemented, the
 * profile is never writable.
 */
    /*
 * Optional: Determine if a profile is modified in memory relative to the
 * persistent store.  If not implemented, the profile is assumed to never be
 * modified.
 */
    /*
 * Optional: Change the value of a relation, or remove it if new_value is NULL.
 * If old_value is set and the relation does not have that value, return
 * PROF_NO_RELATION.
 */
    /*
 * Optional: Rename a section to new_name, or remove the section if new_name is
 * NULL.
 */
    /*
 * Optional: Add a new relation, or a new section if new_value is NULL.  Add
 * any intermediate sections as necessary.
 */
    /*
 * Optional: Flush any pending memory updates to the persistent store.  If
 * implemented, this function will be called by profile_release as well as
 * profile_flush, so make sure it's not inefficient to flush an unmodified
 * profile.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "242:8"]
    pub struct profile_vtable {
        pub minor_ver: libc::c_int,
        pub get_values: profile_get_values_fn,
        pub free_values: profile_free_values_fn,
        pub cleanup: profile_cleanup_fn,
        pub copy: profile_copy_fn,
        pub iterator_create: profile_iterator_create_fn,
        pub iterator: profile_iterator_fn,
        pub iterator_free: profile_iterator_free_fn,
        pub free_string: profile_free_string_fn,
        pub writable: profile_writable_fn,
        pub modified: profile_modified_fn,
        pub update_relation: profile_update_relation_fn,
        pub rename_section: profile_rename_section_fn,
        pub add_relation: profile_add_relation_fn,
        pub flush: profile_flush_fn,
    }
    #[c2rust::src_loc = "239:1"]
    pub type profile_flush_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_long>;
    #[c2rust::src_loc = "229:1"]
    pub type profile_add_relation_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *const libc::c_char,
                                    _: *const libc::c_char) -> libc::c_long>;
    #[c2rust::src_loc = "221:1"]
    pub type profile_rename_section_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *const libc::c_char,
                                    _: *const libc::c_char) -> libc::c_long>;
    #[c2rust::src_loc = "213:1"]
    pub type profile_update_relation_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *const libc::c_char,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char) -> libc::c_long>;
    #[c2rust::src_loc = "205:1"]
    pub type profile_modified_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_int)
                   -> libc::c_long>;
    #[c2rust::src_loc = "197:1"]
    pub type profile_writable_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_int)
                   -> libc::c_long>;
    #[c2rust::src_loc = "190:1"]
    pub type profile_free_string_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut libc::c_char) -> ()>;
    #[c2rust::src_loc = "186:1"]
    pub type profile_iterator_free_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "178:1"]
    pub type profile_iterator_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut *mut libc::c_char,
                                    _: *mut *mut libc::c_char)
                   -> libc::c_long>;
    #[c2rust::src_loc = "169:1"]
    pub type profile_iterator_create_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const *const libc::c_char,
                                    _: libc::c_int, _: *mut *mut libc::c_void)
                   -> libc::c_long>;
    #[c2rust::src_loc = "155:1"]
    pub type profile_copy_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *mut libc::c_void)
                   -> libc::c_long>;
    #[c2rust::src_loc = "147:1"]
    pub type profile_cleanup_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "143:1"]
    pub type profile_free_values_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *mut libc::c_char) -> ()>;
    #[c2rust::src_loc = "138:1"]
    pub type profile_get_values_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const *const libc::c_char,
                                    _: *mut *mut *mut libc::c_char)
                   -> libc::c_long>;
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    #[c2rust::src_loc = "40:1"]
    pub type profile_filespec_t = *mut libc::c_char;
    #[c2rust::src_loc = "42:1"]
    pub type const_profile_filespec_t = *const libc::c_char;
    use super::prof_int_h::_profile_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "45:1"]
        pub fn profile_init(files: *mut const_profile_filespec_t,
                            ret_profile: *mut profile_t) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "48:1"]
        pub fn profile_init_flags(files: *mut const_profile_filespec_t,
                                  flags: libc::c_int,
                                  ret_profile: *mut profile_t)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn profile_release(profile: profile_t);
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:29"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    #[c2rust::src_loc = "206:1"]
    pub type krb5_magic = krb5_error_code;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:16"]
    pub struct _krb5_data {
        pub magic: krb5_magic,
        pub length: libc::c_uint,
        pub data: *mut libc::c_char,
    }
    #[c2rust::src_loc = "208:1"]
    pub type krb5_data = _krb5_data;
    #[c2rust::src_loc = "8510:1"]
    pub type krb5_post_recv_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_error_code, _: *const krb5_data,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    /* per Kerberos v5 protocol spec */
    /* not yet in the spec... */
    /* macros to determine if a type is a local type */
    /*
 * end "hostaddr.h"
 */
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    #[c2rust::src_loc = "8475:1"]
    pub type krb5_pre_send_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "8391:1"]
    pub type krb5_trace_callback
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *const krb5_trace_info,
                                    _: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "8387:1"]
    pub type krb5_trace_info = _krb5_trace_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8387:16"]
    pub struct _krb5_trace_info {
        pub message: *const libc::c_char,
    }
    #[c2rust::src_loc = "7865:1"]
    pub type krb5_prompt_type = krb5_int32;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:29"]
pub mod k5_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1208:8"]
    pub struct _krb5_context {
        pub magic: krb5_magic,
        pub in_tkt_etypes: *mut krb5_enctype,
        pub tgs_etypes: *mut krb5_enctype,
        pub os_context: _krb5_os_context,
        pub default_realm: *mut libc::c_char,
        pub profile: profile_t,
        pub dal_handle: *mut kdb5_dal_handle,
        pub clockskew: krb5_deltat,
        pub kdc_default_options: krb5_flags,
        pub library_options: krb5_flags,
        pub profile_secure: krb5_boolean,
        pub fcc_default_format: libc::c_int,
        pub prompt_types: *mut krb5_prompt_type,
        pub udp_pref_limit: libc::c_int,
        pub use_conf_ktypes: krb5_boolean,
        pub libkrb5_plugins: plugin_dir_handle,
        pub preauth_context: krb5_preauth_context,
        pub ccselect_handles: *mut *mut ccselect_module_handle,
        pub localauth_handles: *mut *mut localauth_module_handle,
        pub hostrealm_handles: *mut *mut hostrealm_module_handle,
        pub tls: *mut k5_tls_vtable_st,
        pub err: errinfo,
        pub err_fmt: *mut libc::c_char,
        pub kdblog_context: *mut _kdb_log_context,
        pub allow_weak_crypto: krb5_boolean,
        pub ignore_acceptor_hostname: krb5_boolean,
        pub enforce_ok_as_delegate: krb5_boolean,
        pub dns_canonicalize_hostname: dns_canonhost,
        pub trace_callback: krb5_trace_callback,
        pub trace_callback_data: *mut libc::c_void,
        pub kdc_send_hook: krb5_pre_send_fn,
        pub kdc_send_hook_data: *mut libc::c_void,
        pub kdc_recv_hook: krb5_post_recv_fn,
        pub kdc_recv_hook_data: *mut libc::c_void,
        pub plugins: [plugin_interface; 13],
        pub plugin_base_dir: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1137:8"]
    pub struct plugin_interface {
        pub modules: *mut *mut plugin_mapping,
        pub configured: krb5_boolean,
    }
    #[c2rust::src_loc = "1194:1"]
    pub type dns_canonhost = libc::c_uint;
    #[c2rust::src_loc = "1197:5"]
    pub const CANONHOST_FALLBACK: dns_canonhost = 2;
    #[c2rust::src_loc = "1196:5"]
    pub const CANONHOST_TRUE: dns_canonhost = 1;
    #[c2rust::src_loc = "1195:5"]
    pub const CANONHOST_FALSE: dns_canonhost = 0;
    #[c2rust::src_loc = "1203:1"]
    pub type krb5_preauth_context = *mut krb5_preauth_context_st;
    #[c2rust::src_loc = "1201:1"]
    pub type kdb5_dal_handle = _kdb5_dal_handle;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "702:16"]
    pub struct _krb5_os_context {
        pub magic: krb5_magic,
        pub time_offset: krb5_int32,
        pub usec_offset: krb5_int32,
        pub os_flags: krb5_int32,
        pub default_ccname: *mut libc::c_char,
    }
    #[c2rust::src_loc = "702:1"]
    pub type krb5_os_context = *mut _krb5_os_context;
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
        #[c2rust::src_loc = "1202:8"]
        pub type _kdb_log_context;
        #[c2rust::src_loc = "1207:8"]
        pub type k5_tls_vtable_st;
        #[c2rust::src_loc = "1206:8"]
        pub type hostrealm_module_handle;
        #[c2rust::src_loc = "1205:8"]
        pub type localauth_module_handle;
        #[c2rust::src_loc = "1204:8"]
        pub type ccselect_module_handle;
        #[c2rust::src_loc = "1203:16"]
        pub type krb5_preauth_context_st;
        #[c2rust::src_loc = "1200:8"]
        pub type _kdb5_dal_handle;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:29"]
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
    /* K5_ERR_H */
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
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/usr/include/stdlib.h:29"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "639:1"]
        pub fn secure_getenv(__name: *const libc::c_char)
         -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:29"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "124:14"]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/int-proto.h:31"]
pub mod int_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "224:1"]
        pub fn k5_free_preauth_context(context: krb5_context);
    }
    /* KRB5_INT_FUNC_PROTO__ */
}
pub use self::types_h::{__int32_t, __uint64_t, __time_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::stdint_uintn_h::uint64_t;
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t};
pub use self::prof_int_h::{_profile_t, prf_lib_handle_t, _prf_lib_handle_t,
                           prf_file_t, _prf_file_t, _prf_data_t,
                           C2RustUnnamed, prf_magic_t, profile_node,
                           profile_copy};
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle,
                            krb5int_close_plugin_dirs};
pub use self::profile_h::{profile_vtable, profile_flush_fn,
                          profile_add_relation_fn, profile_rename_section_fn,
                          profile_update_relation_fn, profile_modified_fn,
                          profile_writable_fn, profile_free_string_fn,
                          profile_iterator_free_fn, profile_iterator_fn,
                          profile_iterator_create_fn, profile_copy_fn,
                          profile_cleanup_fn, profile_free_values_fn,
                          profile_get_values_fn, profile_t,
                          profile_filespec_t, const_profile_filespec_t,
                          profile_init, profile_init_flags, profile_release};
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_enctype, krb5_flags,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_os_context, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::com_err_h::errcode_t;
use self::stdlib_h::{malloc, free, secure_getenv};
use self::string_h::{strlen, strchr, strdup, strncpy, memcpy};
use self::int_proto_h::k5_free_preauth_context;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/os/init_os_ctx.c */
/*
 * Copyright 1994, 2007, 2008, 2009 by the Massachusetts Institute of Technology.
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
/* _WIN32 */
#[c2rust::src_loc = "222:1"]
unsafe extern "C" fn free_filespecs(mut files: *mut profile_filespec_t) {
    let mut cp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if files.is_null() { return }
    cp = files;
    while !(*cp).is_null() {
        free(*cp as *mut libc::c_void);
        cp = cp.offset(1)
    }
    free(files as *mut libc::c_void);
}
/* This function is needed by KfM's KerberosPreferences API
 * because it needs to be able to specify "secure" */
#[c2rust::src_loc = "237:1"]
unsafe extern "C" fn os_get_default_config_files(mut pfiles:
                                                     *mut *mut profile_filespec_t,
                                                 mut secure: krb5_boolean)
 -> krb5_error_code {
    let mut files: *mut profile_filespec_t = 0 as *mut profile_filespec_t;
    /* !_WIN32 */
    let mut filepath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n_entries: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ent_len: libc::c_uint = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut t: *const libc::c_char = 0 as *const libc::c_char;
    if secure != 0 {
        filepath =
            b"/etc/krb5.conf:/usr/local/etc/krb5.conf\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char
    } else {
        filepath =
            secure_getenv(b"KRB5_CONFIG\x00" as *const u8 as
                              *const libc::c_char);
        if filepath.is_null() {
            filepath =
                b"/etc/krb5.conf:/usr/local/etc/krb5.conf\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
    }
    /* count the distinct filename components */
    s = filepath;
    n_entries = 1 as libc::c_int;
    while *s != 0 {
        if *s as libc::c_int == ':' as i32 { n_entries += 1 }
        s = s.offset(1)
    }
    /* the array is NULL terminated */
    files =
        malloc(((n_entries + 1 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *mut libc::c_char;
    if files.is_null() { return 12 as libc::c_int }
    /* measure, copy, and skip each one */
    s = filepath;
    i = 0 as libc::c_int;
    loop  {
        t = strchr(s, ':' as i32);
        if !(!t.is_null() ||
                 { t = s.offset(strlen(s) as isize); !t.is_null() }) {
            break ;
        }
        ent_len = t.wrapping_offset_from(s) as libc::c_long as libc::c_uint;
        let ref mut fresh0 = *files.offset(i as isize);
        *fresh0 =
            malloc(ent_len.wrapping_add(1 as libc::c_int as libc::c_uint) as
                       libc::c_ulong) as *mut libc::c_char;
        if (*files.offset(i as isize)).is_null() {
            loop 
                 /* if malloc fails, free the ones that worked */
                 {
                i -= 1;
                if !(i >= 0 as libc::c_int) { break ; }
                free(*files.offset(i as isize) as *mut libc::c_void);
            }
            free(files as *mut libc::c_void);
            return 12 as libc::c_int
        }
        strncpy(*files.offset(i as isize), s, ent_len as libc::c_ulong);
        *(*files.offset(i as isize)).offset(ent_len as isize) =
            0 as libc::c_int as libc::c_char;
        if *t as libc::c_int == 0 as libc::c_int {
            i += 1;
            break ;
        } else { s = t.offset(1 as libc::c_int as isize); i += 1 }
    }
    /* cap the array */
    let ref mut fresh1 = *files.offset(i as isize);
    *fresh1 = 0 as profile_filespec_t;
    /* !_WIN32 */
    *pfiles = files;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "340:1"]
unsafe extern "C" fn add_kdc_config_file(mut pfiles:
                                             *mut *mut profile_filespec_t)
 -> krb5_error_code {
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut newfiles: *mut profile_filespec_t = 0 as *mut profile_filespec_t;
    file =
        secure_getenv(b"KRB5_KDC_PROFILE\x00" as *const u8 as
                          *const libc::c_char);
    if file.is_null() {
        file =
            b"/usr/local/var/krb5kdc/kdc.conf\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char
    }
    count = 0 as libc::c_int as size_t;
    while !(*(*pfiles).offset(count as isize)).is_null() {
        count = count.wrapping_add(1)
    }
    count =
        (count as
             libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as
            size_t as size_t;
    newfiles =
        malloc(count.wrapping_mul(::std::mem::size_of::<profile_filespec_t>()
                                      as libc::c_ulong)) as
            *mut profile_filespec_t;
    if newfiles.is_null() { return 12 as libc::c_int }
    memcpy(newfiles.offset(1 as libc::c_int as isize) as *mut libc::c_void,
           *pfiles as *const libc::c_void,
           count.wrapping_sub(1 as libc::c_int as
                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<profile_filespec_t>()
                                                                  as
                                                                  libc::c_ulong));
    let ref mut fresh2 = *newfiles.offset(0 as libc::c_int as isize);
    *fresh2 = strdup(file);
    if (*newfiles.offset(0 as libc::c_int as isize)).is_null() {
        let mut e: libc::c_int = 12 as libc::c_int;
        free(newfiles as *mut libc::c_void);
        return e
    }
    free(*pfiles as *mut libc::c_void);
    *pfiles = newfiles;
    return 0 as libc::c_int;
}
/* Set the profile paths in the context.  If secure is set to TRUE
   then do not include user paths (from environment variables, etc).
   If kdc is TRUE, include kdc.conf from whereever we expect to find
   it.  */
#[c2rust::src_loc = "374:1"]
unsafe extern "C" fn os_init_paths(mut ctx: krb5_context,
                                   mut kdc: krb5_boolean) -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut files: *mut profile_filespec_t = 0 as *mut profile_filespec_t;
    let mut secure: krb5_boolean = (*ctx).profile_secure;
    retval = os_get_default_config_files(&mut files, secure);
    if retval == 0 as libc::c_int && kdc != 0 {
        retval = add_kdc_config_file(&mut files)
    }
    if retval == 0 {
        retval =
            profile_init_flags(files as *mut const_profile_filespec_t,
                               0x1 as libc::c_int, &mut (*ctx).profile) as
                krb5_error_code;
        /* If none of the filenames can be opened, use an empty profile. */
        if retval == 2 as libc::c_int {
            retval =
                profile_init(0 as *mut const_profile_filespec_t,
                             &mut (*ctx).profile) as krb5_error_code
        }
    }
    if !files.is_null() { free_filespecs(files); }
    if retval != 0 { (*ctx).profile = 0 as profile_t }
    if retval == 2 as libc::c_int {
        return -(1765328249 as libc::c_long) as krb5_error_code
    }
    if retval as libc::c_long == -(1429577715 as libc::c_long) ||
           retval as libc::c_long == -(1429577714 as libc::c_long) ||
           retval as libc::c_long == -(1429577713 as libc::c_long) ||
           retval as libc::c_long == -(1429577712 as libc::c_long) ||
           retval as libc::c_long == -(1429577711 as libc::c_long) {
        return -(1765328248 as libc::c_long) as krb5_error_code
    }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "414:1"]
pub unsafe extern "C" fn k5_os_init_context(mut ctx: krb5_context,
                                            mut profile: profile_t,
                                            mut flags: krb5_flags)
 -> krb5_error_code {
    let mut os_ctx: krb5_os_context = 0 as *mut _krb5_os_context;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    /* _WIN32 */
    os_ctx = &mut (*ctx).os_context;
    (*os_ctx).magic = -(1760647387 as libc::c_long) as krb5_magic;
    (*os_ctx).time_offset = 0 as libc::c_int;
    (*os_ctx).usec_offset = 0 as libc::c_int;
    (*os_ctx).os_flags = 0 as libc::c_int;
    (*os_ctx).default_ccname = 0 as *mut libc::c_char;
    (*ctx).libkrb5_plugins.files = 0 as *mut *mut plugin_file_handle;
    (*ctx).preauth_context = 0 as krb5_preauth_context;
    /* Use the profile we were handed, or create one from config files. */
    if !profile.is_null() {
        retval = profile_copy(profile, &mut (*ctx).profile) as krb5_error_code
    } else {
        retval =
            os_init_paths(ctx,
                          (flags & 0x2 as libc::c_int != 0 as libc::c_int) as
                              libc::c_int as krb5_boolean)
    }
    if retval != 0 { return retval }
    /* _WIN32 */
    return 0 as libc::c_int;
}
/* *
 * Set the default credential cache name.
 *
 * @param [in] context          Library context
 * @param [in] name             Default credential cache name or NULL
 *
 * Set the default credential cache name to @a name for future operations using
 * @a context.  If @a name is NULL, clear any previous application-set default
 * name and forget any cached value of the default name for @a context.
 *
 * Calls to this function invalidate the result of any previous calls to
 * krb5_cc_default_name() using @a context.
 *
 * @retval
 *  0  Success
 * @retval
 *  KV5M_CONTEXT          Bad magic number for @c _krb5_context structure
 * @return
 * Kerberos error codes
 */
/* *
 * Resolve the default credential cache name.
 *
 * @param [in]  context         Library context
 * @param [out] ccache          Pointer to credential cache name
 *
 * Create a handle to the default credential cache as given by
 * krb5_cc_default_name().
 *
 * @retval
 * 0  Success
 * @retval
 * KV5M_CONTEXT            Bad magic number for @c _krb5_context structure
 * @retval
 * KRB5_FCC_INTERNAL       The name of the default credential cache cannot be
 *                         obtained
 * @return
 * Kerberos error codes
 */
/* *
 * Copy a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  incc            Credential cache to be copied
 * @param [out] outcc           Copy of credential cache to be filled in
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
/* *
 * Get a configuration value from a credential cache.
 *
 * @param [in]     context      Library context
 * @param [in]     id           Credential cache handle
 * @param [in]     principal    Configuration for this principal;
 *                              if NULL, global for the whole cache
 * @param [in]     key          Name of config variable
 * @param [out]    data         Data to be fetched
 *
 * Use krb5_free_data_contents() to free @a data when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
/* *
 * Store a configuration value in a credential cache.
 *
 * @param [in]     context      Library context
 * @param [in]     id           Credential cache handle
 * @param [in]     principal    Configuration for a specific principal;
 *                              if NULL, global for the whole cache
 * @param [in]     key          Name of config variable
 * @param [in]     data         Data to store, or NULL to remove
 *
 * @note Existing configuration under the same key is over-written.
 *
 * @warning Before version 1.10 @a data was assumed to be always non-null.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
/* *
 * Test whether a principal is a configuration principal.
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal to check
 *
 * @return
 * @c TRUE if the principal is a configuration principal (generated part of
 * krb5_cc_set_config()); @c FALSE otherwise.
 */
/* *
 * Make a credential cache the primary cache for its collection.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * If the type of @a cache supports it, set @a cache to be the primary
 * credential cache for the collection it belongs to.
 *
 * @retval
 * 0  Success, or the type of @a cache doesn't support switching
 * @return
 * Kerberos error codes
 */
/* *
 * Determine whether a credential cache type supports switching.
 *
 * @param [in] context          Library context
 * @param [in] type             Credential cache type
 *
 * @version New in 1.10
 *
 * @retval TRUE if @a type supports switching
 * @retval FALSE if it does not or is not a valid credential cache type.
 */
/* *
 * Find a credential cache with a specified client principal.
 *
 * @param [in]  context         Library context
 * @param [in]  client          Client principal
 * @param [out] cache_out       Credential cache handle
 *
 * Find a cache within the collection whose default principal is @a client.
 * Use @a krb5_cc_close to close @a ccache when it is no longer needed.
 *
 * @retval 0 Success
 * @retval KRB5_CC_NOTFOUND
 *
 * @sa krb5_cccol_cursor_new
 *
 * @version New in 1.10
 */
/* *
 * Select a credential cache to use with a server principal.
 *
 * @param [in]  context         Library context
 * @param [in]  server          Server principal
 * @param [out] cache_out       Credential cache handle
 * @param [out] princ_out       Client principal
 *
 * Select a cache within the collection containing credentials most appropriate
 * for use with @a server, according to configured rules and heuristics.
 *
 * Use krb5_cc_close() to release @a cache_out when it is no longer needed.
 * Use krb5_free_principal() to release @a princ_out when it is no longer
 * needed.  Note that @a princ_out is set in some error conditions.
 *
 * @return
 * If an appropriate cache is found, 0 is returned, @a cache_out is set to the
 * selected cache, and @a princ_out is set to the default principal of that
 * cache.
 *
 * If the appropriate client principal can be authoritatively determined but
 * the cache collection contains no credentials for that principal, then
 * KRB5_CC_NOTFOUND is returned, @a cache_out is set to NULL, and @a princ_out
 * is set to the appropriate client principal.
 *
 * If no configured mechanism can determine the appropriate cache or principal,
 * KRB5_CC_NOTFOUND is returned and @a cache_out and @a princ_out are set to
 * NULL.
 *
 * Any other error code indicates a fatal error in the processing of a cache
 * selection mechanism.
 *
 * @version New in 1.10
 */
/* krb5_free.c */
/* *
 * Free the storage assigned to a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Principal to be freed
 */
/* *
 * Free a krb5_authenticator structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Authenticator structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
/* *
 * Free the data stored in array of addresses.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of addresses to be freed
 *
 * This function frees the contents of @a val and the array itself.
 *
 * @note The last entry in the array must be a NULL pointer.
 */
/* *
 * Free the storage assigned to array of authentication data.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of authentication data to be freed
 *
 * This function frees the contents of @a val and the array itself.
 *
 * @note The last entry in the array must be a NULL pointer.
 */
/* *
 * Free a ticket.
 *
 * @param [in] context          Library context
 * @param [in] val              Ticket to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
/* *
 * Free an error allocated by krb5_read_error() or krb5_sendauth().
 *
 * @param [in] context          Library context
 * @param [in] val              Error data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
/* *
 * Free a krb5_creds structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Credential structure to be freed.
 *
 * This function frees the contents of @a val and the structure itself.
 */
/* *
 * Free the contents of a krb5_creds structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Credential structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
/* *
 * Free a krb5_checksum structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Checksum structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
/* *
 * Free the contents of a krb5_checksum structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Checksum structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
/* *
 * Free a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Keyblock to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
/* *
 * Free the contents of a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] key              Keyblock to be freed
 *
 * This function frees the contents of @a key, but not the structure itself.
 */
/* *
 * Free a krb5_ap_rep_enc_part structure.
 *
 * @param [in] context          Library context
 * @param [in] val              AP-REP enc part to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
/* *
 * Free a krb5_data structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
/* Free a krb5_octet_data structure (should be unused). */
/* *
 * Free the contents of a krb5_data structure and zero the data field.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
/* *
 * Free a string representation of a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Name string to be freed
 */
/* *
 * Free a string allocated by a krb5 function.
 *
 * @param [in] context          Library context
 * @param [in] val              String to be freed
 *
 * @version New in 1.10
 */
/* *
 * Free an array of encryption types.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of enctypes to be freed
 *
 * @version New in 1.12
 */
/* *
 * Free an array of checksum types.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of checksum types to be freed
 */
/* From krb5/os, but needed by the outside world */
/* *
 * Retrieve the system time of day, in sec and ms, since the epoch.
 *
 * @param [in]  context         Library context
 * @param [out] seconds         System timeofday, seconds portion
 * @param [out] microseconds    System timeofday, microseconds portion
 *
 * This function retrieves the system time of day with the context
 * specific time offset adjustment.
 *
 * @sa krb5_crypto_us_timeofday()
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
/* *
 * Retrieve the current time with context specific time offset adjustment.
 *
 * @param [in]  context         Library context
 * @param [out] timeret         Timestamp to fill in
 *
 * This function retrieves the system time of day with the context specific
 * time offset adjustment.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
/* *
 * Check if a timestamp is within the allowed clock skew of the current time.
 *
 * @param [in]     context      Library context
 * @param [in]     date         Timestamp to check
 *
 * This function checks if @a date is close enough to the current time
 * according to the configured allowable clock skew.
 *
 * @version New in 1.10
 *
 * @retval 0 Success
 * @retval KRB5KRB_AP_ERR_SKEW @a date is not within allowable clock skew
 */
/* *
 * Return all interface addresses for this host.
 *
 * @param [in]  context         Library context
 * @param [out] addr            Array of krb5_address pointers, ending with
 *                              NULL
 *
 * Use krb5_free_addresses() to free @a addr when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Retrieve the default realm.
 *
 * @param [in]  context         Library context
 * @param [out] lrealm          Default realm name
 *
 * Retrieves the default realm to be used if no user-specified realm is
 * available.
 *
 * Use krb5_free_default_realm() to free @a lrealm when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
/* *
 * Override the default realm for the specified context.
 *
 * @param [in]     context      Library context
 * @param [in]     lrealm       Realm name for the default realm
 *
 * If @a lrealm is NULL, clear the default realm setting.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
/* *
 * Free a default realm string returned by krb5_get_default_realm().
 *
 * @param [in] context          Library context
 * @param [in] lrealm           Realm to be freed
 */
/* *
 * Canonicalize a hostname, possibly using name service.
 *
 * @param [in]  context         Library context
 * @param [in]  host            Input hostname
 * @param [out] canonhost_out   Canonicalized hostname
 *
 * This function canonicalizes orig_hostname, possibly using name service
 * lookups if configuration permits.  Use krb5_free_string() to free @a
 * canonhost_out when it is no longer needed.
 *
 * @version New in 1.15
 */
/* *
 * Generate a full principal name from a service name.
 *
 * @param [in]  context         Library context
 * @param [in]  hostname        Host name, or NULL to use local host
 * @param [in]  sname           Service name, or NULL to use @c "host"
 * @param [in]  type            Principal type
 * @param [out] ret_princ       Generated principal
 *
 * This function converts a @a hostname and @a sname into @a krb5_principal
 * structure @a ret_princ.  The returned principal will be of the form @a
 * sname\/hostname\@REALM where REALM is determined by krb5_get_host_realm().
 * In some cases this may be the referral (empty) realm.
 *
 * The @a type can be one of the following:
 *
 * @li #KRB5_NT_SRV_HST canonicalizes the host name before looking up the
 * realm and generating the principal.
 *
 * @li #KRB5_NT_UNKNOWN accepts the hostname as given, and does not
 * canonicalize it.
 *
 * Use krb5_free_principal to free @a ret_princ when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
/* *
 * Test whether a principal matches a matching principal.
 *
 * @param [in]  context         Library context
 * @param [in]  matching        Matching principal
 * @param [in]  princ           Principal to test
 *
 * @note A matching principal is a host-based principal with an empty realm
 * and/or second data component (hostname).  Profile configuration may cause
 * the hostname to be ignored even if it is present.  A principal matches a
 * matching principal if the former has the same non-empty (and non-ignored)
 * components of the latter.
 *
 * If @a matching is NULL, return TRUE.  If @a matching is not a matching
 * principal, return the value of krb5_principal_compare(context, matching,
 * princ).
 *
 * @return
 * TRUE if @a princ matches @a matching, FALSE otherwise.
 */
/* *
 * Change a password for an existing Kerberos account.
 *
 * @param [in]  context             Library context
 * @param [in]  creds               Credentials for kadmin/changepw service
 * @param [in]  newpw               New password
 * @param [out] result_code         Numeric error code from server
 * @param [out] result_code_string  String equivalent to @a result_code
 * @param [out] result_string       Change password response from the KDC
 *
 * Change the password for the existing principal identified by @a creds.
 *
 * The possible values of the output @a result_code are:
 *
 * @li #KRB5_KPASSWD_SUCCESS   (0) - success
 * @li #KRB5_KPASSWD_MALFORMED (1) - Malformed request error
 * @li #KRB5_KPASSWD_HARDERROR (2) - Server error
 * @li #KRB5_KPASSWD_AUTHERROR (3) - Authentication error
 * @li #KRB5_KPASSWD_SOFTERROR (4) - Password change rejected
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Set a password for a principal using specified credentials.
 *
 * @param [in]  context              Library context
 * @param [in]  creds                Credentials for kadmin/changepw service
 * @param [in]  newpw                New password
 * @param [in]  change_password_for  Change the password for this principal
 * @param [out] result_code          Numeric error code from server
 * @param [out] result_code_string   String equivalent to @a result_code
 * @param [out] result_string        Data returned from the remote system
 *
 * This function uses the credentials @a creds to set the password @a newpw for
 * the principal @a change_password_for.  It implements the set password
 * operation of RFC 3244, for interoperability with Microsoft Windows
 * implementations.
 *
 * @note If @a change_password_for is NULL, the change is performed on the
 * current principal. If @a change_password_for is non-null, the change is
 * performed on the principal name passed in @a change_password_for.
 *
 * The error code and strings are returned in @a result_code,
 * @a result_code_string and @a result_string.
 *
 * @sa krb5_set_password_using_ccache()
 *
 * @retval
 * 0  Success and result_code is set to #KRB5_KPASSWD_SUCCESS.
 * @return
 * Kerberos error codes.
 */
/* *
 * Set a password for a principal using cached credentials.
 *
 * @param [in]  context              Library context
 * @param [in]  ccache               Credential cache
 * @param [in]  newpw                New password
 * @param [in]  change_password_for  Change the password for this principal
 * @param [out] result_code          Numeric error code from server
 * @param [out] result_code_string   String equivalent to @a result_code
 * @param [out] result_string        Data returned from the remote system
 *
 * This function uses the cached credentials from @a ccache to set the password
 * @a newpw for the principal @a change_password_for.  It implements RFC 3244
 * set password operation (interoperable with MS Windows implementations) using
 * the credential cache.
 *
 * The error code and strings are returned in @a result_code,
 * @a result_code_string and @a result_string.
 *
 * @note If @a change_password_for is set to NULL, the change is performed on
 * the default principal in @a ccache. If @a change_password_for is non null,
 * the change is performed on the specified principal.
 *
 * @sa krb5_set_password()
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
/* *
 * Get a result message for changing or setting a password.
 *
 * @param [in]  context            Library context
 * @param [in]  server_string      Data returned from the remote system
 * @param [out] message_out        A message displayable to the user
 *
 * This function processes the @a server_string returned in the @a
 * result_string parameter of krb5_change_password(), krb5_set_password(), and
 * related functions, and returns a displayable string.  If @a server_string
 * contains Active Directory structured policy information, it will be
 * converted into human-readable text.
 *
 * Use krb5_free_string() to free @a message_out when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 *
 * @version New in 1.11
 */
/* *
 * Retrieve configuration profile from the context.
 *
 * @param [in]  context         Library context
 * @param [out] profile         Pointer to data read from a configuration file
 *
 * This function creates a new @a profile object that reflects profile
 * in the supplied @a context.
 *
 * The @a profile object may be freed with profile_release() function.
 * See profile.h and profile API for more details.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
#[no_mangle]
#[c2rust::src_loc = "453:1"]
pub unsafe extern "C" fn krb5_get_profile(mut ctx: krb5_context,
                                          mut profile: *mut profile_t)
 -> krb5_error_code {
    return profile_copy((*ctx).profile, profile) as krb5_error_code;
}
#[no_mangle]
#[c2rust::src_loc = "459:1"]
pub unsafe extern "C" fn krb5_set_config_files(mut ctx: krb5_context,
                                               mut filenames:
                                                   *mut *const libc::c_char)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut profile: profile_t = 0 as *mut _profile_t;
    retval =
        profile_init_flags(filenames, 0x1 as libc::c_int, &mut profile) as
            krb5_error_code;
    if retval != 0 { return retval }
    if !(*ctx).profile.is_null() { profile_release((*ctx).profile); }
    (*ctx).profile = profile;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "477:1"]
pub unsafe extern "C" fn krb5_get_default_config_files(mut pfilenames:
                                                           *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    if pfilenames.is_null() { return 22 as libc::c_int }
    return os_get_default_config_files(pfilenames,
                                       0 as libc::c_int as krb5_boolean);
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 1989,1990,1991,1992,1993,1994,1995,2000,2001,
 * 2003,2006,2007,2008,2009 by the Massachusetts Institute of Technology,
 * Cambridge, MA, USA.  All Rights Reserved.
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
/*
 * Copyright (C) 1998 by the FundsXpress, INC.
 *
 * All rights reserved.
 *
 * Export of this software from the United States of America may require
 * a specific license from the United States Government.  It is the
 * responsibility of any person or organization contemplating export to
 * obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of FundsXpress. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  FundsXpress makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */
/*
 * This prototype for k5-int.h (Krb5 internals include file)
 * includes the user-visible definitions from krb5.h and then
 * includes other definitions that are not user-visible but are
 * required for compiling Kerberos internal routines.
 *
 * John Gilmore, Cygnus Support, Sat Jan 21 22:45:52 PST 1995
 */
/* KRB5_GENERAL__ */
/*
 * Begin "k5-config.h"
 */
/*
 * Machine-type definitions: PC Clone 386 running Microloss Windows
 */
/* From autoconf.h */
/* HAVE_SYS_TYPES_H */
/* HAVE_SYS_TYPES_H */
/* KRB5_SYSTYPES__ */
/* one day */
/* one week */
/* Thu Jan  1 00:00:00 2038 UTC */
/*
 * Windows requires a different api interface to each function. Here
 * just define it as NULL.
 */
/* #define KRB5_OLD_CRYPTO is done in krb5.h */
/* KRB5_CONFIG__ */
/*
 * End "k5-config.h"
 */
/*
 * After loading the configuration definitions, load the Kerberos definitions.
 */
/* Get mutex support; currently used only for the replay cache.  */
/* Get error info support.  */
/* Get string buffer support. */
/* Define tracing macros. */
/* Profile variables.  Constants are named KRB5_CONF_STRING, where STRING
 * matches the variable name.  Keep these alphabetized. */
/* Cache configuration variables */
/* Error codes used in KRB_ERROR protocol messages.
   Return values of library routines are based on a different error table
   (which allows non-ambiguous error codes between subsystems) */
/* KDC errors */
/* No error */
/* Client's entry in DB expired */
/* Server's entry in DB expired */
/* Requested pvno not supported */
/* C's key encrypted in old master */
/* S's key encrypted in old master */
/* Client not found in Kerberos DB */
/* Server not found in Kerberos DB */
/* Multiple entries in Kerberos DB */
/* The C or S has a null key */
/* Tkt ineligible for postdating */
/* Requested starttime > endtime */
/* KDC policy rejects request */
/* KDC can't do requested opt. */
/* No support for encryption type */
/* No support for checksum type */
/* No support for padata type */
/* No support for transited type */
/* C's creds have been revoked */
/* S's creds have been revoked */
/* TGT has been revoked */
/* C not yet valid */
/* S not yet valid */
/* Password has expired */
/* Preauthentication failed */
/* Additional preauthentication */
                                           /* required */
/* Requested server and */
                                           /* ticket don't match*/
/* Server principal valid for */
                                           /*   user2user only */
/* KDC policy rejected transited */
                                           /*   path */
/* A service is not
                                            * available that is
                                            * required to process the
                                            * request */
/* Application errors */
/* Decrypt integrity check failed */
/* Ticket expired */
/* Ticket not yet valid */
/* Request is a replay */
/* The ticket isn't for us */
/* Ticket/authenticator don't match */
/* Clock skew too great */
/* Incorrect net address */
/* Protocol version mismatch */
/* Invalid message type */
/* Message stream modified */
/* Message out of order */
/* Key version is not available */
/* Service key not available */
/* Mutual authentication failed */
/* Incorrect message direction */
/* Alternative authentication */
                                        /* method required */
/* Incorrect sequence numnber */
                                        /* in message */
/* Inappropriate type of */
                                        /* checksum in message */
/* Policy rejects transited path */
/* Response too big for UDP, */
                                        /*   retry with TCP */
/* other errors */
/* Generic error (description */
                                        /* in e-text) */
/* Field is too long for impl. */
/* PKINIT server-reported errors */
/* client cert not trusted */
/* client signature verify failed */
/* invalid Diffie-Hellman parameters */
/* client cert not verifiable to */
                                                   /* trusted root cert */
/* client cert had invalid signature */
/* client cert was revoked */
/* client cert revoked, reason unknown */
/* mismatch between client cert and */
                                                   /* principal name */
/* bad extended key use */
/* bad digest algorithm in client cert */
/* missing paChecksum in PA-PK-AS-REQ */
/* bad digest algorithm in SignedData */
/* The IAKERB proxy could
                                                      not find a KDC */
/* The KDC did not respond
                                                      to the IAKERB proxy */
/* RFC 6113 */
/* RFC 6113 */
/* err table base max offset for protocol err codes */
/*
 * A null-terminated array of this structure is returned by the KDC as
 * the data part of the ETYPE_INFO preauth type.  It informs the
 * client which encryption types are supported.
 * The  same data structure is used by both etype-info and etype-info2
 * but s2kparams must be null when encoding etype-info.
 */
/*
 *  This is essentially -1 without sign extension which can screw up
 *  comparisons on 64 bit machines. If the length is this value, then
 *  the salt data is not present. This is to distinguish between not
 *  being set and being of 0 length.
 */
/* RFC 4537 */
/* sam_type values -- informational only */
/*  Enigma Logic */
/*  Digital Pathways */
/*  S/key where  KDC has key 0 */
/*  Traditional S/Key */
/*  Security Dynamics */
/*  CRYPTOCard */
/* XXX need to figure out who has which numbers assigned */
/*  ActivCard decimal mode */
/*  ActivCard hex mode */
/*  Digital Pathways hex mode */
/* experimental */
/* testing */
/* special */
/* Array of checksums */
/* information */
/* KRB5_SAM_* values */
/* informational */
/* KRB5_SAM_* values */
/* copied */
/* krb5_enc_sam_response_enc */
/*
 * Keep the pkinit definitions in a separate file so that the plugin
 * only has to include k5-int-pkinit.h rather than k5-int.h
 */
/* -1 for unspecified */
/* -1 for unspecified */
/* -1 for unspecified */
/* -1 for unspecified */
/* -1 for unspecified */
/* Plain text of an encrypted PA-FX-COOKIE value produced by the KDC. */
/* In PAC options, indicates Resource-Based Constrained Delegation support. */
/* struct stat, stat() */
/* MAXPATHLEN */
/* prototypes for file-related
                                           syscalls; flags for open &
                                           friends */
/* libos.spec */
/* Internal structure of an opaque key identifier */
/*
     * Cache of data private to the cipher implementation, which we
     * don't want to have to recompute for every operation.  This may
     * include key schedules, iteration counts, etc.
     *
     * The cipher implementation is responsible for setting this up
     * whenever needed, and the enc_provider key_cleanup method must
     * then be provided to dispose of it.
     */
/* Write the SHA-256 hash of in (containing n elements) to out. */
/* Convenience function: zap and free ptr if it is non-NULL. */
/* Convenience function: zap and free zero-terminated str if it is non-NULL. */
/* Convenience function: zap and free krb5_data pointer if it is non-NULL. */
/*
 * End "los-proto.h"
 */
/*
 * Flags for the os_flags field
 *
 * KRB5_OS_TOFFSET_VALID means that the time offset fields are valid.
 * The intention is that this facility to correct the system clocks so
 * that they reflect the "real" time, for systems where for some
 * reason we can't set the system clock.  Instead we calculate the
 * offset between the system time and real time, and store the offset
 * in the os context so that we can correct the system clock as necessary.
 *
 * KRB5_OS_TOFFSET_TIME means that the time offset fields should be
 * returned as the time by the krb5 time routines.  This should only
 * be used for testing purposes (obviously!)
 */
/* lock mode flags */
/*
 * Begin "preauth.h"
 *
 * (Originally written by Glen Machin at Sandia Labs.)
 */
/*
 * Sandia National Laboratories also makes no representations about the
 * suitability of the modifications, or additions to this software for
 * any purpose.  It is provided "as is" without express or implied warranty.
 */
/* check logon hour restrictions */
/* sign with usage 27 instead of 26 */
/* padata from req_body is used*/
/* Bits 0-15 are critical in FAST options (RFC 6113 section 7.3). */
/*
 * AD-CAMMAC's other-verifiers field is a sequence of Verifier, which is an
 * extensible choice with only one selection, Verifier-MAC.  For the time being
 * we will represent this field directly as an array of krb5_verifier_mac.
 * That will have to change if other selections are added.
 */
/* Does not return a copy; original padata sequence responsible for freeing*/
/* Allocate a pa-data object with uninitialized contents of size len.  If len
 * is 0, set the contents field to NULL. */
/* Free a single pa-data object. */
/* Without copying, add single element *pa to *list, reallocating as necessary.
 * If *list is NULL, allocate a new list.  Set *pa to NULL on success. */
/* Without copying, add a pa-data element of type pa_type to *list with the
 * contents in data.  Set *data to empty_data() on success. */
/* Add an empty pa-data element of type pa_type to *list. */
/* KRB5_PREAUTH__ */
/*
 * End "preauth.h"
 */
/* #include "krb5/wordsize.h" -- comes in through base-defs.h. */
/* ** Plugin framework ***/
/*
 * This framework can be used to create pluggable interfaces.  Not all existing
 * pluggable interface use this framework, but new ones should.  A new
 * pluggable interface entails:
 *
 * - An interface ID definition in the list of #defines below.
 *
 * - A name in the interface_names array in lib/krb5/krb/plugins.c.
 *
 * - An installed public header file in include/krb5.  The public header should
 *   include <krb5/plugin.h> and should declare a vtable structure for each
 *   supported major version of the interface.
 *
 * - A consumer API implementation, located within the code unit which makes
 *   use of the pluggable interface.  The consumer API should consist of:
 *
 *   . An interface-specific handle type which contains a vtable structure for
 *     the module (or a union of several such structures, if there are multiple
 *     supported major versions) and, optionally, resource data bound to the
 *     handle.
 *
 *   . An interface-specific loader function which creates a handle or list of
 *     handles.  A list of handles would be created if the interface is a
 *     one-to-many interface where the consumer wants to consult all available
 *     modules; a single handle would be created for an interface where the
 *     consumer wants to consult a specific module.  The loader function should
 *     use k5_plugin_load or k5_plugin_load_all to produce one or a list of
 *     vtable initializer functions, and should use those functions to fill in
 *     the vtable structure for the module (if necessary, trying each supported
 *     major version starting from the most recent).  The loader function can
 *     also bind resource data into the handle based on caller arguments, if
 *     appropriate.
 *
 *   . For each plugin method, a wrapper function which accepts a krb5_context,
 *     a plugin handle, and the method arguments.  Wrapper functions should
 *     invoke the method function contained in the handle's vtable.
 *
 * - Possibly, built-in implementations of the interface, also located within
 *   the code unit which makes use of the interface.  Built-in implementations
 *   must be registered with k5_plugin_register before the first call to
 *   k5_plugin_load or k5_plugin_load_all.
 *
 * A pluggable interface should have one or more currently supported major
 * versions, starting at 1.  Each major version should have a current minor
 * version, also starting at 1.  If new methods are added to a vtable, the
 * minor version should be incremented and the vtable stucture should document
 * where each minor vtable version ends.  If method signatures for a vtable are
 * changed, the major version should be incremented.
 *
 * Plugin module implementations (either built-in or dynamically loaded) should
 * define a function named <interfacename>_<modulename>_initvt, matching the
 * signature of krb5_plugin_initvt_fn as declared in include/krb5/plugin.h.
 * The initvt function should check the given maj_ver argument against its own
 * supported major versions, cast the vtable pointer to the appropriate
 * interface-specific vtable type, and fill in the vtable methods, stopping as
 * appropriate for the given min_ver.  Memory for the vtable structure is
 * allocated by the caller, not by the module.
 *
 * Dynamic plugin modules are registered with the framework through the
 * [plugins] section of the profile, as described in the admin documentation
 * and krb5.conf man page.
 */
/* Holds krb5_context information about each pluggable interface. */
/* A list of plugin interface IDs.  Make sure to increment
 * PLUGIN_NUM_INTERFACES when a new interface is added, and add an entry to the
 * interface_names table in lib/krb5/krb/plugin.c. */
/* Retrieve the plugin module of type interface_id and name modname,
 * storing the result into module. */
/* Retrieve all plugin modules of type interface_id, storing the result
 * into modules.  Free the result with k5_plugin_free_handles. */
/* Release a module list allocated by k5_plugin_load_all. */
/* Register a plugin module of type interface_id and name modname. */
/*
 * Register a plugin module which is part of the krb5 tree but is built as a
 * dynamic plugin.  Look for the module in modsubdir relative to the
 * context->base_plugin_dir.
 */
/* Destroy the module state within context; used by krb5_free_context. */
/* private, in kdb5.h */
/* allowable clock skew */
/* Message size above which we'll try TCP first in send-to-kdc
       type code.  Aside from the 2**16 size limit, we put no
       absolute limit on the UDP packet size.  */
/* Use the config-file ktypes instead of app-specified?  */
/* locate_kdc module stuff */
/* preauth module stuff */
/* cache module stuff */
/* localauth module stuff */
/* hostrealm module stuff */
/* TLS module vtable (if loaded) */
/* error detail info */
/* For Sun iprop code; does this really have to be here?  */
/* could be used in a table to find an etype and initialize a block */
/* internal message representations */
/* user data */
/* client time, optional */
/* microsecond portion of time,
                                           optional */
/* sequence #, optional */
/* sender address */
/* recipient address, optional */
/* data integrity checksum */
/* encrypted part */
/* user data */
/* client time, optional */
/* microsecond portion of time, opt. */
/* sequence #, optional */
/* sender address */
/* recipient address, optional */
/*
 * Begin "asn1.h"
 */
/* ASN.1 encoding knowledge; KEEP IN SYNC WITH ASN.1 defs! */
/* here we use some knowledge of ASN.1 encodings */
/*
  Ticket is APPLICATION 1.
  Authenticator is APPLICATION 2.
  AS_REQ is APPLICATION 10.
  AS_REP is APPLICATION 11.
  TGS_REQ is APPLICATION 12.
  TGS_REP is APPLICATION 13.
  AP_REQ is APPLICATION 14.
  AP_REP is APPLICATION 15.
  KRB_SAFE is APPLICATION 20.
  KRB_PRIV is APPLICATION 21.
  KRB_CRED is APPLICATION 22.
  EncASRepPart is APPLICATION 25.
  EncTGSRepPart is APPLICATION 26.
  EncAPRepPart is APPLICATION 27.
  EncKrbPrivPart is APPLICATION 28.
  EncKrbCredPart is APPLICATION 29.
  KRB_ERROR is APPLICATION 30.
*/
/* allow either constructed or primitive encoding, so check for bit 6
   set or reset */
/* ************************************************************************
 * Prototypes for krb5_encode.c
 *************************************************************************/
/*
  krb5_error_code encode_krb5_structure(const krb5_structure *rep,
  krb5_data **code);
  modifies  *code
  effects   Returns the ASN.1 encoding of *rep in **code.
  Returns ASN1_MISSING_FIELD if a required field is emtpy in *rep.
  Returns ENOMEM if memory runs out.
*/
/* yes, the translation is identical to that used for KDC__REP */
/* yes, the translation is identical to that used for KDC__REP */
/* ************************************************************************
 * End of prototypes for krb5_encode.c
 *************************************************************************/
/* ************************************************************************
 * Prototypes for krb5_decode.c
 *************************************************************************/
/*
  krb5_error_code decode_krb5_structure(const krb5_data *code,
  krb5_structure **rep);

  requires  Expects **rep to not have been allocated;
  a new *rep is allocated regardless of the old value.
  effects   Decodes *code into **rep.
  Returns ENOMEM if memory is exhausted.
  Returns asn1 and krb5 errors.
*/
/* kdb.h */
/* Master key version number */
/* kvno of key_data elements (all the same) */
/* ************************************************************************
 * End of prototypes for krb5_decode.c
 *************************************************************************/
/* KRB5_ASN1__ */
/*
 * End "asn1.h"
 */
/*
 * Internal krb5 library routines
 */
/* Return true if s is non-empty and composed solely of digits. */
/*
 * Initialization routines.
 */
/* [De]serialize 4-byte integer */
/* [De]serialize 8-byte integer */
/* [De]serialize byte string */
/* Fill in the buffer with random alpha-numeric data. */
/* value to use when requesting a keytab entry and KVNO doesn't matter */
/* value to use when requesting a keytab entry and enctype doesn't matter */
/* To keep happy libraries which are (for now) accessing internal stuff */
/* Make sure to increment by one when changing the struct */
/* Used for KDB LDAP back end.  */
/*
     * pkinit asn.1 encode/decode functions
     */
/* Set *tag_out to the integrity tag of *enc.  (Does not allocate memory;
 * returned buffer is a subrange of *ctext.) */
/*
 * This structure was exposed and used in macros in krb5 1.2, so do not
 * change its ABI.
 */
/* routines always present */
/* routines to be included on extended version (write routines) */
/* Not sure it's ready for exposure just yet.  */
/*
 * Referral definitions and subfunctions.
 */
/* should move into k5-int.h */
/* chk_trans.c */
/* free_rtree.c */
#[no_mangle]
#[c2rust::src_loc = "485:1"]
pub unsafe extern "C" fn krb5_free_config_files(mut filenames:
                                                    *mut *mut libc::c_char) {
    free_filespecs(filenames);
}
#[no_mangle]
#[c2rust::src_loc = "491:1"]
pub unsafe extern "C" fn k5_os_free_context(mut ctx: krb5_context) {
    let mut os_ctx: krb5_os_context = 0 as *mut _krb5_os_context;
    os_ctx = &mut (*ctx).os_context;
    if !(*os_ctx).default_ccname.is_null() {
        free((*os_ctx).default_ccname as *mut libc::c_void);
        (*os_ctx).default_ccname = 0 as *mut libc::c_char
    }
    (*os_ctx).magic = 0 as libc::c_int;
    if !(*ctx).profile.is_null() {
        profile_release((*ctx).profile);
        (*ctx).profile = 0 as profile_t
    }
    if !(*ctx).preauth_context.is_null() {
        k5_free_preauth_context(ctx);
        (*ctx).preauth_context = 0 as krb5_preauth_context
    }
    krb5int_close_plugin_dirs(&mut (*ctx).libkrb5_plugins);
    /* _WIN32 */
}
