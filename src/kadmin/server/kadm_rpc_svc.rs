use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:7"]
pub mod types_h {
    #[c2rust::src_loc = "32:1"]
    pub type __u_short = libc::c_ushort;
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/sys/types.h:7"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_short, __u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:7"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:7"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:7"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:7"]
pub mod krb5_h {
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "137:1"]
    pub type krb5_int16 = int16_t;
    #[c2rust::src_loc = "138:1"]
    pub type krb5_ui_2 = uint16_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
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
    #[c2rust::src_loc = "236:1"]
    pub type krb5_principal = *mut krb5_principal_data;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:7"]
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
    #[inline]
    #[c2rust::src_loc = "2244:1"]
    pub unsafe extern "C" fn data_eq_string(mut d: krb5_data,
                                            mut s: *const libc::c_char)
     -> libc::c_int {
        return (d.length as libc::c_ulong == strlen(s) &&
                    (d.length == 0 as libc::c_int as libc::c_uint ||
                         memcmp(d.data as *const libc::c_void,
                                s as *const libc::c_void,
                                d.length as libc::c_ulong) == 0)) as
                   libc::c_int;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_data};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::string_h::{strlen, memcmp};
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:7"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:7"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:7"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:7"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:7"]
pub mod in_h {
    #[c2rust::src_loc = "119:1"]
    pub type in_port_t = uint16_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "238:8"]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [libc::c_uchar; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:11"]
pub mod kdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:16"]
    pub struct _krb5_key_data {
        pub key_data_ver: krb5_int16,
        pub key_data_kvno: krb5_ui_2,
        pub key_data_type: [krb5_int16; 2],
        pub key_data_length: [krb5_ui_2; 2],
        pub key_data_contents: [*mut krb5_octet; 2],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "156:16"]
    pub struct krb5_string_attr_st {
        pub key: *mut libc::c_char,
        pub value: *mut libc::c_char,
    }
    #[c2rust::src_loc = "156:1"]
    pub type krb5_string_attr = krb5_string_attr_st;
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_data,
                        krb5_enctype, krb5_int32};
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:8"]
pub mod gssrpc_types_h {
    #[c2rust::src_loc = "88:1"]
    pub type rpcprog_t = uint32_t;
    #[c2rust::src_loc = "89:1"]
    pub type rpcvers_t = uint32_t;
    /* @(#)types.h	2.3 88/08/15 4.0 RPCSRC */
/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 *     * Neither the name of the “Oracle America, Inc.” nor the names of
 *       its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*      @(#)types.h 1.18 87/07/24 SMI      */
    /*
 * Rpc additions to <sys/types.h>
 */
    /*
 * Try to get MAXHOSTNAMELEN from somewhere.
 */
    /* #include <netdb.h> */
    /* Get htonl(), ntohl(), etc. */
    /* Define if we need to fake up some BSD type aliases. */
    /* Allow application to override. */
    /* #undef GSSRPC__BSD_TYPEALIASES */
    #[c2rust::src_loc = "91:1"]
    pub type rpcproc_t = uint32_t;
    #[c2rust::src_loc = "93:1"]
    pub type rpc_inline_t = int32_t;
    use super::stdint_uintn_h::uint32_t;
    use super::stdint_intn_h::int32_t;
    /* !defined(GSSRPC_TYPES_H) */
    /*
 * The below should probably be internal-only, but seem to be
 * traditionally exported in RPC implementations.
 */
    /* XXX namespace */
    /* This is for rpc/netdb.h */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:8"]
pub mod xdr_h {
    /* @(#)xdr.h	2.2 88/07/29 4.0 RPCSRC */
/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 *     * Neither the name of the "Oracle America, Inc." nor the names of
 *       its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*      @(#)xdr.h 1.19 87/04/22 SMI      */
    /*
 * xdr.h, External Data Representation Serialization Routines.
 */
    /* for FILE */
    /*
 * XDR provides a conventional way for converting between C data
 * types and an external bit-string representation.  Library supplied
 * routines provide for the conversion on built-in C data types.  These
 * routines and utility routines defined here are used to help implement
 * a type encode/decode routine for each user-defined type.
 *
 * Each data type provides a single procedure which takes two arguments:
 *
 *	bool_t
 *	xdrproc(xdrs, argresp)
 *		XDR *xdrs;
 *		<type> *argresp;
 *
 * xdrs is an instance of a XDR handle, to which or from which the data
 * type is to be converted.  argresp is a pointer to the structure to be
 * converted.  The XDR handle contains an operation field which indicates
 * which of the operations (ENCODE, DECODE * or FREE) is to be performed.
 *
 * XDR_DECODE may allocate space if the pointer argresp is null.  This
 * data can be freed with the XDR_FREE operation.
 *
 * We write only one procedure per data type to make it easy
 * to keep the encode and decode procedures for a data type consistent.
 * In many cases the same code performs all operations on a user defined type,
 * because all the hard work is done in the component type routines.
 * decode as a series of calls on the nested data types.
 */
    /*
 * Xdr operations.  XDR_ENCODE causes the type to be encoded into the
 * stream.  XDR_DECODE causes the type to be extracted from the stream.
 * XDR_FREE can be used to release the space allocated by an XDR_DECODE
 * request.
 */
    #[c2rust::src_loc = "81:1"]
    pub type xdr_op = libc::c_uint;
    #[c2rust::src_loc = "84:2"]
    pub const XDR_FREE: xdr_op = 2;
    #[c2rust::src_loc = "83:2"]
    pub const XDR_DECODE: xdr_op = 1;
    #[c2rust::src_loc = "82:2"]
    pub const XDR_ENCODE: xdr_op = 0;
    /*
 * This is the number of bytes per unit of external data.
 */
    /*
 * A xdrproc_t exists for each data type which is to be encoded or decoded.
 *
 * The second argument to the xdrproc_t is a pointer to an opaque pointer.
 * The opaque pointer generally points to a structure of the data type
 * to be decoded.  If this pointer is 0, then the type routines should
 * allocate dynamic storage of the appropriate size and return it.
 * bool_t	(*xdrproc_t)(XDR *, caddr_t *);
 *
 * XXX can't actually prototype it, because some take three args!!!
 */
    #[c2rust::src_loc = "105:1"]
    pub type xdrproc_t = Option<unsafe extern "C" fn() -> libc::c_int>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:16"]
    pub struct XDR {
        pub x_op: xdr_op,
        pub x_ops: *mut xdr_ops,
        pub x_public: caddr_t,
        pub x_private: *mut libc::c_void,
        pub x_base: caddr_t,
        pub x_handy: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "115:9"]
    pub struct xdr_ops {
        pub x_getlong: Option<unsafe extern "C" fn(_: *mut XDR,
                                                   _: *mut libc::c_long)
                                  -> libc::c_int>,
        pub x_putlong: Option<unsafe extern "C" fn(_: *mut XDR,
                                                   _: *mut libc::c_long)
                                  -> libc::c_int>,
        pub x_getbytes: Option<unsafe extern "C" fn(_: *mut XDR, _: caddr_t,
                                                    _: u_int) -> libc::c_int>,
        pub x_putbytes: Option<unsafe extern "C" fn(_: *mut XDR, _: caddr_t,
                                                    _: u_int) -> libc::c_int>,
        pub x_getpostn: Option<unsafe extern "C" fn(_: *mut XDR) -> u_int>,
        pub x_setpostn: Option<unsafe extern "C" fn(_: *mut XDR, _: u_int)
                                   -> libc::c_int>,
        pub x_inline: Option<unsafe extern "C" fn(_: *mut XDR, _: libc::c_int)
                                 -> *mut rpc_inline_t>,
        pub x_destroy: Option<unsafe extern "C" fn(_: *mut XDR) -> ()>,
    }
    use super::sys_types_h::{caddr_t, u_int};
    use super::gssrpc_types_h::rpc_inline_t;
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        /*
 * In-line routines for fast encode/decode of primitve data types.
 * Caveat emptor: these use single memory cycles to get the
 * data from the underlying buffer, and will fail to operate
 * properly if the data is not aligned.  The standard way to use these
 * is to say:
 *	if ((buf = XDR_INLINE(xdrs, count)) == NULL)
 *		return (FALSE);
 *	<<< macro calls >>>
 * where ``count'' is the number of bytes of data occupied
 * by the primitive data types.
 *
 * N.B. and frozen for all time: each data type here uses 4 bytes
 * of external representation.
 */
        /*
 * These are the "generic" xdr routines.
 */
        #[no_mangle]
        #[c2rust::src_loc = "251:1"]
        pub fn gssrpc_xdr_void(_: *mut XDR, _: *mut libc::c_void)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "298:1"]
        pub fn gssrpc_xdr_u_int32(_: *mut XDR, _: *mut uint32_t)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:8"]
pub mod auth_h {
    #[c2rust::src_loc = "55:1"]
    pub type auth_stat = libc::c_uint;
    #[c2rust::src_loc = "74:2"]
    pub const RPCSEC_GSS_CTXPROBLEM: auth_stat = 14;
    #[c2rust::src_loc = "73:2"]
    pub const RPCSEC_GSS_CREDPROBLEM: auth_stat = 13;
    #[c2rust::src_loc = "69:2"]
    pub const AUTH_FAILED: auth_stat = 7;
    #[c2rust::src_loc = "68:2"]
    pub const AUTH_INVALIDRESP: auth_stat = 6;
    #[c2rust::src_loc = "64:2"]
    pub const AUTH_TOOWEAK: auth_stat = 5;
    #[c2rust::src_loc = "63:2"]
    pub const AUTH_REJECTEDVERF: auth_stat = 4;
    #[c2rust::src_loc = "62:2"]
    pub const AUTH_BADVERF: auth_stat = 3;
    #[c2rust::src_loc = "61:2"]
    pub const AUTH_REJECTEDCRED: auth_stat = 2;
    #[c2rust::src_loc = "60:2"]
    pub const AUTH_BADCRED: auth_stat = 1;
    #[c2rust::src_loc = "56:2"]
    pub const AUTH_OK: auth_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:8"]
    pub struct opaque_auth {
        pub oa_flavor: libc::c_int,
        pub oa_base: caddr_t,
        pub oa_length: u_int,
    }
    use super::sys_types_h::{caddr_t, u_int};
    /* !defined(GSSRPC_AUTH_H) */
    /* RPCSEC_GSS */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:8"]
pub mod rpc_msg_h {
    /* @(#)rpc_msg.h	2.1 88/07/29 4.0 RPCSRC */
/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 *     * Neither the name of the "Oracle America, Inc." nor the names of
 *       its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*      @(#)rpc_msg.h 1.7 86/07/16 SMI      */
    /*
 * rpc_msg.h
 * rpc message definition
 */
    /*
 * Bottom up definition of an rpc message.
 * NOTE: call and reply use the same overall stuct but
 * different parts of unions within it.
 */
    /*
 * Reply part of an rpc exchange
 */
    /*
 * Reply to an rpc request that was accepted by the server.
 * Note: there could be an error even though the request was
 * accepted.
 */
    /* and many other null cases */
    /*
 * Reply to an rpc request that was rejected by the server.
 */
    /* why authentication did not work */
    /*
 * Body of a reply to an rpc request.
 */
    /*
 * Body of an rpc request call.
 */
    /* must be equal to two */
    /* protocol specific - provided by client */
    /*
 * The rpc message
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "150:8"]
    pub struct rpc_msg {
        pub rm_xid: uint32_t,
        pub rm_direction: msg_type,
        pub ru: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed {
        pub RM_cmb: call_body,
        pub RM_rmb: reply_body,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "125:8"]
    pub struct reply_body {
        pub rp_stat: reply_stat,
        pub ru: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:2"]
    pub union C2RustUnnamed_0 {
        pub RP_ar: accepted_reply,
        pub RP_dr: rejected_reply,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:8"]
    pub struct rejected_reply {
        pub rj_stat: reject_stat,
        pub ru: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:2"]
    pub union C2RustUnnamed_1 {
        pub RJ_versions: C2RustUnnamed_2,
        pub RJ_why: auth_stat,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:3"]
    pub struct C2RustUnnamed_2 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    #[c2rust::src_loc = "74:1"]
    pub type reject_stat = libc::c_uint;
    #[c2rust::src_loc = "76:2"]
    pub const AUTH_ERROR: reject_stat = 1;
    #[c2rust::src_loc = "75:2"]
    pub const RPC_MISMATCH: reject_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "88:8"]
    pub struct accepted_reply {
        pub ar_verf: opaque_auth,
        pub ar_stat: accept_stat,
        pub ru: C2RustUnnamed_3,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:2"]
    pub union C2RustUnnamed_3 {
        pub AR_versions: C2RustUnnamed_5,
        pub AR_results: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "96:3"]
    pub struct C2RustUnnamed_4 {
        pub where_0: caddr_t,
        pub proc_0: xdrproc_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:3"]
    pub struct C2RustUnnamed_5 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    #[c2rust::src_loc = "65:1"]
    pub type accept_stat = libc::c_uint;
    #[c2rust::src_loc = "71:2"]
    pub const SYSTEM_ERR: accept_stat = 5;
    #[c2rust::src_loc = "70:2"]
    pub const GARBAGE_ARGS: accept_stat = 4;
    #[c2rust::src_loc = "69:2"]
    pub const PROC_UNAVAIL: accept_stat = 3;
    #[c2rust::src_loc = "68:2"]
    pub const PROG_MISMATCH: accept_stat = 2;
    #[c2rust::src_loc = "67:2"]
    pub const PROG_UNAVAIL: accept_stat = 1;
    #[c2rust::src_loc = "66:2"]
    pub const SUCCESS: accept_stat = 0;
    #[c2rust::src_loc = "60:1"]
    pub type reply_stat = libc::c_uint;
    #[c2rust::src_loc = "62:2"]
    pub const MSG_DENIED: reply_stat = 1;
    #[c2rust::src_loc = "61:2"]
    pub const MSG_ACCEPTED: reply_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:8"]
    pub struct call_body {
        pub cb_rpcvers: rpcvers_t,
        pub cb_prog: rpcprog_t,
        pub cb_vers: rpcvers_t,
        pub cb_proc: rpcproc_t,
        pub cb_cred: opaque_auth,
        pub cb_verf: opaque_auth,
    }
    #[c2rust::src_loc = "55:1"]
    pub type msg_type = libc::c_uint;
    #[c2rust::src_loc = "57:2"]
    pub const REPLY: msg_type = 1;
    #[c2rust::src_loc = "56:2"]
    pub const CALL: msg_type = 0;
    use super::stdint_uintn_h::uint32_t;
    use super::auth_h::{auth_stat, opaque_auth};
    use super::gssrpc_types_h::{rpcvers_t, rpcprog_t, rpcproc_t};
    use super::sys_types_h::caddr_t;
    use super::xdr_h::xdrproc_t;
    /* !defined(GSSRPC_RPC_MSG_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:8"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    /*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    /* OM_STRING */
    /*
 * We can't use X/Open definitions, so roll our own.
 */
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
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
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        /* name_equal */
        #[no_mangle]
        #[c2rust::src_loc = "554:1"]
        pub fn gss_display_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_buffer_t, _: *mut gss_OID)
         -> OM_uint32;
        /* output_name */
        #[no_mangle]
        #[c2rust::src_loc = "569:1"]
        pub fn gss_release_name(_: *mut OM_uint32, _: *mut gss_name_t)
         -> OM_uint32;
        /* input_name */
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        /* mechanisms */
        /* Last argument new for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "594:1"]
        pub fn gss_inquire_context(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: *mut gss_name_t, _: *mut gss_name_t,
                                   _: *mut OM_uint32, _: *mut gss_OID,
                                   _: *mut OM_uint32, _: *mut libc::c_int,
                                   _: *mut libc::c_int) -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc.h:8"]
pub mod svc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:8"]
    pub struct svc_req {
        pub rq_prog: rpcprog_t,
        pub rq_vers: rpcvers_t,
        pub rq_proc: rpcproc_t,
        pub rq_cred: opaque_auth,
        pub rq_clntcred: *mut libc::c_void,
        pub rq_svccred: *mut libc::c_void,
        pub rq_clntname: *mut libc::c_void,
        pub rq_xprt: *mut SVCXPRT,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "76:16"]
    pub struct SVCXPRT {
        pub xp_sock: libc::c_int,
        pub xp_port: u_short,
        pub xp_ops: *mut xp_ops,
        pub xp_addrlen: libc::c_int,
        pub xp_raddr: sockaddr_in,
        pub xp_verf: opaque_auth,
        pub xp_auth: *mut SVCAUTH,
        pub xp_p1: *mut libc::c_void,
        pub xp_p2: *mut libc::c_void,
        pub xp_laddrlen: libc::c_int,
        pub xp_laddr: sockaddr_in,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "83:9"]
    pub struct xp_ops {
        pub xp_recv: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                 _: *mut rpc_msg)
                                -> libc::c_int>,
        pub xp_stat: Option<unsafe extern "C" fn(_: *mut SVCXPRT)
                                -> xprt_stat>,
        pub xp_getargs: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                    _: xdrproc_t,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int>,
        pub xp_reply: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                  _: *mut rpc_msg)
                                 -> libc::c_int>,
        pub xp_freeargs: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                     _: xdrproc_t,
                                                     _: *mut libc::c_void)
                                    -> libc::c_int>,
        pub xp_destroy: Option<unsafe extern "C" fn(_: *mut SVCXPRT) -> ()>,
    }
    #[c2rust::src_loc = "67:1"]
    pub type xprt_stat = libc::c_uint;
    #[c2rust::src_loc = "70:2"]
    pub const XPRT_IDLE: xprt_stat = 2;
    #[c2rust::src_loc = "69:2"]
    pub const XPRT_MOREREQS: xprt_stat = 1;
    #[c2rust::src_loc = "68:2"]
    pub const XPRT_DIED: xprt_stat = 0;
    use super::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t};
    use super::auth_h::opaque_auth;
    use super::sys_types_h::{u_short, caddr_t};
    use super::in_h::sockaddr_in;
    use super::svc_auth_h::SVCAUTH;
    use super::rpc_msg_h::rpc_msg;
    use super::xdr_h::xdrproc_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "258:1"]
        pub fn gssrpc_svc_sendreply(_: *mut SVCXPRT, _: xdrproc_t, _: caddr_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "259:1"]
        pub fn gssrpc_svcerr_decode(_: *mut SVCXPRT);
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn gssrpc_svcerr_weakauth(_: *mut SVCXPRT);
        #[no_mangle]
        #[c2rust::src_loc = "261:1"]
        pub fn gssrpc_svcerr_noproc(_: *mut SVCXPRT);
        #[no_mangle]
        #[c2rust::src_loc = "265:1"]
        pub fn gssrpc_svcerr_systemerr(_: *mut SVCXPRT);
    }
    /* !defined(GSSRPC_SVC_H) */
    /* XXX add auth_gsapi_log_*? */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc_auth.h:8"]
pub mod svc_auth_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:16"]
    pub struct SVCAUTH {
        pub svc_ah_ops: *mut svc_auth_ops,
        pub svc_ah_private: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "54:9"]
    pub struct svc_auth_ops {
        pub svc_ah_wrap: Option<unsafe extern "C" fn(_: *mut SVCAUTH,
                                                     _: *mut XDR,
                                                     _: xdrproc_t, _: caddr_t)
                                    -> libc::c_int>,
        pub svc_ah_unwrap: Option<unsafe extern "C" fn(_: *mut SVCAUTH,
                                                       _: *mut XDR,
                                                       _: xdrproc_t,
                                                       _: caddr_t)
                                      -> libc::c_int>,
        pub svc_ah_destroy: Option<unsafe extern "C" fn(_: *mut SVCAUTH)
                                       -> libc::c_int>,
    }
    use super::xdr_h::{XDR, xdrproc_t};
    use super::sys_types_h::caddr_t;
    /* !defined(GSSRPC_SVC_AUTH_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:11"]
pub mod admin_h {
    #[c2rust::src_loc = "71:1"]
    pub type kadm5_ret_t = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "190:16"]
    pub struct _kadm5_principal_ent_t {
        pub principal: krb5_principal,
        pub princ_expire_time: krb5_timestamp,
        pub last_pwd_change: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub max_life: krb5_deltat,
        pub mod_name: krb5_principal,
        pub mod_date: krb5_timestamp,
        pub attributes: krb5_flags,
        pub kvno: krb5_kvno,
        pub mkvno: krb5_kvno,
        pub policy: *mut libc::c_char,
        pub aux_attributes: libc::c_long,
        pub max_renewable_life: krb5_deltat,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_key_data: krb5_int16,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    #[c2rust::src_loc = "190:1"]
    pub type kadm5_principal_ent_rec = _kadm5_principal_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "215:16"]
    pub struct _kadm5_policy_ent_t {
        pub policy: *mut libc::c_char,
        pub pw_min_life: libc::c_long,
        pub pw_max_life: libc::c_long,
        pub pw_min_length: libc::c_long,
        pub pw_min_classes: libc::c_long,
        pub pw_history_num: libc::c_long,
        pub policy_refcnt: libc::c_long,
        pub pw_max_fail: krb5_kvno,
        pub pw_failcnt_interval: krb5_deltat,
        pub pw_lockout_duration: krb5_deltat,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub allowed_keysalts: *mut libc::c_char,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
    }
    #[c2rust::src_loc = "215:1"]
    pub type kadm5_policy_ent_rec = _kadm5_policy_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:16"]
    pub struct _kadm5_config_params {
        pub mask: libc::c_long,
        pub realm: *mut libc::c_char,
        pub kadmind_port: libc::c_int,
        pub kpasswd_port: libc::c_int,
        pub admin_server: *mut libc::c_char,
        pub dbname: *mut libc::c_char,
        pub acl_file: *mut libc::c_char,
        pub dict_file: *mut libc::c_char,
        pub mkey_from_kbd: libc::c_int,
        pub stash_file: *mut libc::c_char,
        pub mkey_name: *mut libc::c_char,
        pub enctype: krb5_enctype,
        pub max_life: krb5_deltat,
        pub max_rlife: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub flags: krb5_flags,
        pub keysalts: *mut krb5_key_salt_tuple,
        pub num_keysalts: krb5_int32,
        pub kvno: krb5_kvno,
        pub iprop_enabled: libc::c_int,
        pub iprop_ulogsize: uint32_t,
        pub iprop_poll_time: krb5_deltat,
        pub iprop_logfile: *mut libc::c_char,
        pub iprop_port: libc::c_int,
        pub iprop_resync_timeout: libc::c_int,
        pub kadmind_listen: *mut libc::c_char,
        pub kpasswd_listen: *mut libc::c_char,
        pub iprop_listen: *mut libc::c_char,
    }
    #[c2rust::src_loc = "241:1"]
    pub type kadm5_config_params = _kadm5_config_params;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "284:16"]
    pub struct _kadm5_key_data {
        pub kvno: krb5_kvno,
        pub key: krb5_keyblock,
        pub salt: krb5_keysalt,
    }
    #[c2rust::src_loc = "284:1"]
    pub type kadm5_key_data = _kadm5_key_data;
    use super::krb5_h::{krb5_principal, krb5_timestamp, krb5_deltat,
                        krb5_flags, krb5_kvno, krb5_int16, krb5_enctype,
                        krb5_int32, krb5_keyblock};
    use super::kdb_h::{krb5_tl_data, krb5_key_data, krb5_key_salt_tuple,
                       krb5_keysalt};
    use super::stdint_uintn_h::uint32_t;
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/kadm_rpc.h:11"]
pub mod kadm_rpc_h {
    /* -*- mode: c; c-file-style: "bsd"; indent-tabs-mode: t -*- */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:8"]
    pub struct cprinc_arg {
        pub api_version: krb5_ui_4,
        pub rec: kadm5_principal_ent_rec,
        pub mask: libc::c_long,
        pub passwd: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:8"]
    pub struct cprinc3_arg {
        pub api_version: krb5_ui_4,
        pub rec: kadm5_principal_ent_rec,
        pub mask: libc::c_long,
        pub n_ks_tuple: libc::c_int,
        pub ks_tuple: *mut krb5_key_salt_tuple,
        pub passwd: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:8"]
    pub struct generic_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:8"]
    pub struct dprinc_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:8"]
    pub struct mprinc_arg {
        pub api_version: krb5_ui_4,
        pub rec: kadm5_principal_ent_rec,
        pub mask: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:8"]
    pub struct rprinc_arg {
        pub api_version: krb5_ui_4,
        pub src: krb5_principal,
        pub dest: krb5_principal,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "54:8"]
    pub struct gprincs_arg {
        pub api_version: krb5_ui_4,
        pub exp: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:8"]
    pub struct gprincs_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub princs: *mut *mut libc::c_char,
        pub count: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "68:8"]
    pub struct chpass_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub pass: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "75:8"]
    pub struct chpass3_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepold: krb5_boolean,
        pub n_ks_tuple: libc::c_int,
        pub ks_tuple: *mut krb5_key_salt_tuple,
        pub pass: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:8"]
    pub struct setkey_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keyblocks: *mut krb5_keyblock,
        pub n_keys: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "93:8"]
    pub struct setkey3_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepold: krb5_boolean,
        pub n_ks_tuple: libc::c_int,
        pub ks_tuple: *mut krb5_key_salt_tuple,
        pub keyblocks: *mut krb5_keyblock,
        pub n_keys: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "104:8"]
    pub struct setkey4_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepold: krb5_boolean,
        pub key_data: *mut kadm5_key_data,
        pub n_key_data: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:8"]
    pub struct chrand_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "119:8"]
    pub struct chrand3_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepold: krb5_boolean,
        pub n_ks_tuple: libc::c_int,
        pub ks_tuple: *mut krb5_key_salt_tuple,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "128:8"]
    pub struct chrand_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub key: krb5_keyblock,
        pub keys: *mut krb5_keyblock,
        pub n_keys: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "137:8"]
    pub struct gprinc_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub mask: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "144:8"]
    pub struct gprinc_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub rec: kadm5_principal_ent_rec,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "151:8"]
    pub struct cpol_arg {
        pub api_version: krb5_ui_4,
        pub rec: kadm5_policy_ent_rec,
        pub mask: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "158:8"]
    pub struct dpol_arg {
        pub api_version: krb5_ui_4,
        pub name: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "164:8"]
    pub struct mpol_arg {
        pub api_version: krb5_ui_4,
        pub rec: kadm5_policy_ent_rec,
        pub mask: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "171:8"]
    pub struct gpol_arg {
        pub api_version: krb5_ui_4,
        pub name: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:8"]
    pub struct gpol_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub rec: kadm5_policy_ent_rec,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "184:8"]
    pub struct gpols_arg {
        pub api_version: krb5_ui_4,
        pub exp: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "190:8"]
    pub struct gpols_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub pols: *mut *mut libc::c_char,
        pub count: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "198:8"]
    pub struct getprivs_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub privs: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "205:8"]
    pub struct purgekeys_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepkvno: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "212:8"]
    pub struct gstrings_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "218:8"]
    pub struct gstrings_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub strings: *mut krb5_string_attr,
        pub count: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "226:8"]
    pub struct sstring_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub key: *mut libc::c_char,
        pub value: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "234:8"]
    pub struct getpkeys_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub kvno: krb5_kvno,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:8"]
    pub struct getpkeys_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub key_data: *mut kadm5_key_data,
        pub n_key_data: libc::c_int,
    }
    use super::krb5_h::{krb5_ui_4, krb5_principal, krb5_boolean,
                        krb5_keyblock, krb5_kvno};
    use super::admin_h::{kadm5_principal_ent_rec, kadm5_ret_t, kadm5_key_data,
                         kadm5_policy_ent_rec};
    use super::kdb_h::{krb5_key_salt_tuple, krb5_string_attr};
    use super::svc_h::svc_req;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "254:1"]
        pub fn create_principal_2_svc(_: *mut cprinc_arg, _: *mut generic_ret,
                                      _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "259:1"]
        pub fn delete_principal_2_svc(_: *mut dprinc_arg, _: *mut generic_ret,
                                      _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "264:1"]
        pub fn modify_principal_2_svc(_: *mut mprinc_arg, _: *mut generic_ret,
                                      _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "269:1"]
        pub fn rename_principal_2_svc(_: *mut rprinc_arg, _: *mut generic_ret,
                                      _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "273:1"]
        pub fn get_principal_2_svc(_: *mut gprinc_arg, _: *mut gprinc_ret,
                                   _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "278:1"]
        pub fn chpass_principal_2_svc(_: *mut chpass_arg, _: *mut generic_ret,
                                      _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "283:1"]
        pub fn chrand_principal_2_svc(_: *mut chrand_arg, _: *mut chrand_ret,
                                      _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "287:1"]
        pub fn create_policy_2_svc(_: *mut cpol_arg, _: *mut generic_ret,
                                   _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "291:1"]
        pub fn delete_policy_2_svc(_: *mut dpol_arg, _: *mut generic_ret,
                                   _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "295:1"]
        pub fn modify_policy_2_svc(_: *mut mpol_arg, _: *mut generic_ret,
                                   _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "299:1"]
        pub fn get_policy_2_svc(_: *mut gpol_arg, _: *mut gpol_ret,
                                _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "302:1"]
        pub fn get_privs_2_svc(_: *mut krb5_ui_4, _: *mut getprivs_ret,
                               _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "305:1"]
        pub fn init_2_svc(_: *mut krb5_ui_4, _: *mut generic_ret,
                          _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "308:1"]
        pub fn get_princs_2_svc(_: *mut gprincs_arg, _: *mut gprincs_ret,
                                _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "312:1"]
        pub fn get_pols_2_svc(_: *mut gpols_arg, _: *mut gpols_ret,
                              _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "316:1"]
        pub fn setkey_principal_2_svc(_: *mut setkey_arg, _: *mut generic_ret,
                                      _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "324:1"]
        pub fn create_principal3_2_svc(_: *mut cprinc3_arg,
                                       _: *mut generic_ret, _: *mut svc_req)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "329:1"]
        pub fn chpass_principal3_2_svc(_: *mut chpass3_arg,
                                       _: *mut generic_ret, _: *mut svc_req)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "334:1"]
        pub fn chrand_principal3_2_svc(_: *mut chrand3_arg,
                                       _: *mut chrand_ret, _: *mut svc_req)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "339:1"]
        pub fn setkey_principal3_2_svc(_: *mut setkey3_arg,
                                       _: *mut generic_ret, _: *mut svc_req)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "343:1"]
        pub fn purgekeys_2_svc(_: *mut purgekeys_arg, _: *mut generic_ret,
                               _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "347:1"]
        pub fn get_strings_2_svc(_: *mut gstrings_arg, _: *mut gstrings_ret,
                                 _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "351:1"]
        pub fn set_string_2_svc(_: *mut sstring_arg, _: *mut generic_ret,
                                _: *mut svc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "356:1"]
        pub fn setkey_principal4_2_svc(_: *mut setkey4_arg,
                                       _: *mut generic_ret, _: *mut svc_req)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "361:1"]
        pub fn get_principal_keys_2_svc(_: *mut getpkeys_arg,
                                        _: *mut getpkeys_ret, _: *mut svc_req)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "364:1"]
        pub fn xdr_cprinc_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "365:1"]
        pub fn xdr_cprinc3_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn xdr_generic_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "367:1"]
        pub fn xdr_dprinc_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "368:1"]
        pub fn xdr_mprinc_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "369:1"]
        pub fn xdr_rprinc_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "370:1"]
        pub fn xdr_gprincs_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "371:1"]
        pub fn xdr_gprincs_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn xdr_chpass_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "373:1"]
        pub fn xdr_chpass3_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "374:1"]
        pub fn xdr_setkey_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "375:1"]
        pub fn xdr_setkey3_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "376:1"]
        pub fn xdr_setkey4_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "377:1"]
        pub fn xdr_chrand_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "378:1"]
        pub fn xdr_chrand3_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "379:1"]
        pub fn xdr_chrand_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn xdr_gprinc_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "381:1"]
        pub fn xdr_gprinc_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "391:1"]
        pub fn xdr_cpol_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "392:1"]
        pub fn xdr_dpol_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "393:1"]
        pub fn xdr_mpol_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "394:1"]
        pub fn xdr_gpol_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "395:1"]
        pub fn xdr_gpol_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "396:1"]
        pub fn xdr_gpols_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "397:1"]
        pub fn xdr_gpols_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "398:1"]
        pub fn xdr_getprivs_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "399:1"]
        pub fn xdr_purgekeys_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "400:1"]
        pub fn xdr_gstrings_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "401:1"]
        pub fn xdr_gstrings_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "402:1"]
        pub fn xdr_sstring_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "405:1"]
        pub fn xdr_getpkeys_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "406:1"]
        pub fn xdr_getpkeys_ret() -> libc::c_int;
    }
    /* __KADM_RPC_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/server_internal.h:16"]
pub mod server_internal_h {
    #[c2rust::src_loc = "42:1"]
    pub type kadm5_server_handle_t = *mut _kadm5_server_handle_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:16"]
    pub struct _kadm5_server_handle_t {
        pub magic_number: krb5_ui_4,
        pub struct_version: krb5_ui_4,
        pub api_version: krb5_ui_4,
        pub context: krb5_context,
        pub current_caller: krb5_principal,
        pub params: kadm5_config_params,
        pub lhandle: *mut _kadm5_server_handle_t,
        pub db_args: *mut *mut libc::c_char,
        pub qual_handles: *mut pwqual_handle,
        pub hook_handles: *mut kadm5_hook_handle,
    }
    #[c2rust::src_loc = "40:1"]
    pub type kadm5_hook_handle = *mut kadm5_hook_handle_st;
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 */
    /*
 * This header file is used internally by the Admin API server
 * libraries and Admin server.  IF YOU THINK YOU NEED TO USE THIS FILE
 * FOR ANYTHING, YOU'RE ALMOST CERTAINLY WRONG.
 */
    /*
 * This is the history key version for a newly created DB.  We use this value
 * for principals which have no password history yet to avoid having to look up
 * the history key.  Values other than 2 will cause compatibility issues with
 * pre-1.8 libkadm5 code; the older code will reject key changes when it sees
 * an unexpected value of admin_history_kvno.
 */
    /* A pwqual_handle represents a password quality plugin module. */
    #[c2rust::src_loc = "38:1"]
    pub type pwqual_handle = *mut pwqual_handle_st;
    use super::krb5_h::{krb5_ui_4, krb5_context, krb5_principal};
    use super::admin_h::kadm5_config_params;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type kadm5_hook_handle_st;
        #[c2rust::src_loc = "38:16"]
        pub type pwqual_handle_st;
    }
    /* __KADM5_SERVER_INTERNAL_H__ */
    /* * @}*/
}
#[c2rust::header_src = "/usr/include/string.h:7"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "132:14"]
        pub fn strncat(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:7"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:7"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_krb5.h:9"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "89:33"]
        pub static gss_nt_krb5_name: gss_OID;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:14"]
pub mod adm_proto_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn krb5_klog_syslog(_: libc::c_int, _: *const libc::c_char,
                                _: ...) -> libc::c_int;
    }
    /* KRB5_ADM_PROTO_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/server/misc.h:15"]
pub mod misc_h {
    use super::gssapi_h::OM_uint32;
    use super::svc_h::SVCXPRT;
    use super::stddef_h::size_t;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1994 OpenVision Technologies, Inc., All Rights Reserved
 *
 */
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn log_badauth(major: OM_uint32, minor: OM_uint32,
                           xprt: *mut SVCXPRT, data: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "28:1"]
        pub fn trunc_name(len: *mut size_t, dots: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn client_addr(xprt: *mut SVCXPRT) -> *const libc::c_char;
    }
    /* _MISC_H */
}
pub use self::types_h::{__u_short, __u_int, __uint8_t, __int16_t, __uint16_t,
                        __int32_t, __uint32_t, __caddr_t};
pub use self::sys_types_h::{u_short, u_int, caddr_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal, krb5_post_recv_fn,
                       krb5_context, krb5_pre_send_fn, krb5_trace_callback,
                       krb5_trace_info, _krb5_trace_info, krb5_prompt_type,
                       _krb5_keyblock, krb5_keyblock, _profile_t,
                       krb5_parse_name, krb5_free_principal};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, data_eq_string, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::sockaddr_h::sa_family_t;
pub use self::in_h::{in_port_t, sockaddr_in, in_addr, in_addr_t};
pub use self::kdb_h::{_krb5_key_data, _krb5_tl_data, krb5_tl_data,
                      krb5_string_attr_st, krb5_string_attr, krb5_key_data,
                      _krb5_keysalt, krb5_keysalt, __krb5_key_salt_tuple,
                      krb5_key_salt_tuple};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_void, gssrpc_xdr_u_int32};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, opaque_auth};
pub use self::rpc_msg_h::{rpc_msg, C2RustUnnamed, reply_body, C2RustUnnamed_0,
                          rejected_reply, C2RustUnnamed_1, C2RustUnnamed_2,
                          reject_stat, AUTH_ERROR, RPC_MISMATCH,
                          accepted_reply, C2RustUnnamed_3, C2RustUnnamed_4,
                          C2RustUnnamed_5, accept_stat, SYSTEM_ERR,
                          GARBAGE_ARGS, PROC_UNAVAIL, PROG_MISMATCH,
                          PROG_UNAVAIL, SUCCESS, reply_stat, MSG_DENIED,
                          MSG_ACCEPTED, call_body, msg_type, REPLY, CALL};
pub use self::gssapi_h::{gss_name_t, gss_ctx_id_t, gss_uint32, OM_uint32,
                         gss_OID_desc_struct, gss_OID, gss_buffer_desc_struct,
                         gss_buffer_desc, gss_buffer_t, gss_name_struct,
                         gss_ctx_id_struct, gss_display_name,
                         gss_release_name, gss_release_buffer,
                         gss_inquire_context};
pub use self::svc_h::{svc_req, SVCXPRT, xp_ops, xprt_stat, XPRT_IDLE,
                      XPRT_MOREREQS, XPRT_DIED, gssrpc_svc_sendreply,
                      gssrpc_svcerr_decode, gssrpc_svcerr_weakauth,
                      gssrpc_svcerr_noproc, gssrpc_svcerr_systemerr};
pub use self::svc_auth_h::{SVCAUTH, svc_auth_ops};
pub use self::admin_h::{kadm5_ret_t, _kadm5_principal_ent_t,
                        kadm5_principal_ent_rec, _kadm5_policy_ent_t,
                        kadm5_policy_ent_rec, _kadm5_config_params,
                        kadm5_config_params, _kadm5_key_data, kadm5_key_data};
pub use self::kadm_rpc_h::{cprinc_arg, cprinc3_arg, generic_ret, dprinc_arg,
                           mprinc_arg, rprinc_arg, gprincs_arg, gprincs_ret,
                           chpass_arg, chpass3_arg, setkey_arg, setkey3_arg,
                           setkey4_arg, chrand_arg, chrand3_arg, chrand_ret,
                           gprinc_arg, gprinc_ret, cpol_arg, dpol_arg,
                           mpol_arg, gpol_arg, gpol_ret, gpols_arg, gpols_ret,
                           getprivs_ret, purgekeys_arg, gstrings_arg,
                           gstrings_ret, sstring_arg, getpkeys_arg,
                           getpkeys_ret, create_principal_2_svc,
                           delete_principal_2_svc, modify_principal_2_svc,
                           rename_principal_2_svc, get_principal_2_svc,
                           chpass_principal_2_svc, chrand_principal_2_svc,
                           create_policy_2_svc, delete_policy_2_svc,
                           modify_policy_2_svc, get_policy_2_svc,
                           get_privs_2_svc, init_2_svc, get_princs_2_svc,
                           get_pols_2_svc, setkey_principal_2_svc,
                           create_principal3_2_svc, chpass_principal3_2_svc,
                           chrand_principal3_2_svc, setkey_principal3_2_svc,
                           purgekeys_2_svc, get_strings_2_svc,
                           set_string_2_svc, setkey_principal4_2_svc,
                           get_principal_keys_2_svc, xdr_cprinc_arg,
                           xdr_cprinc3_arg, xdr_generic_ret, xdr_dprinc_arg,
                           xdr_mprinc_arg, xdr_rprinc_arg, xdr_gprincs_arg,
                           xdr_gprincs_ret, xdr_chpass_arg, xdr_chpass3_arg,
                           xdr_setkey_arg, xdr_setkey3_arg, xdr_setkey4_arg,
                           xdr_chrand_arg, xdr_chrand3_arg, xdr_chrand_ret,
                           xdr_gprinc_arg, xdr_gprinc_ret, xdr_cpol_arg,
                           xdr_dpol_arg, xdr_mpol_arg, xdr_gpol_arg,
                           xdr_gpol_ret, xdr_gpols_arg, xdr_gpols_ret,
                           xdr_getprivs_ret, xdr_purgekeys_arg,
                           xdr_gstrings_arg, xdr_gstrings_ret,
                           xdr_sstring_arg, xdr_getpkeys_arg,
                           xdr_getpkeys_ret};
pub use self::server_internal_h::{kadm5_server_handle_t,
                                  _kadm5_server_handle_t, kadm5_hook_handle,
                                  pwqual_handle, kadm5_hook_handle_st,
                                  pwqual_handle_st};
use self::string_h::{memset, memcmp, strncat, strlen};
use self::stdlib_h::{malloc, free};
use self::libintl_h::dgettext;
use self::gssapi_krb5_h::gss_nt_krb5_name;
use self::adm_proto_h::krb5_klog_syslog;
use self::misc_h::{log_badauth, trunc_name, client_addr};
extern "C" {
    /* -*- mode: c; c-file-style: "bsd"; indent-tabs-mode: t -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved.
 *
 */
    #[no_mangle]
    #[c2rust::src_loc = "18:14"]
    pub static mut global_server_handle: *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "63:6"]
pub union C2RustUnnamed_6 {
    pub gen_ret: generic_ret,
    pub get_principal_2_ret: gprinc_ret,
    pub chrand_principal_2_ret: chrand_ret,
    pub get_policy_2_ret: gpol_ret,
    pub get_privs_2_ret: getprivs_ret,
    pub get_princs_2_ret: gprincs_ret,
    pub get_pols_2_ret: gpols_ret,
    pub chrand_principal3_2_ret: chrand_ret,
    pub get_string_2_ret: gstrings_ret,
    pub get_principal_keys_ret: getpkeys_ret,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "43:6"]
pub union C2RustUnnamed_7 {
    pub create_principal_2_arg: cprinc_arg,
    pub delete_principal_2_arg: dprinc_arg,
    pub modify_principal_2_arg: mprinc_arg,
    pub rename_principal_2_arg: rprinc_arg,
    pub get_principal_2_arg: gprinc_arg,
    pub chpass_principal_2_arg: chpass_arg,
    pub chrand_principal_2_arg: chrand_arg,
    pub create_policy_2_arg: cpol_arg,
    pub delete_policy_2_arg: dpol_arg,
    pub modify_policy_2_arg: mpol_arg,
    pub get_policy_2_arg: gpol_arg,
    pub setkey_principal_2_arg: setkey_arg,
    pub create_principal3_2_arg: cprinc3_arg,
    pub chpass_principal3_2_arg: chpass3_arg,
    pub chrand_principal3_2_arg: chrand3_arg,
    pub setkey_principal3_2_arg: setkey3_arg,
    pub setkey_principal4_2_arg: setkey4_arg,
    pub get_principal_keys_2_arg: getpkeys_arg,
}
/*
 * Function: kadm_1
 *
 * Purpose: RPC proccessing procedure.
 *	    originally generated from rpcgen
 *
 * Arguments:
 *	rqstp		    (input) rpc request structure
 *	transp		    (input) rpc transport structure
 *	(input/output)
 *	<return value>
 *
 * Requires:
 * Effects:
 * Modifies:
 */
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn kadm_1(mut rqstp: *mut svc_req,
                                mut transp: *mut SVCXPRT) {
    let mut argument: C2RustUnnamed_7 =
        C2RustUnnamed_7{create_principal_2_arg:
                            cprinc_arg{api_version: 0,
                                       rec:
                                           kadm5_principal_ent_rec{principal:
                                                                       0 as
                                                                           *mut krb5_principal_data,
                                                                   princ_expire_time:
                                                                       0,
                                                                   last_pwd_change:
                                                                       0,
                                                                   pw_expiration:
                                                                       0,
                                                                   max_life:
                                                                       0,
                                                                   mod_name:
                                                                       0 as
                                                                           *mut krb5_principal_data,
                                                                   mod_date:
                                                                       0,
                                                                   attributes:
                                                                       0,
                                                                   kvno: 0,
                                                                   mkvno: 0,
                                                                   policy:
                                                                       0 as
                                                                           *mut libc::c_char,
                                                                   aux_attributes:
                                                                       0,
                                                                   max_renewable_life:
                                                                       0,
                                                                   last_success:
                                                                       0,
                                                                   last_failed:
                                                                       0,
                                                                   fail_auth_count:
                                                                       0,
                                                                   n_key_data:
                                                                       0,
                                                                   n_tl_data:
                                                                       0,
                                                                   tl_data:
                                                                       0 as
                                                                           *mut krb5_tl_data,
                                                                   key_data:
                                                                       0 as
                                                                           *mut krb5_key_data,},
                                       mask: 0,
                                       passwd: 0 as *mut libc::c_char,},};
    let mut result: C2RustUnnamed_6 =
        C2RustUnnamed_6{gen_ret: generic_ret{api_version: 0, code: 0,},};
    let mut retval: libc::c_int = 0;
    let mut xdr_argument: Option<unsafe extern "C" fn() -> libc::c_int> =
        None;
    let mut xdr_result: Option<unsafe extern "C" fn() -> libc::c_int> = None;
    let mut local: Option<unsafe extern "C" fn() -> libc::c_int> = None;
    if (*rqstp).rq_cred.oa_flavor != 300001 as libc::c_int &&
           check_rpcsec_auth(rqstp) == 0 {
        krb5_klog_syslog(3 as libc::c_int,
                         b"Authentication attempt failed: %s, RPC authentication flavor %d\x00"
                             as *const u8 as *const libc::c_char,
                         client_addr((*rqstp).rq_xprt),
                         (*rqstp).rq_cred.oa_flavor);
        gssrpc_svcerr_weakauth(transp);
        return
    }
    match (*rqstp).rq_proc {
        0 => {
            gssrpc_svc_sendreply(transp,
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                         *mut XDR,
                                                                                     _:
                                                                                         *mut libc::c_void)
                                                                    ->
                                                                        libc::c_int>,
                                                         xdrproc_t>(Some(gssrpc_xdr_void
                                                                             as
                                                                             unsafe extern "C" fn(_:
                                                                                                      *mut XDR,
                                                                                                  _:
                                                                                                      *mut libc::c_void)
                                                                                 ->
                                                                                     libc::c_int)),
                                 0 as *mut libc::c_void as *mut libc::c_char);
            return
        }
        1 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_cprinc_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut cprinc_arg,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(create_principal_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut cprinc_arg,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        2 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_dprinc_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut dprinc_arg,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(delete_principal_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut dprinc_arg,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        3 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_mprinc_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut mprinc_arg,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(modify_principal_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut mprinc_arg,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        4 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_rprinc_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut rprinc_arg,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(rename_principal_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut rprinc_arg,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        5 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_gprinc_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_gprinc_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut gprinc_arg,
                                                                    _:
                                                                        *mut gprinc_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(get_principal_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut gprinc_arg,
                                                                                                   _:
                                                                                                       *mut gprinc_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        14 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_gprincs_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_gprincs_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut gprincs_arg,
                                                                    _:
                                                                        *mut gprincs_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(get_princs_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut gprincs_arg,
                                                                                                   _:
                                                                                                       *mut gprincs_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        6 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_chpass_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut chpass_arg,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(chpass_principal_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut chpass_arg,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        16 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_setkey_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut setkey_arg,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(setkey_principal_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut setkey_arg,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        7 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_chrand_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_chrand_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut chrand_arg,
                                                                    _:
                                                                        *mut chrand_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(chrand_principal_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut chrand_arg,
                                                                                                   _:
                                                                                                       *mut chrand_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        8 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_cpol_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut cpol_arg,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(create_policy_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut cpol_arg,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        9 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_dpol_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut dpol_arg,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(delete_policy_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut dpol_arg,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        10 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_mpol_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut mpol_arg,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(modify_policy_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut mpol_arg,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        11 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_gpol_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_gpol_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut gpol_arg,
                                                                    _:
                                                                        *mut gpol_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(get_policy_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut gpol_arg,
                                                                                                   _:
                                                                                                       *mut gpol_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        15 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_gpols_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_gpols_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut gpols_arg,
                                                                    _:
                                                                        *mut gpols_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(get_pols_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut gpols_arg,
                                                                                                   _:
                                                                                                       *mut gpols_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        12 => {
            xdr_argument =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut XDR,
                                                                    _:
                                                                        *mut uint32_t)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(gssrpc_xdr_u_int32
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut XDR,
                                                                                                   _:
                                                                                                       *mut uint32_t)
                                                                                  ->
                                                                                      libc::c_int));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_getprivs_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut krb5_ui_4,
                                                                    _:
                                                                        *mut getprivs_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(get_privs_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut krb5_ui_4,
                                                                                                   _:
                                                                                                       *mut getprivs_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        13 => {
            xdr_argument =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut XDR,
                                                                    _:
                                                                        *mut uint32_t)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(gssrpc_xdr_u_int32
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut XDR,
                                                                                                   _:
                                                                                                       *mut uint32_t)
                                                                                  ->
                                                                                      libc::c_int));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut krb5_ui_4,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(init_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut krb5_ui_4,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        18 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_cprinc3_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut cprinc3_arg,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(create_principal3_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut cprinc3_arg,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        19 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_chpass3_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut chpass3_arg,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(chpass_principal3_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut chpass3_arg,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        20 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_chrand3_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_chrand_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut chrand3_arg,
                                                                    _:
                                                                        *mut chrand_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(chrand_principal3_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut chrand3_arg,
                                                                                                   _:
                                                                                                       *mut chrand_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        21 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_setkey3_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut setkey3_arg,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(setkey_principal3_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut setkey3_arg,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        22 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_purgekeys_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut purgekeys_arg,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(purgekeys_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut purgekeys_arg,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        23 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_gstrings_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_gstrings_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut gstrings_arg,
                                                                    _:
                                                                        *mut gstrings_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(get_strings_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut gstrings_arg,
                                                                                                   _:
                                                                                                       *mut gstrings_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        24 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_sstring_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut sstring_arg,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(set_string_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut sstring_arg,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        25 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_setkey4_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_generic_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut setkey4_arg,
                                                                    _:
                                                                        *mut generic_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(setkey_principal4_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut setkey4_arg,
                                                                                                   _:
                                                                                                       *mut generic_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        26 => {
            xdr_argument =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_getpkeys_arg));
            xdr_result =
                Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                 -> libc::c_int,
                                             unsafe extern "C" fn()
                                                 ->
                                                     libc::c_int>(xdr_getpkeys_ret));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut getpkeys_arg,
                                                                    _:
                                                                        *mut getpkeys_ret,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(get_principal_keys_2_svc
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut getpkeys_arg,
                                                                                                   _:
                                                                                                       *mut getpkeys_ret,
                                                                                                   _:
                                                                                                       *mut svc_req)
                                                                                  ->
                                                                                      libc::c_int))
        }
        _ => {
            krb5_klog_syslog(3 as libc::c_int,
                             b"Invalid KADM5 procedure number: %s, %d\x00" as
                                 *const u8 as *const libc::c_char,
                             client_addr((*rqstp).rq_xprt), (*rqstp).rq_proc);
            gssrpc_svcerr_noproc(transp);
            return
        }
    }
    memset(&mut argument as *mut C2RustUnnamed_7 as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong);
    if Some((*(*transp).xp_ops).xp_getargs.expect("non-null function pointer")).expect("non-null function pointer")(transp,
                                                                                                                    xdr_argument,
                                                                                                                    &mut argument
                                                                                                                        as
                                                                                                                        *mut C2RustUnnamed_7
                                                                                                                        as
                                                                                                                        *mut libc::c_void)
           == 0 {
        gssrpc_svcerr_decode(transp);
        return
    }
    memset(&mut result as *mut C2RustUnnamed_6 as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong);
    retval =
        ::std::mem::transmute::<_,
                                fn(_: _, _: _, _: _)
                                    ->
                                        libc::c_int>(Some(local.expect("non-null function pointer")).expect("non-null function pointer"))(&mut argument,
                                                                                                                                          &mut result,
                                                                                                                                          rqstp);
    if retval != 0 &&
           gssrpc_svc_sendreply(transp, xdr_result,
                                &mut result as *mut C2RustUnnamed_6 as
                                    *mut libc::c_void as caddr_t) == 0 {
        krb5_klog_syslog(3 as libc::c_int,
                         b"WARNING! Unable to send function results, continuing.\x00"
                             as *const u8 as *const libc::c_char);
        gssrpc_svcerr_systemerr(transp);
    }
    if Some((*(*transp).xp_ops).xp_freeargs.expect("non-null function pointer")).expect("non-null function pointer")(transp,
                                                                                                                     xdr_argument,
                                                                                                                     &mut argument
                                                                                                                         as
                                                                                                                         *mut C2RustUnnamed_7
                                                                                                                         as
                                                                                                                         *mut libc::c_void)
           == 0 {
        krb5_klog_syslog(3 as libc::c_int,
                         b"WARNING! Unable to free arguments, continuing.\x00"
                             as *const u8 as *const libc::c_char);
    }
    if Some((*(*transp).xp_ops).xp_freeargs.expect("non-null function pointer")).expect("non-null function pointer")(transp,
                                                                                                                     xdr_result,
                                                                                                                     &mut result
                                                                                                                         as
                                                                                                                         *mut C2RustUnnamed_6
                                                                                                                         as
                                                                                                                         *mut libc::c_void)
           == 0 {
        krb5_klog_syslog(3 as libc::c_int,
                         b"WARNING! Unable to free results, continuing.\x00"
                             as *const u8 as *const libc::c_char);
    };
}
#[c2rust::src_loc = "273:1"]
unsafe extern "C" fn check_rpcsec_auth(mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut ctx: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut kctx: krb5_context = 0 as *mut _krb5_context;
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    let mut name: gss_name_t = 0 as *mut gss_name_struct;
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut ret: libc::c_int = 0;
    let mut success: libc::c_int = 0;
    let mut c1: *mut krb5_data = 0 as *mut krb5_data;
    let mut c2: *mut krb5_data = 0 as *mut krb5_data;
    let mut realm: *mut krb5_data = 0 as *mut krb5_data;
    let mut gss_str: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut slen: size_t = 0;
    let mut sdots: *mut libc::c_char = 0 as *mut libc::c_char;
    success = 0 as libc::c_int;
    handle = global_server_handle as kadm5_server_handle_t;
    if (*rqstp).rq_cred.oa_flavor != 6 as libc::c_int {
        return 0 as libc::c_int
    }
    ctx = (*rqstp).rq_svccred as gss_ctx_id_t;
    maj_stat =
        gss_inquire_context(&mut min_stat, ctx, 0 as *mut gss_name_t,
                            &mut name, 0 as *mut OM_uint32, 0 as *mut gss_OID,
                            0 as *mut OM_uint32, 0 as *mut libc::c_int,
                            0 as *mut libc::c_int);
    if maj_stat != 0 as libc::c_int as libc::c_uint {
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"check_rpcsec_auth: failed inquire_context, stat=%u\x00"
                                      as *const u8 as *const libc::c_char),
                         maj_stat);
        log_badauth(maj_stat, min_stat, (*rqstp).rq_xprt,
                    0 as *mut libc::c_char);
    } else {
        kctx = (*handle).context;
        ret = gss_to_krb5_name_1(rqstp, kctx, name, &mut princ, &mut gss_str);
        if !(ret == 0 as libc::c_int) {
            slen = gss_str.length;
            trunc_name(&mut slen, &mut sdots);
            /*
      * Since we accept with GSS_C_NO_NAME, the client can authenticate
      * against the entire kdb.  Therefore, ensure that the service
      * name is something reasonable.
      */
            if !((*princ).length != 2 as libc::c_int) {
                c1 =
                    if (0 as libc::c_int) < (*princ).length {
                        (*princ).data.offset(0 as libc::c_int as isize)
                    } else { 0 as *mut krb5_data };
                c2 =
                    if (1 as libc::c_int) < (*princ).length {
                        (*princ).data.offset(1 as libc::c_int as isize)
                    } else { 0 as *mut krb5_data };
                realm = &mut (*princ).realm;
                success =
                    (data_eq_string(*realm, (*handle).params.realm) != 0 &&
                         data_eq_string(*c1,
                                        b"kadmin\x00" as *const u8 as
                                            *const libc::c_char) != 0 &&
                         data_eq_string(*c2,
                                        b"history\x00" as *const u8 as
                                            *const libc::c_char) == 0) as
                        libc::c_int
            }
            if success == 0 {
                krb5_klog_syslog(3 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"bad service principal %.*s%s\x00"
                                              as *const u8 as
                                              *const libc::c_char),
                                 slen as libc::c_int,
                                 gss_str.value as *mut libc::c_char, sdots);
            }
            gss_release_buffer(&mut min_stat, &mut gss_str);
            krb5_free_principal(kctx, princ);
        }
    }
    gss_release_name(&mut min_stat, &mut name);
    return success;
}
#[no_mangle]
#[c2rust::src_loc = "338:1"]
pub unsafe extern "C" fn gss_to_krb5_name_1(mut rqstp: *mut svc_req,
                                            mut ctx: krb5_context,
                                            mut gss_name: gss_name_t,
                                            mut princ: *mut krb5_principal,
                                            mut gss_str: gss_buffer_t)
 -> libc::c_int {
    let mut status: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    let mut gss_type: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut success: libc::c_int = 0;
    status =
        gss_display_name(&mut minor_stat, gss_name, gss_str, &mut gss_type);
    if status != 0 as libc::c_int as libc::c_uint ||
           gss_type != gss_nt_krb5_name {
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"gss_to_krb5_name: failed display_name status %d\x00"
                                      as *const u8 as *const libc::c_char),
                         status);
        log_badauth(status, minor_stat, (*rqstp).rq_xprt,
                    0 as *mut libc::c_char);
        return 0 as libc::c_int
    }
    str =
        malloc((*gss_str).length.wrapping_add(1 as libc::c_int as
                                                  libc::c_ulong)) as
            *mut libc::c_char;
    if str.is_null() { return 0 as libc::c_int }
    *str = '\u{0}' as i32 as libc::c_char;
    strncat(str, (*gss_str).value as *const libc::c_char, (*gss_str).length);
    success =
        (krb5_parse_name(ctx, str, princ) == 0 as libc::c_int) as libc::c_int;
    free(str as *mut libc::c_void);
    return success;
}
