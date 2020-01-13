use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:36"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:36"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:36"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "228:16"]
    pub struct krb5_principal_data {
        pub magic: krb5_magic,
        pub realm: krb5_data,
        pub data: *mut krb5_data,
        pub length: krb5_int32,
        pub type_0: krb5_int32,
    }
    #[c2rust::src_loc = "261:1"]
    pub type krb5_const_principal = *const krb5_principal_data;
    #[c2rust::src_loc = "8510:1"]
    pub type krb5_post_recv_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_error_code, _: *const krb5_data,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
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
    extern "C" {
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        #[no_mangle]
        #[c2rust::src_loc = "4912:1"]
        pub fn krb5_free_default_realm(context: krb5_context,
                                       lrealm: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "3650:1"]
        pub fn krb5_realm_compare(context: krb5_context,
                                  princ1: krb5_const_principal,
                                  princ2: krb5_const_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "4887:1"]
        pub fn krb5_get_default_realm(context: krb5_context,
                                      lrealm: *mut *mut libc::c_char)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:36"]
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
    /* Holds krb5_context information about each pluggable interface. */
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
    /* private, in kdb5.h */
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
    #[inline]
    #[c2rust::src_loc = "2268:1"]
    pub unsafe extern "C" fn string2data(mut str: *mut libc::c_char)
     -> krb5_data {
        return make_data(str as *mut libc::c_void,
                         strlen(str) as libc::c_uint);
    }
    #[inline]
    #[c2rust::src_loc = "2251:1"]
    pub unsafe extern "C" fn make_data(mut data: *mut libc::c_void,
                                       mut len: libc::c_uint) -> krb5_data {
        let mut d: krb5_data =
            krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
        d.magic = -(1760647422 as libc::c_long) as krb5_magic;
        d.data = data as *mut libc::c_char;
        d.length = len;
        return d;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_data};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::string_h::strlen;
    extern "C" {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:36"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:36"]
pub mod k5_plugin_h {
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
    /* opaque */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:8"]
    pub struct plugin_dir_handle {
        pub files: *mut *mut plugin_file_handle,
    }
    extern "C" {
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
    }
    /* K5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:36"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/stdlib.h:36"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:36"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::types_h::__int32_t;
pub use self::stdint_intn_h::int32_t;
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_enctype, krb5_flags,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_const_principal,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _profile_t, krb5_free_default_realm,
                       krb5_realm_compare, krb5_get_default_realm};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, string2data, make_data,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
use self::stdlib_h::{malloc, realloc};
use self::string_h::{strlen, memcpy};
#[c2rust::src_loc = "61:1"]
unsafe extern "C" fn component_length_quoted(mut src: *const krb5_data,
                                             mut flags: libc::c_int)
 -> libc::c_int {
    let mut cp: *const libc::c_char = (*src).data;
    let mut length: libc::c_int = (*src).length as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut size: libc::c_int = length;
    if flags & 0x4 as libc::c_int == 0 as libc::c_int {
        let mut no_realm: libc::c_int =
            (flags & 0x2 as libc::c_int != 0 &&
                 flags & 0x1 as libc::c_int == 0) as libc::c_int;
        j = 0 as libc::c_int;
        while j < length {
            if no_realm == 0 && *cp as libc::c_int == '@' as i32 ||
                   *cp as libc::c_int == '/' as i32 ||
                   *cp as libc::c_int == '\u{0}' as i32 ||
                   *cp as libc::c_int == '\\' as i32 ||
                   *cp as libc::c_int == '\t' as i32 ||
                   *cp as libc::c_int == '\n' as i32 ||
                   *cp as libc::c_int == '\u{8}' as i32 {
                size += 1
            }
            j += 1;
            cp = cp.offset(1)
        }
    }
    return size;
}
#[c2rust::src_loc = "84:1"]
unsafe extern "C" fn copy_component_quoting(mut dest: *mut libc::c_char,
                                            mut src: *const krb5_data,
                                            mut flags: libc::c_int)
 -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut cp: *const libc::c_char = (*src).data;
    let mut q: *mut libc::c_char = dest;
    let mut length: libc::c_int = (*src).length as libc::c_int;
    if flags & 0x4 as libc::c_int != 0 {
        if (*src).length > 0 as libc::c_int as libc::c_uint {
            memcpy(dest as *mut libc::c_void,
                   (*src).data as *const libc::c_void,
                   (*src).length as libc::c_ulong);
        }
        return (*src).length as libc::c_int
    }
    j = 0 as libc::c_int;
    while j < length {
        let mut no_realm: libc::c_int =
            (flags & 0x2 as libc::c_int != 0 &&
                 flags & 0x1 as libc::c_int == 0) as libc::c_int;
        let mut current_block_17: u64;
        match *cp as libc::c_int {
            64 => {
                if no_realm != 0 {
                    let fresh0 = q;
                    q = q.offset(1);
                    *fresh0 = *cp;
                    current_block_17 = 15125582407903384992;
                } else { current_block_17 = 12388617853274591701; }
            }
            47 | 92 => { current_block_17 = 12388617853274591701; }
            9 => {
                let fresh3 = q;
                q = q.offset(1);
                *fresh3 = '\\' as i32 as libc::c_char;
                let fresh4 = q;
                q = q.offset(1);
                *fresh4 = 't' as i32 as libc::c_char;
                current_block_17 = 15125582407903384992;
            }
            10 => {
                let fresh5 = q;
                q = q.offset(1);
                *fresh5 = '\\' as i32 as libc::c_char;
                let fresh6 = q;
                q = q.offset(1);
                *fresh6 = 'n' as i32 as libc::c_char;
                current_block_17 = 15125582407903384992;
            }
            8 => {
                let fresh7 = q;
                q = q.offset(1);
                *fresh7 = '\\' as i32 as libc::c_char;
                let fresh8 = q;
                q = q.offset(1);
                *fresh8 = 'b' as i32 as libc::c_char;
                current_block_17 = 15125582407903384992;
            }
            0 => {
                let fresh9 = q;
                q = q.offset(1);
                *fresh9 = '\\' as i32 as libc::c_char;
                let fresh10 = q;
                q = q.offset(1);
                *fresh10 = '0' as i32 as libc::c_char;
                current_block_17 = 15125582407903384992;
            }
            _ => {
                let fresh11 = q;
                q = q.offset(1);
                *fresh11 = *cp;
                current_block_17 = 15125582407903384992;
            }
        }
        match current_block_17 {
            12388617853274591701 => {
                let fresh1 = q;
                q = q.offset(1);
                *fresh1 = '\\' as i32 as libc::c_char;
                let fresh2 = q;
                q = q.offset(1);
                *fresh2 = *cp
            }
            _ => { }
        }
        j += 1;
        cp = cp.offset(1)
    }
    return q.wrapping_offset_from(dest) as libc::c_long as libc::c_int;
}
#[c2rust::src_loc = "136:1"]
unsafe extern "C" fn k5_unparse_name(mut context: krb5_context,
                                     mut principal: krb5_const_principal,
                                     mut flags: libc::c_int,
                                     mut name: *mut *mut libc::c_char,
                                     mut size: *mut libc::c_uint)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: krb5_int32 = 0;
    let mut totalsize: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut default_realm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    if principal.is_null() || name.is_null() {
        return -(1765328250 as libc::c_long) as krb5_error_code
    }
    if flags & 0x1 as libc::c_int != 0 {
        /* omit realm if local realm */
        let mut p: krb5_principal_data =
            krb5_principal_data{magic: 0,
                                realm:
                                    krb5_data{magic: 0,
                                              length: 0,
                                              data: 0 as *mut libc::c_char,},
                                data: 0 as *mut krb5_data,
                                length: 0,
                                type_0: 0,};
        ret = krb5_get_default_realm(context, &mut default_realm);
        if ret != 0 as libc::c_int {
            current_block = 1220941658951966431;
        } else {
            p.realm = string2data(default_realm);
            if krb5_realm_compare(context,
                                  &mut p as *mut krb5_principal_data as
                                      krb5_const_principal, principal) != 0 {
                flags |= 0x2 as libc::c_int
            }
            current_block = 17965632435239708295;
        }
    } else { current_block = 17965632435239708295; }
    match current_block {
        17965632435239708295 => {
            if flags & 0x2 as libc::c_int == 0 as libc::c_int {
                totalsize =
                    totalsize.wrapping_add(component_length_quoted(&(*principal).realm,
                                                                   flags) as
                                               libc::c_uint);
                totalsize = totalsize.wrapping_add(1)
                /* This is for the separator */
            }
            i = 0 as libc::c_int;
            while i < (*principal).length {
                totalsize =
                    totalsize.wrapping_add(component_length_quoted(&mut *(*principal).data.offset(i
                                                                                                      as
                                                                                                      isize),
                                                                   flags) as
                                               libc::c_uint);
                totalsize = totalsize.wrapping_add(1);
                i += 1
                /* This is for the separator */
            }
            if (*principal).length == 0 as libc::c_int {
                totalsize = totalsize.wrapping_add(1)
            }
            /*
     * Allocate space for the ascii string; if space has been
     * provided, use it, realloc'ing it if necessary.
     *
     * We need only n-1 separators for n components, but we need
     * an extra byte for the NUL at the end.
     */
            if !size.is_null() {
                if !(*name).is_null() && *size < totalsize {
                    *name =
                        realloc(*name as *mut libc::c_void,
                                totalsize as libc::c_ulong) as
                            *mut libc::c_char
                } else {
                    *name =
                        malloc(totalsize as libc::c_ulong) as
                            *mut libc::c_char
                } /* Back up last component separator */
                *size = totalsize
            } else {
                *name =
                    malloc(totalsize as libc::c_ulong) as *mut libc::c_char
            }
            if (*name).is_null() {
                ret = 12 as libc::c_int
            } else {
                q = *name;
                i = 0 as libc::c_int;
                while i < (*principal).length {
                    q =
                        q.offset(copy_component_quoting(q,
                                                        &mut *(*principal).data.offset(i
                                                                                           as
                                                                                           isize),
                                                        flags) as isize);
                    let fresh12 = q;
                    q = q.offset(1);
                    *fresh12 = '/' as i32 as libc::c_char;
                    i += 1
                }
                if i > 0 as libc::c_int { q = q.offset(-1) }
                if flags & 0x2 as libc::c_int == 0 as libc::c_int {
                    let fresh13 = q;
                    q = q.offset(1);
                    *fresh13 = '@' as i32 as libc::c_char;
                    q =
                        q.offset(copy_component_quoting(q,
                                                        &(*principal).realm,
                                                        flags) as isize)
                }
                let fresh14 = q;
                q = q.offset(1);
                *fresh14 = '\u{0}' as i32 as libc::c_char
            }
        }
        _ => { }
    }
    if !default_realm.is_null() {
        krb5_free_default_realm(context, default_realm);
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "220:1"]
pub unsafe extern "C" fn krb5_unparse_name(mut context: krb5_context,
                                           mut principal:
                                               krb5_const_principal,
                                           mut name: *mut *mut libc::c_char)
 -> krb5_error_code {
    if !name.is_null() {
        /* name == NULL will return error from _ext */
        *name = 0 as *mut libc::c_char
    }
    return k5_unparse_name(context, principal, 0 as libc::c_int, name,
                           0 as *mut libc::c_uint);
}
#[no_mangle]
#[c2rust::src_loc = "230:1"]
pub unsafe extern "C" fn krb5_unparse_name_ext(mut context: krb5_context,
                                               mut principal:
                                                   krb5_const_principal,
                                               mut name:
                                                   *mut *mut libc::c_char,
                                               mut size: *mut libc::c_uint)
 -> krb5_error_code {
    return k5_unparse_name(context, principal, 0 as libc::c_int, name, size);
}
#[no_mangle]
#[c2rust::src_loc = "237:1"]
pub unsafe extern "C" fn krb5_unparse_name_flags(mut context: krb5_context,
                                                 mut principal:
                                                     krb5_const_principal,
                                                 mut flags: libc::c_int,
                                                 mut name:
                                                     *mut *mut libc::c_char)
 -> krb5_error_code {
    if !name.is_null() { *name = 0 as *mut libc::c_char }
    return k5_unparse_name(context, principal, flags, name,
                           0 as *mut libc::c_uint);
}
#[no_mangle]
#[c2rust::src_loc = "246:1"]
pub unsafe extern "C" fn krb5_unparse_name_flags_ext(mut context:
                                                         krb5_context,
                                                     mut principal:
                                                         krb5_const_principal,
                                                     mut flags: libc::c_int,
                                                     mut name:
                                                         *mut *mut libc::c_char,
                                                     mut size:
                                                         *mut libc::c_uint)
 -> krb5_error_code {
    return k5_unparse_name(context, principal, flags, name, size);
}
