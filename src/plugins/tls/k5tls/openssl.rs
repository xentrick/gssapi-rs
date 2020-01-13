use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:30"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:30"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:30"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:30"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:30"]
pub mod pthreadtypes_h {
    #[c2rust::src_loc = "53:1"]
    pub type pthread_once_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:30"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:30"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:30"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:30"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "188:1"]
    pub type k5_os_nothread_once_t = libc::c_uchar;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "256:9"]
    pub struct k5_once_t {
        pub o: pthread_once_t,
        pub n: k5_os_nothread_once_t,
    }
    use super::pthreadtypes_h::pthread_once_t;
    extern "C" {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:30"]
pub mod k5_platform_h {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:30"]
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
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:30"]
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_tls_h::k5_tls_vtable_st;
    use super::k5_err_h::errinfo;
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
        #[c2rust::src_loc = "1202:8"]
        pub type _kdb_log_context;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:30"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-tls.h:32"]
pub mod k5_tls_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:16"]
    pub struct k5_tls_vtable_st {
        pub setup: k5_tls_setup_fn,
        pub write: k5_tls_write_fn,
        pub read: k5_tls_read_fn,
        pub free_handle: k5_tls_free_handle_fn,
    }
    /* Release a handle.  Do not pass a null pointer. */
    #[c2rust::src_loc = "92:1"]
    pub type k5_tls_free_handle_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: k5_tls_handle) -> ()>;
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-tls.h - internal pluggable interface for TLS */
/*
 * Copyright (C) 2014 by the Massachusetts Institute of Technology.
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
 * This internal pluggable interface allows libkrb5 to load an in-tree module
 * providing TLS support at runtime.  It is currently tailored for the needs of
 * the OpenSSL module as used for HTTP proxy support.  As an internal
 * interface, it can be changed to fit different implementations and consumers
 * without regard for backward compatibility.
 */
    /* An abstract type for localauth module data. */
    #[c2rust::src_loc = "47:1"]
    pub type k5_tls_handle = *mut k5_tls_handle_st;
    /*
 * Read up to data_size bytes of data using TLS.  Return DATA_READ and set
 * *len_out if any data is read.  Return DONE if there is no more data to be
 * read on the connection, WANT_READ or WANT_WRITE if the underlying socket
 * must be readable or writable to continue, and ERROR_TLS if the TLS channel
 * or underlying socket experienced an error.
 *
 * After DATA_READ, there may still be pending buffered data to read.  The
 * caller must call this method again with additional buffer space before
 * selecting for reading on the underlying socket.
 */
    #[c2rust::src_loc = "87:1"]
    pub type k5_tls_read_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: k5_tls_handle,
                                    _: *mut libc::c_void, _: size_t,
                                    _: *mut size_t) -> k5_tls_status>;
    #[c2rust::src_loc = "49:9"]
    pub type k5_tls_status = libc::c_uint;
    #[c2rust::src_loc = "50:45"]
    pub const ERROR_TLS: k5_tls_status = 4;
    #[c2rust::src_loc = "50:33"]
    pub const WANT_WRITE: k5_tls_status = 3;
    #[c2rust::src_loc = "50:22"]
    pub const WANT_READ: k5_tls_status = 2;
    #[c2rust::src_loc = "50:16"]
    pub const DONE: k5_tls_status = 1;
    #[c2rust::src_loc = "50:5"]
    pub const DATA_READ: k5_tls_status = 0;
    /*
 * Write len bytes of data using TLS.  Return DONE if writing is complete,
 * WANT_READ or WANT_WRITE if the underlying socket must be readable or
 * writable to continue, and ERROR_TLS if the TLS channel or underlying socket
 * experienced an error.  After WANT_READ or WANT_WRITE, the operation will be
 * retried with the same arguments even if some data has already been written.
 * (OpenSSL makes this contract easy to fulfill.  For other implementations we
 * might want to change it.)
 */
    #[c2rust::src_loc = "72:1"]
    pub type k5_tls_write_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: k5_tls_handle,
                                    _: *const libc::c_void, _: size_t)
                   -> k5_tls_status>;
    /*
 * Create a handle for fd, where the server certificate must match servername
 * and be trusted according to anchors.  anchors is a null-terminated list
 * using the DIR:/FILE:/ENV: syntax borrowed from PKINIT.  If anchors is null,
 * use the system default trust anchors.
 */
    #[c2rust::src_loc = "59:1"]
    pub type k5_tls_setup_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: libc::c_int,
                                    _: *const libc::c_char,
                                    _: *mut *mut libc::c_char,
                                    _: *mut k5_tls_handle)
                   -> krb5_error_code>;
    /* All functions are mandatory unless they are all null, in which case the
 * caller should assume that TLS is unsupported. */
    #[c2rust::src_loc = "97:1"]
    pub type k5_tls_vtable = *mut k5_tls_vtable_st;
    use super::krb5_h::{krb5_context, krb5_error_code};
    use super::k5_tls_handle_st;
    use super::stddef_h::size_t;
    /* K5_TLS_H */
}
#[c2rust::header_src = "/usr/include/openssl/ossl_typ.h:35"]
pub mod ossl_typ_h {
    #[c2rust::src_loc = "147:1"]
    pub type SSL = ssl_st;
    #[c2rust::src_loc = "40:1"]
    pub type ASN1_INTEGER = asn1_string_st;
    #[c2rust::src_loc = "41:1"]
    pub type ASN1_ENUMERATED = asn1_string_st;
    #[c2rust::src_loc = "42:1"]
    pub type ASN1_BIT_STRING = asn1_string_st;
    #[c2rust::src_loc = "43:1"]
    pub type ASN1_OCTET_STRING = asn1_string_st;
    #[c2rust::src_loc = "44:1"]
    pub type ASN1_PRINTABLESTRING = asn1_string_st;
    #[c2rust::src_loc = "45:1"]
    pub type ASN1_T61STRING = asn1_string_st;
    #[c2rust::src_loc = "46:1"]
    pub type ASN1_IA5STRING = asn1_string_st;
    #[c2rust::src_loc = "47:1"]
    pub type ASN1_GENERALSTRING = asn1_string_st;
    #[c2rust::src_loc = "48:1"]
    pub type ASN1_UNIVERSALSTRING = asn1_string_st;
    #[c2rust::src_loc = "49:1"]
    pub type ASN1_BMPSTRING = asn1_string_st;
    #[c2rust::src_loc = "50:1"]
    pub type ASN1_UTCTIME = asn1_string_st;
    #[c2rust::src_loc = "52:1"]
    pub type ASN1_GENERALIZEDTIME = asn1_string_st;
    #[c2rust::src_loc = "53:1"]
    pub type ASN1_VISIBLESTRING = asn1_string_st;
    #[c2rust::src_loc = "54:1"]
    pub type ASN1_UTF8STRING = asn1_string_st;
    #[c2rust::src_loc = "55:1"]
    pub type ASN1_STRING = asn1_string_st;
    #[c2rust::src_loc = "56:1"]
    pub type ASN1_BOOLEAN = libc::c_int;
    #[c2rust::src_loc = "60:1"]
    pub type ASN1_OBJECT = asn1_object_st;
    #[c2rust::src_loc = "79:1"]
    pub type BIO = bio_st;
    #[c2rust::src_loc = "89:1"]
    pub type EVP_CIPHER = evp_cipher_st;
    #[c2rust::src_loc = "93:1"]
    pub type EVP_PKEY = evp_pkey_st;
    #[c2rust::src_loc = "120:1"]
    pub type X509 = x509_st;
    #[c2rust::src_loc = "121:1"]
    pub type X509_ALGOR = X509_algor_st;
    #[c2rust::src_loc = "122:1"]
    pub type X509_CRL = X509_crl_st;
    #[c2rust::src_loc = "125:1"]
    pub type X509_NAME = X509_name_st;
    #[c2rust::src_loc = "127:1"]
    pub type X509_STORE = x509_store_st;
    #[c2rust::src_loc = "128:1"]
    pub type X509_STORE_CTX = x509_store_ctx_st;
    #[c2rust::src_loc = "141:1"]
    pub type OPENSSL_INIT_SETTINGS = ossl_init_settings_st;
    #[c2rust::src_loc = "148:1"]
    pub type SSL_CTX = ssl_ctx_st;
    #[c2rust::src_loc = "163:1"]
    pub type CRYPTO_EX_DATA = crypto_ex_data_st;
    use super::asn1_h::asn1_string_st;
    use super::x509_h::X509_algor_st;
    use super::crypto_h::crypto_ex_data_st;
    extern "C" {
        #[c2rust::src_loc = "147:16"]
        pub type ssl_st;
        #[c2rust::src_loc = "60:16"]
        pub type asn1_object_st;
        #[c2rust::src_loc = "79:16"]
        pub type bio_st;
        #[c2rust::src_loc = "89:16"]
        pub type evp_cipher_st;
        #[c2rust::src_loc = "93:16"]
        pub type evp_pkey_st;
        #[c2rust::src_loc = "120:16"]
        pub type x509_st;
        #[c2rust::src_loc = "122:16"]
        pub type X509_crl_st;
        #[c2rust::src_loc = "125:16"]
        pub type X509_name_st;
        #[c2rust::src_loc = "127:16"]
        pub type x509_store_st;
        #[c2rust::src_loc = "128:16"]
        pub type x509_store_ctx_st;
        #[c2rust::src_loc = "141:16"]
        pub type ossl_init_settings_st;
        #[c2rust::src_loc = "148:16"]
        pub type ssl_ctx_st;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:30"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:30"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:30"]
pub mod plugin_h {
    #[c2rust::src_loc = "34:1"]
    pub type krb5_plugin_vtable = *mut krb5_plugin_vtable_st;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2010 by the Massachusetts Institute of Technology.
 * All rights reserved.
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
        /* Generic declarations for dynamic modules implementing krb5 plugin
 * modules. */
        /* krb5_plugin_vtable is an abstract type.  Module initvt functions will cast
 * it to the appropriate interface-specific vtable type. */
        #[c2rust::src_loc = "34:16"]
        pub type krb5_plugin_vtable_st;
    }
    /* KRB5_PLUGIN_H */
}
#[c2rust::header_src = "/usr/include/netinet/in.h:30"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "212:8"]
    pub struct in6_addr {
        pub __in6_u: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [uint8_t; 16],
        pub __u6_addr16: [uint16_t; 8],
        pub __u6_addr32: [uint32_t; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
}
#[c2rust::header_src = "/usr/include/openssl/asn1.h:36"]
pub mod asn1_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "146:8"]
    pub struct asn1_string_st {
        pub length: libc::c_int,
        pub type_0: libc::c_int,
        pub data: *mut libc::c_uchar,
        pub flags: libc::c_long,
    }
    #[c2rust::src_loc = "444:1"]
    pub type ASN1_TYPE = asn1_type_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "444:16"]
    pub struct asn1_type_st {
        pub type_0: libc::c_int,
        pub value: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "446:5"]
    pub union C2RustUnnamed_0 {
        pub ptr: *mut libc::c_char,
        pub boolean: ASN1_BOOLEAN,
        pub asn1_string: *mut ASN1_STRING,
        pub object: *mut ASN1_OBJECT,
        pub integer: *mut ASN1_INTEGER,
        pub enumerated: *mut ASN1_ENUMERATED,
        pub bit_string: *mut ASN1_BIT_STRING,
        pub octet_string: *mut ASN1_OCTET_STRING,
        pub printablestring: *mut ASN1_PRINTABLESTRING,
        pub t61string: *mut ASN1_T61STRING,
        pub ia5string: *mut ASN1_IA5STRING,
        pub generalstring: *mut ASN1_GENERALSTRING,
        pub bmpstring: *mut ASN1_BMPSTRING,
        pub universalstring: *mut ASN1_UNIVERSALSTRING,
        pub utctime: *mut ASN1_UTCTIME,
        pub generalizedtime: *mut ASN1_GENERALIZEDTIME,
        pub visiblestring: *mut ASN1_VISIBLESTRING,
        pub utf8string: *mut ASN1_UTF8STRING,
        pub set: *mut ASN1_STRING,
        pub sequence: *mut ASN1_STRING,
        pub asn1_value: *mut ASN1_VALUE,
    }
    #[c2rust::src_loc = "213:1"]
    pub type ASN1_VALUE = ASN1_VALUE_st;
    use super::ossl_typ_h::{ASN1_BOOLEAN, ASN1_STRING, ASN1_OBJECT,
                            ASN1_INTEGER, ASN1_ENUMERATED, ASN1_BIT_STRING,
                            ASN1_OCTET_STRING, ASN1_PRINTABLESTRING,
                            ASN1_T61STRING, ASN1_IA5STRING,
                            ASN1_GENERALSTRING, ASN1_BMPSTRING,
                            ASN1_UNIVERSALSTRING, ASN1_UTCTIME,
                            ASN1_GENERALIZEDTIME, ASN1_VISIBLESTRING,
                            ASN1_UTF8STRING};
    extern "C" {
        #[c2rust::src_loc = "213:16"]
        pub type ASN1_VALUE_st;
        #[no_mangle]
        #[c2rust::src_loc = "596:24"]
        pub fn ASN1_OCTET_STRING_new() -> *mut ASN1_OCTET_STRING;
        #[no_mangle]
        #[c2rust::src_loc = "596:1"]
        pub fn ASN1_OCTET_STRING_free(a: *mut ASN1_OCTET_STRING);
        #[no_mangle]
        #[c2rust::src_loc = "598:1"]
        pub fn ASN1_OCTET_STRING_cmp(a: *const ASN1_OCTET_STRING,
                                     b: *const ASN1_OCTET_STRING)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "600:1"]
        pub fn ASN1_OCTET_STRING_set(str: *mut ASN1_OCTET_STRING,
                                     data: *const libc::c_uchar,
                                     len: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "734:1"]
        pub fn ASN1_STRING_to_UTF8(out: *mut *mut libc::c_uchar,
                                   in_0: *const ASN1_STRING) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/openssl/x509.h:36"]
pub mod x509_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "59:8"]
    pub struct X509_algor_st {
        pub algorithm: *mut ASN1_OBJECT,
        pub parameter: *mut ASN1_TYPE,
    }
    #[c2rust::src_loc = "81:1"]
    pub type X509_EXTENSION = X509_extension_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "230:16"]
    pub struct private_key_st {
        pub version: libc::c_int,
        pub enc_algor: *mut X509_ALGOR,
        pub enc_pkey: *mut ASN1_OCTET_STRING,
        pub dec_pkey: *mut EVP_PKEY,
        pub key_length: libc::c_int,
        pub key_data: *mut libc::c_char,
        pub key_free: libc::c_int,
        pub cipher: EVP_CIPHER_INFO,
    }
    #[c2rust::src_loc = "230:1"]
    pub type X509_PKEY = private_key_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "245:16"]
    pub struct X509_info_st {
        pub x509: *mut X509,
        pub crl: *mut X509_CRL,
        pub x_pkey: *mut X509_PKEY,
        pub enc_cipher: EVP_CIPHER_INFO,
        pub enc_len: libc::c_int,
        pub enc_data: *mut libc::c_char,
    }
    #[c2rust::src_loc = "245:1"]
    pub type X509_INFO = X509_info_st;
    #[c2rust::src_loc = "254:1"]
    pub type sk_X509_INFO_freefunc
        =
        Option<unsafe extern "C" fn(_: *mut X509_INFO) -> ()>;
    #[inline]
    #[c2rust::src_loc = "254:1"]
    pub unsafe extern "C" fn sk_X509_INFO_num(mut sk:
                                                  *const stack_st_X509_INFO)
     -> libc::c_int {
        return OPENSSL_sk_num(sk as *const OPENSSL_STACK);
    }
    #[inline]
    #[c2rust::src_loc = "254:1"]
    pub unsafe extern "C" fn sk_X509_INFO_value(mut sk:
                                                    *const stack_st_X509_INFO,
                                                mut idx: libc::c_int)
     -> *mut X509_INFO {
        return OPENSSL_sk_value(sk as *const OPENSSL_STACK, idx) as
                   *mut X509_INFO;
    }
    #[inline]
    #[c2rust::src_loc = "254:1"]
    pub unsafe extern "C" fn sk_X509_INFO_pop_free(mut sk:
                                                       *mut stack_st_X509_INFO,
                                                   mut freefunc:
                                                       sk_X509_INFO_freefunc) {
        OPENSSL_sk_pop_free(sk as *mut OPENSSL_STACK,
                            ::std::mem::transmute::<sk_X509_INFO_freefunc,
                                                    OPENSSL_sk_freefunc>(freefunc));
    }
    use super::ossl_typ_h::{ASN1_OBJECT, X509_ALGOR, ASN1_OCTET_STRING,
                            EVP_PKEY, X509, X509_CRL, X509_NAME, BIO};
    use super::asn1_h::ASN1_TYPE;
    use super::evp_h::EVP_CIPHER_INFO;
    use super::stack_h::{OPENSSL_sk_num, OPENSSL_STACK, OPENSSL_sk_value,
                         OPENSSL_sk_pop_free, OPENSSL_sk_freefunc};
    extern "C" {
        #[c2rust::src_loc = "81:16"]
        pub type X509_extension_st;
        #[c2rust::src_loc = "254:1"]
        pub type stack_st_X509_INFO;
        #[no_mangle]
        #[c2rust::src_loc = "348:1"]
        pub fn X509_verify_cert_error_string(n: libc::c_long)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "642:1"]
        pub fn X509_get_subject_name(a: *const X509) -> *mut X509_NAME;
        #[no_mangle]
        #[c2rust::src_loc = "608:1"]
        pub fn X509_INFO_free(a: *mut X509_INFO);
        #[no_mangle]
        #[c2rust::src_loc = "789:1"]
        pub fn X509_NAME_print_ex(out: *mut BIO, nm: *const X509_NAME,
                                  indent: libc::c_int, flags: libc::c_ulong)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "802:1"]
        pub fn X509_NAME_get_text_by_NID(name: *mut X509_NAME,
                                         nid: libc::c_int,
                                         buf: *mut libc::c_char,
                                         len: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "861:1"]
        pub fn X509_get_ext_by_NID(x: *const X509, nid: libc::c_int,
                                   lastpos: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "864:1"]
        pub fn X509_get_ext(x: *const X509, loc: libc::c_int)
         -> *mut X509_EXTENSION;
    }
}
#[c2rust::header_src = "/usr/include/openssl/x509v3.h:38"]
pub mod x509v3_h {
    #[c2rust::src_loc = "167:1"]
    pub type GENERAL_NAMES = stack_st_GENERAL_NAME;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:16"]
    pub struct otherName_st {
        pub type_id: *mut ASN1_OBJECT,
        pub value: *mut ASN1_TYPE,
    }
    #[c2rust::src_loc = "113:1"]
    pub type OTHERNAME = otherName_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:16"]
    pub struct EDIPartyName_st {
        pub nameAssigner: *mut ASN1_STRING,
        pub partyName: *mut ASN1_STRING,
    }
    #[c2rust::src_loc = "118:1"]
    pub type EDIPARTYNAME = EDIPartyName_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "123:16"]
    pub struct GENERAL_NAME_st {
        pub type_0: libc::c_int,
        pub d: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "134:5"]
    pub union C2RustUnnamed_1 {
        pub ptr: *mut libc::c_char,
        pub otherName: *mut OTHERNAME,
        pub rfc822Name: *mut ASN1_IA5STRING,
        pub dNSName: *mut ASN1_IA5STRING,
        pub x400Address: *mut ASN1_TYPE,
        pub directoryName: *mut X509_NAME,
        pub ediPartyName: *mut EDIPARTYNAME,
        pub uniformResourceIdentifier: *mut ASN1_IA5STRING,
        pub iPAddress: *mut ASN1_OCTET_STRING,
        pub registeredID: *mut ASN1_OBJECT,
        pub ip: *mut ASN1_OCTET_STRING,
        pub dirn: *mut X509_NAME,
        pub ia5: *mut ASN1_IA5STRING,
        pub rid: *mut ASN1_OBJECT,
        pub other: *mut ASN1_TYPE,
    }
    #[c2rust::src_loc = "123:1"]
    pub type GENERAL_NAME = GENERAL_NAME_st;
    #[c2rust::src_loc = "166:1"]
    pub type sk_GENERAL_NAME_freefunc
        =
        Option<unsafe extern "C" fn(_: *mut GENERAL_NAME) -> ()>;
    #[inline]
    #[c2rust::src_loc = "166:1"]
    pub unsafe extern "C" fn sk_GENERAL_NAME_num(mut sk:
                                                     *const stack_st_GENERAL_NAME)
     -> libc::c_int {
        return OPENSSL_sk_num(sk as *const OPENSSL_STACK);
    }
    #[inline]
    #[c2rust::src_loc = "166:1"]
    pub unsafe extern "C" fn sk_GENERAL_NAME_value(mut sk:
                                                       *const stack_st_GENERAL_NAME,
                                                   mut idx: libc::c_int)
     -> *mut GENERAL_NAME {
        return OPENSSL_sk_value(sk as *const OPENSSL_STACK, idx) as
                   *mut GENERAL_NAME;
    }
    #[inline]
    #[c2rust::src_loc = "166:1"]
    pub unsafe extern "C" fn sk_GENERAL_NAME_pop_free(mut sk:
                                                          *mut stack_st_GENERAL_NAME,
                                                      mut freefunc:
                                                          sk_GENERAL_NAME_freefunc) {
        OPENSSL_sk_pop_free(sk as *mut OPENSSL_STACK,
                            ::std::mem::transmute::<sk_GENERAL_NAME_freefunc,
                                                    OPENSSL_sk_freefunc>(freefunc));
    }
    use super::ossl_typ_h::{ASN1_OBJECT, ASN1_STRING, ASN1_IA5STRING,
                            X509_NAME, ASN1_OCTET_STRING};
    use super::asn1_h::ASN1_TYPE;
    use super::stack_h::{OPENSSL_sk_num, OPENSSL_STACK, OPENSSL_sk_value,
                         OPENSSL_sk_pop_free, OPENSSL_sk_freefunc};
    use super::x509_h::X509_EXTENSION;
    extern "C" {
        #[c2rust::src_loc = "166:1"]
        pub type stack_st_GENERAL_NAME;
        #[no_mangle]
        #[c2rust::src_loc = "469:1"]
        pub fn GENERAL_NAME_free(a: *mut GENERAL_NAME);
        #[no_mangle]
        #[c2rust::src_loc = "624:1"]
        pub fn X509V3_EXT_d2i(ext: *mut X509_EXTENSION) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/openssl/crypto.h:35"]
pub mod crypto_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:8"]
    pub struct crypto_ex_data_st {
        pub sk: *mut stack_st_void,
    }
    #[c2rust::src_loc = "166:1"]
    pub type CRYPTO_EX_new
        =
        unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                             _: *mut CRYPTO_EX_DATA, _: libc::c_int,
                             _: libc::c_long, _: *mut libc::c_void) -> ();
    #[c2rust::src_loc = "168:1"]
    pub type CRYPTO_EX_free
        =
        unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                             _: *mut CRYPTO_EX_DATA, _: libc::c_int,
                             _: libc::c_long, _: *mut libc::c_void) -> ();
    #[c2rust::src_loc = "170:1"]
    pub type CRYPTO_EX_dup
        =
        unsafe extern "C" fn(_: *mut CRYPTO_EX_DATA, _: *const CRYPTO_EX_DATA,
                             _: *mut libc::c_void, _: libc::c_int,
                             _: libc::c_long, _: *mut libc::c_void)
            -> libc::c_int;
    use super::ossl_typ_h::{CRYPTO_EX_DATA, OPENSSL_INIT_SETTINGS};
    use super::stdint_uintn_h::uint64_t;
    extern "C" {
        #[c2rust::src_loc = "87:5"]
        pub type stack_st_void;
        #[no_mangle]
        #[c2rust::src_loc = "172:8"]
        pub fn CRYPTO_get_ex_new_index(class_index: libc::c_int,
                                       argl: libc::c_long,
                                       argp: *mut libc::c_void,
                                       new_func: Option<CRYPTO_EX_new>,
                                       dup_func: Option<CRYPTO_EX_dup>,
                                       free_func: Option<CRYPTO_EX_free>)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "271:1"]
        pub fn CRYPTO_free(ptr: *mut libc::c_void, file: *const libc::c_char,
                           line: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "388:1"]
        pub fn OPENSSL_init_crypto(opts: uint64_t,
                                   settings: *const OPENSSL_INIT_SETTINGS)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/openssl/stack.h:35"]
pub mod stack_h {
    #[c2rust::src_loc = "17:1"]
    pub type OPENSSL_STACK = stack_st;
    #[c2rust::src_loc = "20:1"]
    pub type OPENSSL_sk_freefunc
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
    extern "C" {
        #[c2rust::src_loc = "17:16"]
        pub type stack_st;
        #[no_mangle]
        #[c2rust::src_loc = "23:1"]
        pub fn OPENSSL_sk_num(_: *const OPENSSL_STACK) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "24:1"]
        pub fn OPENSSL_sk_value(_: *const OPENSSL_STACK, _: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "33:1"]
        pub fn OPENSSL_sk_pop_free(st: *mut OPENSSL_STACK,
                                   func:
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut libc::c_void)
                                                  -> ()>);
    }
}
#[c2rust::header_src = "/usr/include/openssl/bio.h:35"]
pub mod bio_h {
    #[c2rust::src_loc = "249:1"]
    pub type BIO_METHOD = bio_method_st;
    use super::ossl_typ_h::BIO;
    extern "C" {
        #[c2rust::src_loc = "249:16"]
        pub type bio_method_st;
        #[no_mangle]
        #[c2rust::src_loc = "548:1"]
        pub fn BIO_new(type_0: *const BIO_METHOD) -> *mut BIO;
        #[no_mangle]
        #[c2rust::src_loc = "549:1"]
        pub fn BIO_free(a: *mut BIO) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn BIO_ctrl(bp: *mut BIO, cmd: libc::c_int, larg: libc::c_long,
                        parg: *mut libc::c_void) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "588:1"]
        pub fn BIO_s_mem() -> *const BIO_METHOD;
    }
}
#[c2rust::header_src = "/usr/include/openssl/evp.h:36"]
pub mod evp_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "396:16"]
    pub struct evp_cipher_info_st {
        pub cipher: *const EVP_CIPHER,
        pub iv: [libc::c_uchar; 16],
    }
    #[c2rust::src_loc = "396:1"]
    pub type EVP_CIPHER_INFO = evp_cipher_info_st;
    use super::ossl_typ_h::EVP_CIPHER;
}
#[c2rust::header_src = "/usr/include/openssl/pem.h:36"]
pub mod pem_h {
    #[c2rust::src_loc = "231:1"]
    pub type pem_password_cb
        =
        unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int,
                             _: libc::c_int, _: *mut libc::c_void)
            -> libc::c_int;
    use super::FILE_h::FILE;
    use super::x509_h::stack_st_X509_INFO;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "274:1"]
        pub fn PEM_X509_INFO_read(fp: *mut FILE, sk: *mut stack_st_X509_INFO,
                                  cb: Option<pem_password_cb>,
                                  u: *mut libc::c_void)
         -> *mut stack_st_X509_INFO;
    }
}
#[c2rust::header_src = "/usr/include/openssl/ssl.h:36"]
pub mod ssl_h {
    #[c2rust::src_loc = "211:1"]
    pub type SSL_METHOD = ssl_method_st;
    #[c2rust::src_loc = "294:1"]
    pub type SSL_verify_cb
        =
        Option<unsafe extern "C" fn(_: libc::c_int, _: *mut X509_STORE_CTX)
                   -> libc::c_int>;
    use super::ossl_typ_h::{X509_STORE_CTX, OPENSSL_INIT_SETTINGS, SSL,
                            SSL_CTX, X509_STORE};
    use super::stdint_uintn_h::uint64_t;
    extern "C" {
        #[c2rust::src_loc = "211:16"]
        pub type ssl_method_st;
        #[no_mangle]
        #[c2rust::src_loc = "2364:1"]
        pub fn OPENSSL_init_ssl(opts: uint64_t,
                                settings: *const OPENSSL_INIT_SETTINGS)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "2026:8"]
        pub fn SSL_get_ex_data_X509_STORE_CTX_idx() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "2016:1"]
        pub fn SSL_get_ex_data(ssl: *const SSL, idx: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "2015:8"]
        pub fn SSL_set_ex_data(ssl: *mut SSL, idx: libc::c_int,
                               data: *mut libc::c_void) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1983:8"]
        pub fn SSL_CTX_set_default_verify_paths(ctx: *mut SSL_CTX)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1952:1"]
        pub fn SSL_set_connect_state(s: *mut SSL);
        #[no_mangle]
        #[c2rust::src_loc = "1874:8"]
        pub fn TLS_client_method() -> *const SSL_METHOD;
        #[no_mangle]
        #[c2rust::src_loc = "1855:8"]
        pub fn SSL_get_error(s: *const SSL, ret_code: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1844:1"]
        pub fn SSL_ctrl(ssl: *mut SSL, cmd: libc::c_int, larg: libc::c_long,
                        parg: *mut libc::c_void) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "1840:8"]
        pub fn SSL_write(ssl: *mut SSL, buf: *const libc::c_void,
                         num: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1829:8"]
        pub fn SSL_read(ssl: *mut SSL, buf: *mut libc::c_void,
                        num: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1815:1"]
        pub fn SSL_free(ssl: *mut SSL);
        #[no_mangle]
        #[c2rust::src_loc = "1723:1"]
        pub fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
        #[no_mangle]
        #[c2rust::src_loc = "1686:1"]
        pub fn SSL_CTX_set_verify(ctx: *mut SSL_CTX, mode: libc::c_int,
                                  callback: SSL_verify_cb);
        #[no_mangle]
        #[c2rust::src_loc = "1539:8"]
        pub fn SSL_set_fd(s: *mut SSL, fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1508:8"]
        pub fn SSL_CTX_get_cert_store(_: *const SSL_CTX) -> *mut X509_STORE;
        #[no_mangle]
        #[c2rust::src_loc = "1505:1"]
        pub fn SSL_CTX_free(_: *mut SSL_CTX);
        #[no_mangle]
        #[c2rust::src_loc = "1503:8"]
        pub fn SSL_CTX_new(meth: *const SSL_METHOD) -> *mut SSL_CTX;
        #[no_mangle]
        #[c2rust::src_loc = "583:1"]
        pub fn SSL_CTX_get_options(ctx: *const SSL_CTX) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "587:1"]
        pub fn SSL_CTX_set_options(ctx: *mut SSL_CTX, op: libc::c_ulong)
         -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/bits/dirent.h:39"]
pub mod dirent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "22:8"]
    pub struct dirent {
        pub d_ino: __ino_t,
        pub d_off: __off_t,
        pub d_reclen: libc::c_ushort,
        pub d_type: libc::c_uchar,
        pub d_name: [libc::c_char; 256],
    }
    use super::types_h::{__ino_t, __off_t};
}
#[c2rust::header_src = "/usr/include/dirent.h:39"]
pub mod include_dirent_h {
    #[c2rust::src_loc = "127:1"]
    pub type DIR = __dirstream;
    use super::dirent_h::dirent;
    extern "C" {
        #[c2rust::src_loc = "127:16"]
        pub type __dirstream;
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn opendir(__name: *const libc::c_char) -> *mut DIR;
        #[no_mangle]
        #[c2rust::src_loc = "149:1"]
        pub fn closedir(__dirp: *mut DIR) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "162:1"]
        pub fn readdir(__dirp: *mut DIR) -> *mut dirent;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:30"]
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
#[c2rust::header_src = "/usr/include/string.h:30"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "90:14"]
        pub fn memchr(_: *const libc::c_void, _: libc::c_int,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "272:15"]
        pub fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:30"]
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
#[c2rust::header_src = "/usr/include/stdio.h:30"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:30"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/arpa/inet.h:30"]
pub mod inet_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "58:1"]
        pub fn inet_pton(__af: libc::c_int, __cp: *const libc::c_char,
                         __buf: *mut libc::c_void) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-trace.h:30"]
pub mod k5_trace_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::krb5_context;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-trace.h */
/*
 * Copyright (C) 2010 by the Massachusetts Institute of Technology.
 * All rights reserved.
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
 * This header contains trace macro definitions, which map trace points within
 * the code to krb5int_trace() calls with descriptive text strings.
 *
 * A new trace macro must be defined in this file for each new location to
 * be traced; the TRACE() macro should never be used directly.  This keeps
 * the tracing logic centralized in one place, to facilitate integration with
 * alternate tracing backends such as DTrace.
 *
 * Trace logging is intended to aid power users in diagnosing configuration
 * problems by showing what's going on behind the scenes of complex operations.
 * Although trace logging is sometimes useful to developers, it is not intended
 * as a replacement for a debugger, and it is not desirable to drown the user
 * in output.  Observe the following guidelines when adding trace points:
 *
 *   - Avoid mentioning function or variable names in messages.
 *
 *   - Try to convey what decisions are being made and what external inputs
 *     they are based on, not the process of making decisions.
 *
 *   - It is generally not necessary to trace before returning an unrecoverable
 *     error.  If an error code is unclear by itself, make it clearer with
 *     krb5_set_error_message().
 *
 *   - Keep macros simple.  Add format specifiers to krb5int_trace's formatter
 *     as necessary (and document them here) instead of transforming macro
 *     arguments.
 *
 *   - Like printf, the trace formatter interface is not type-safe.  Check your
 *     formats carefully.  Cast integral arguments to the appropriate type if
 *     they do not already patch.
 *
 * The following specifiers are supported by the formatter (see the
 * implementation in lib/krb5/os/trace.c for details):
 *
 *   {int}         int, in decimal
 *   {long}        long, in decimal
 *   {str}         const char *, display as C string
 *   {lenstr}      size_t and const char *, as a counted string
 *   {hexlenstr}   size_t and const char *, as hex bytes
 *   {hashlenstr}  size_t and const char *, as four-character hex hash
 *   {raddr}       struct remote_address *, show socket type, address, port
 *   {data}        krb5_data *, display as counted string
 *   {hexdata}     krb5_data *, display as hex bytes
 *   {errno}       int, display as number/errorstring
 *   {kerr}        krb5_error_code, display as number/errorstring
 *   {keyblock}    const krb5_keyblock *, display enctype and hash of key
 *   {key}         krb5_key, display enctype and hash of key
 *   {cksum}       const krb5_checksum *, display cksumtype and hex checksum
 *   {princ}       krb5_principal, unparse and display
 *   {ptype}       krb5_int32, krb5_principal type, display name
 *   {patype}      krb5_preauthtype, a single padata type number
 *   {patypes}     krb5_pa_data **, display list of padata type numbers
 *   {etype}       krb5_enctype, display shortest name of enctype
 *   {etypes}      krb5_enctype *, display list of enctypes
 *   {ccache}      krb5_ccache, display type:name
 *   {keytab}      krb5_keytab, display name
 *   {creds}       krb5_creds *, display clientprinc -> serverprinc
 */
        #[no_mangle]
        #[c2rust::src_loc = "94:1"]
        pub fn krb5int_trace(context: krb5_context, fmt: *const libc::c_char,
                             _: ...);
    }
    /* K5_TRACE_H */
    /* DISABLE_TRACING */
    /* Try to optimize away argument evaluation and function call when we're not
 * tracing, if this source file knows the internals of the context. */
}
#[c2rust::header_src = "/usr/include/openssl/err.h:35"]
pub mod err_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "237:1"]
        pub fn ERR_error_string_n(e: libc::c_ulong, buf: *mut libc::c_char,
                                  len: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "223:1"]
        pub fn ERR_get_error() -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/openssl/x509_vfy.h:36"]
pub mod x509_vfy_h {
    use super::ossl_typ_h::{X509_STORE, X509, X509_STORE_CTX};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "276:1"]
        pub fn X509_STORE_set_flags(ctx: *mut X509_STORE,
                                    flags: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "447:1"]
        pub fn X509_STORE_add_cert(ctx: *mut X509_STORE, x: *mut X509)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "488:1"]
        pub fn X509_STORE_CTX_get_ex_data(ctx: *mut X509_STORE_CTX,
                                          idx: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "489:1"]
        pub fn X509_STORE_CTX_get_error(ctx: *mut X509_STORE_CTX)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "491:1"]
        pub fn X509_STORE_CTX_get_error_depth(ctx: *mut X509_STORE_CTX)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "493:1"]
        pub fn X509_STORE_CTX_get_current_cert(ctx: *mut X509_STORE_CTX)
         -> *mut X509;
    }
}
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                        __uint64_t, __ino_t, __off_t, __off64_t, __ssize_t};
pub use self::sys_types_h::ssize_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::pthreadtypes_h::pthread_once_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::k5_thread_h::{k5_os_nothread_once_t, k5_once_t, k5_once};
pub use self::k5_platform_h::k5_init_t;
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_enctype, krb5_flags,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _profile_t};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, plugin_mapping, _kdb_log_context,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_tls_h::{k5_tls_vtable_st, k5_tls_free_handle_fn,
                         k5_tls_handle, k5_tls_read_fn, k5_tls_status,
                         ERROR_TLS, WANT_WRITE, WANT_READ, DONE, DATA_READ,
                         k5_tls_write_fn, k5_tls_setup_fn, k5_tls_vtable};
pub use self::ossl_typ_h::{SSL, ASN1_INTEGER, ASN1_ENUMERATED,
                           ASN1_BIT_STRING, ASN1_OCTET_STRING,
                           ASN1_PRINTABLESTRING, ASN1_T61STRING,
                           ASN1_IA5STRING, ASN1_GENERALSTRING,
                           ASN1_UNIVERSALSTRING, ASN1_BMPSTRING, ASN1_UTCTIME,
                           ASN1_GENERALIZEDTIME, ASN1_VISIBLESTRING,
                           ASN1_UTF8STRING, ASN1_STRING, ASN1_BOOLEAN,
                           ASN1_OBJECT, BIO, EVP_CIPHER, EVP_PKEY, X509,
                           X509_ALGOR, X509_CRL, X509_NAME, X509_STORE,
                           X509_STORE_CTX, OPENSSL_INIT_SETTINGS, SSL_CTX,
                           CRYPTO_EX_DATA, ssl_st, asn1_object_st, bio_st,
                           evp_cipher_st, evp_pkey_st, x509_st, X509_crl_st,
                           X509_name_st, x509_store_st, x509_store_ctx_st,
                           ossl_init_settings_st, ssl_ctx_st};
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_vtable_st};
pub use self::in_h::{in6_addr, C2RustUnnamed, in_addr, in_addr_t};
pub use self::asn1_h::{asn1_string_st, ASN1_TYPE, asn1_type_st,
                       C2RustUnnamed_0, ASN1_VALUE, ASN1_VALUE_st,
                       ASN1_OCTET_STRING_new, ASN1_OCTET_STRING_free,
                       ASN1_OCTET_STRING_cmp, ASN1_OCTET_STRING_set,
                       ASN1_STRING_to_UTF8};
pub use self::x509_h::{X509_algor_st, X509_EXTENSION, private_key_st,
                       X509_PKEY, X509_info_st, X509_INFO,
                       sk_X509_INFO_freefunc, sk_X509_INFO_num,
                       sk_X509_INFO_value, sk_X509_INFO_pop_free,
                       X509_extension_st, stack_st_X509_INFO,
                       X509_verify_cert_error_string, X509_get_subject_name,
                       X509_INFO_free, X509_NAME_print_ex,
                       X509_NAME_get_text_by_NID, X509_get_ext_by_NID,
                       X509_get_ext};
pub use self::x509v3_h::{GENERAL_NAMES, otherName_st, OTHERNAME,
                         EDIPartyName_st, EDIPARTYNAME, GENERAL_NAME_st,
                         C2RustUnnamed_1, GENERAL_NAME,
                         sk_GENERAL_NAME_freefunc, sk_GENERAL_NAME_num,
                         sk_GENERAL_NAME_value, sk_GENERAL_NAME_pop_free,
                         stack_st_GENERAL_NAME, GENERAL_NAME_free,
                         X509V3_EXT_d2i};
pub use self::crypto_h::{crypto_ex_data_st, CRYPTO_EX_new, CRYPTO_EX_free,
                         CRYPTO_EX_dup, stack_st_void,
                         CRYPTO_get_ex_new_index, CRYPTO_free,
                         OPENSSL_init_crypto};
pub use self::stack_h::{OPENSSL_STACK, OPENSSL_sk_freefunc, stack_st,
                        OPENSSL_sk_num, OPENSSL_sk_value,
                        OPENSSL_sk_pop_free};
pub use self::bio_h::{BIO_METHOD, bio_method_st, BIO_new, BIO_free, BIO_ctrl,
                      BIO_s_mem};
pub use self::evp_h::{evp_cipher_info_st, EVP_CIPHER_INFO};
pub use self::pem_h::{pem_password_cb, PEM_X509_INFO_read};
pub use self::ssl_h::{SSL_METHOD, SSL_verify_cb, ssl_method_st,
                      OPENSSL_init_ssl, SSL_get_ex_data_X509_STORE_CTX_idx,
                      SSL_get_ex_data, SSL_set_ex_data,
                      SSL_CTX_set_default_verify_paths, SSL_set_connect_state,
                      TLS_client_method, SSL_get_error, SSL_ctrl, SSL_write,
                      SSL_read, SSL_free, SSL_new, SSL_CTX_set_verify,
                      SSL_set_fd, SSL_CTX_get_cert_store, SSL_CTX_free,
                      SSL_CTX_new, SSL_CTX_get_options, SSL_CTX_set_options};
pub use self::dirent_h::dirent;
pub use self::include_dirent_h::{DIR, __dirstream, opendir, closedir,
                                 readdir};
use self::assert_h::__assert_fail;
use self::string_h::{memchr, strncmp, strdup, strcspn, strlen};
use self::stdlib_h::{malloc, free, secure_getenv};
use self::stdio_h::{fclose, fopen, snprintf};
use self::errno_h::__errno_location;
use self::inet_h::inet_pton;
use self::k5_trace_h::krb5int_trace;
use self::err_h::{ERR_error_string_n, ERR_get_error};
use self::x509_vfy_h::{X509_STORE_set_flags, X509_STORE_add_cert,
                       X509_STORE_CTX_get_ex_data, X509_STORE_CTX_get_error,
                       X509_STORE_CTX_get_error_depth,
                       X509_STORE_CTX_get_current_cert};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/tls/k5tls/openssl.c - OpenSSL TLS module implementation */
/*
 * Copyright 2013,2014 Red Hat, Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *    1. Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *    2. Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
 * OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "41:8"]
pub struct k5_tls_handle_st {
    pub ssl: *mut SSL,
    pub servername: *mut libc::c_char,
}
#[c2rust::src_loc = "46:12"]
static mut ex_context_id: libc::c_int = -(1 as libc::c_int);
#[c2rust::src_loc = "47:12"]
static mut ex_handle_id: libc::c_int = -(1 as libc::c_int);
#[c2rust::src_loc = "49:1"]
unsafe extern "C" fn init_openssl__aux() {
    init_openssl__once.did_run = 1 as libc::c_int;
    init_openssl__once.error = init_openssl();
}
#[c2rust::src_loc = "49:1"]
static mut init_openssl__once: k5_init_t =
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
                              Some(init_openssl__aux as
                                       unsafe extern "C" fn() -> ()),};
            init
        }
    };
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn init_openssl() -> libc::c_int {
    OPENSSL_init_ssl(0 as libc::c_int as uint64_t,
                     0 as *const OPENSSL_INIT_SETTINGS);
    OPENSSL_init_ssl((0x200000 as libc::c_long | 0x2 as libc::c_long) as
                         uint64_t, 0 as *const OPENSSL_INIT_SETTINGS);
    OPENSSL_init_crypto((0x4 as libc::c_long | 0x8 as libc::c_long) as
                            uint64_t, 0 as *const OPENSSL_INIT_SETTINGS);
    ex_context_id =
        CRYPTO_get_ex_new_index(0 as libc::c_int,
                                0 as libc::c_int as libc::c_long,
                                0 as *mut libc::c_void, None, None, None);
    ex_handle_id =
        CRYPTO_get_ex_new_index(0 as libc::c_int,
                                0 as libc::c_int as libc::c_long,
                                0 as *mut libc::c_void, None, None, None);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "62:1"]
unsafe extern "C" fn flush_errors(mut context: krb5_context) {
    let mut err: libc::c_ulong = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    loop  {
        err = ERR_get_error();
        if !(err != 0 as libc::c_int as libc::c_ulong) { break ; }
        ERR_error_string_n(err, buf.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 128]>() as
                               libc::c_ulong);
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"TLS error: {str}\x00" as *const u8 as
                              *const libc::c_char, buf.as_mut_ptr());
        }
    };
}
/* Return the passed-in character, lower-cased if it's an ASCII character. */
#[inline]
#[c2rust::src_loc = "75:1"]
unsafe extern "C" fn ascii_tolower(mut p: libc::c_char) -> libc::c_char {
    if p as libc::c_int >= 'A' as i32 && p as libc::c_int <= 'Z' as i32 {
        return (p as libc::c_int + ('a' as i32 - 'A' as i32)) as libc::c_char
    }
    return p;
}
/*
 * Check a single label.  If allow_wildcard is true, and the presented name
 * includes a wildcard, return true and note that we matched a wildcard.
 * Otherwise, for both the presented and expected values, do a case-insensitive
 * comparison of ASCII characters, and a case-sensitive comparison of
 * everything else.
 */
#[c2rust::src_loc = "90:1"]
unsafe extern "C" fn label_match(mut presented: *const libc::c_char,
                                 mut plen: size_t,
                                 mut expected: *const libc::c_char,
                                 mut elen: size_t,
                                 mut allow_wildcard: krb5_boolean,
                                 mut wildcard: *mut krb5_boolean)
 -> krb5_boolean {
    let mut i: libc::c_uint = 0;
    if allow_wildcard != 0 && plen == 1 as libc::c_int as libc::c_ulong &&
           *presented.offset(0 as libc::c_int as isize) as libc::c_int ==
               '*' as i32 {
        *wildcard = 1 as libc::c_int as krb5_boolean;
        return 1 as libc::c_int as krb5_boolean
    }
    if plen != elen { return 0 as libc::c_int as krb5_boolean }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < elen {
        if ascii_tolower(*presented.offset(i as isize)) as libc::c_int !=
               ascii_tolower(*expected.offset(i as isize)) as libc::c_int {
            return 0 as libc::c_int as krb5_boolean
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int as krb5_boolean;
}
/* Break up the two names and check them, label by label. */
#[c2rust::src_loc = "112:1"]
unsafe extern "C" fn domain_match(mut presented: *const libc::c_char,
                                  mut plen: size_t,
                                  mut expected: *const libc::c_char)
 -> krb5_boolean {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut n_label: libc::c_int = 0;
    let mut used_wildcard: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    n_label = 0 as libc::c_int;
    p = presented;
    r = expected;
    while p < presented.offset(plen as isize) &&
              *r as libc::c_int != '\u{0}' as i32 {
        q =
            memchr(p as *const libc::c_void, '.' as i32,
                   plen.wrapping_sub(p.wrapping_offset_from(presented) as
                                         libc::c_long as libc::c_ulong)) as
                *const libc::c_char;
        if q.is_null() { q = presented.offset(plen as isize) }
        s =
            r.offset(strcspn(r, b".\x00" as *const u8 as *const libc::c_char)
                         as isize);
        if label_match(p, q.wrapping_offset_from(p) as libc::c_long as size_t,
                       r, s.wrapping_offset_from(r) as libc::c_long as size_t,
                       (n_label == 0 as libc::c_int) as libc::c_int as
                           krb5_boolean, &mut used_wildcard) == 0 {
            return 0 as libc::c_int as krb5_boolean
        }
        p =
            if q < presented.offset(plen as isize) {
                q.offset(1 as libc::c_int as isize)
            } else { q };
        r =
            if *s as libc::c_int != 0 {
                s.offset(1 as libc::c_int as isize)
            } else { s };
        n_label += 1
    }
    if used_wildcard != 0 && n_label <= 2 as libc::c_int {
        return 0 as libc::c_int as krb5_boolean
    }
    if p == presented.offset(plen as isize) &&
           *r as libc::c_int == '\u{0}' as i32 {
        return 1 as libc::c_int as krb5_boolean
    }
    return 0 as libc::c_int as krb5_boolean;
}
/* Fetch the list of subjectAltNames from a certificate. */
#[c2rust::src_loc = "141:1"]
unsafe extern "C" fn get_cert_sans(mut x: *mut X509) -> *mut GENERAL_NAMES {
    let mut ext: libc::c_int = 0;
    let mut san_ext: *mut X509_EXTENSION = 0 as *mut X509_EXTENSION;
    ext = X509_get_ext_by_NID(x, 85 as libc::c_int, -(1 as libc::c_int));
    if ext < 0 as libc::c_int { return 0 as *mut GENERAL_NAMES }
    san_ext = X509_get_ext(x, ext);
    if san_ext.is_null() { return 0 as *mut GENERAL_NAMES }
    return X509V3_EXT_d2i(san_ext) as *mut GENERAL_NAMES;
}
/* Fetch a CN value from the subjct name field, returning its length, or -1 if
 * there is no subject name or it contains no CN value. */
#[c2rust::src_loc = "158:1"]
unsafe extern "C" fn get_cert_cn(mut x: *mut X509, mut buf: *mut libc::c_char,
                                 mut bufsize: size_t) -> libc::c_int {
    let mut name: *mut X509_NAME = 0 as *mut X509_NAME;
    name = X509_get_subject_name(x);
    if name.is_null() { return -(1 as libc::c_int) }
    return X509_NAME_get_text_by_NID(name, 13 as libc::c_int, buf,
                                     bufsize as libc::c_int);
}
/* Return true if text matches any of the addresses we can recover from x. */
#[c2rust::src_loc = "170:1"]
unsafe extern "C" fn check_cert_address(mut x: *mut X509,
                                        mut text: *const libc::c_char)
 -> krb5_boolean {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut sans: *mut GENERAL_NAMES = 0 as *mut GENERAL_NAMES;
    let mut san: *mut GENERAL_NAME = 0 as *mut GENERAL_NAME;
    let mut ip: *mut ASN1_OCTET_STRING = 0 as *mut ASN1_OCTET_STRING;
    let mut found_ip_san: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut matched: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut n_sans: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut name_length: libc::c_int = 0;
    let mut sin: in_addr = in_addr{s_addr: 0,};
    let mut sin6: in6_addr =
        in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    /* Parse the IP address into an octet string. */
    ip = ASN1_OCTET_STRING_new();
    if ip.is_null() { return 0 as libc::c_int as krb5_boolean }
    if inet_pton(2 as libc::c_int, text,
                 &mut sin as *mut in_addr as *mut libc::c_void) != 0 {
        ASN1_OCTET_STRING_set(ip,
                              &mut sin as *mut in_addr as *mut libc::c_uchar,
                              ::std::mem::size_of::<in_addr>() as
                                  libc::c_ulong as libc::c_int);
    } else if inet_pton(10 as libc::c_int, text,
                        &mut sin6 as *mut in6_addr as *mut libc::c_void) != 0
     {
        ASN1_OCTET_STRING_set(ip,
                              &mut sin6 as *mut in6_addr as
                                  *mut libc::c_uchar,
                              ::std::mem::size_of::<in6_addr>() as
                                  libc::c_ulong as libc::c_int);
    } else {
        ASN1_OCTET_STRING_free(ip);
        return 0 as libc::c_int as krb5_boolean
    }
    /* Check for matches in ipaddress subjectAltName values. */
    sans = get_cert_sans(x);
    if !sans.is_null() {
        n_sans = sk_GENERAL_NAME_num(sans);
        i = 0 as libc::c_int;
        while i < n_sans {
            san = sk_GENERAL_NAME_value(sans, i);
            if !((*san).type_0 != 7 as libc::c_int) {
                found_ip_san = 1 as libc::c_int as krb5_boolean;
                matched =
                    (ASN1_OCTET_STRING_cmp(ip, (*san).d.iPAddress) ==
                         0 as libc::c_int) as libc::c_int as krb5_boolean;
                if matched != 0 { break ; }
            }
            i += 1
        }
        sk_GENERAL_NAME_pop_free(sans,
                                 Some(GENERAL_NAME_free as
                                          unsafe extern "C" fn(_:
                                                                   *mut GENERAL_NAME)
                                              -> ()));
    }
    ASN1_OCTET_STRING_free(ip);
    if found_ip_san != 0 { return matched }
    /* Check for a match against the CN value in the peer's subject name. */
    name_length =
        get_cert_cn(x, buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as
                        libc::c_ulong);
    if name_length >= 0 as libc::c_int {
        /* Do a string compare to check if it's an acceptable value. */
        return (strlen(text) == name_length as size_t &&
                    strncmp(text, buf.as_mut_ptr(),
                            name_length as libc::c_ulong) == 0 as libc::c_int)
                   as libc::c_int as krb5_boolean
    }
    /* We didn't find a match. */
    return 0 as libc::c_int as krb5_boolean;
}
/* Return true if expected matches any of the names we can recover from x. */
#[c2rust::src_loc = "229:1"]
unsafe extern "C" fn check_cert_servername(mut x: *mut X509,
                                           mut expected: *const libc::c_char)
 -> krb5_boolean {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut sans: *mut GENERAL_NAMES = 0 as *mut GENERAL_NAMES;
    let mut san: *mut GENERAL_NAME = 0 as *mut GENERAL_NAME;
    let mut dnsname: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut found_dns_san: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut matched: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut name_length: libc::c_int = 0;
    let mut n_sans: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /* Check for matches in dnsname subjectAltName values. */
    sans = get_cert_sans(x);
    if !sans.is_null() {
        n_sans = sk_GENERAL_NAME_num(sans);
        i = 0 as libc::c_int;
        while i < n_sans {
            san = sk_GENERAL_NAME_value(sans, i);
            if !((*san).type_0 != 2 as libc::c_int) {
                found_dns_san = 1 as libc::c_int as krb5_boolean;
                dnsname = 0 as *mut libc::c_uchar;
                name_length =
                    ASN1_STRING_to_UTF8(&mut dnsname, (*san).d.dNSName);
                if !dnsname.is_null() {
                    matched =
                        domain_match(dnsname as *mut libc::c_char,
                                     name_length as size_t, expected);
                    CRYPTO_free(dnsname as *mut libc::c_void,
                                b"openssl.c\x00" as *const u8 as
                                    *const libc::c_char, 253 as libc::c_int);
                    if matched != 0 { break ; }
                }
            }
            i += 1
        }
        sk_GENERAL_NAME_pop_free(sans,
                                 Some(GENERAL_NAME_free as
                                          unsafe extern "C" fn(_:
                                                                   *mut GENERAL_NAME)
                                              -> ()));
    }
    if matched != 0 { return 1 as libc::c_int as krb5_boolean }
    if found_dns_san != 0 { return matched }
    /* Check for a match against the CN value in the peer's subject name. */
    name_length =
        get_cert_cn(x, buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as
                        libc::c_ulong);
    if name_length >= 0 as libc::c_int {
        return domain_match(buf.as_mut_ptr(), name_length as size_t, expected)
    }
    /* We didn't find a match. */
    return 0 as libc::c_int as krb5_boolean;
}
#[c2rust::src_loc = "274:1"]
unsafe extern "C" fn check_cert_name_or_ip(mut x: *mut X509,
                                           mut expected_name:
                                               *const libc::c_char)
 -> krb5_boolean {
    let mut in_0: in_addr = in_addr{s_addr: 0,};
    let mut in6: in6_addr =
        in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    if inet_pton(2 as libc::c_int, expected_name,
                 &mut in_0 as *mut in_addr as *mut libc::c_void) !=
           0 as libc::c_int ||
           inet_pton(10 as libc::c_int, expected_name,
                     &mut in6 as *mut in6_addr as *mut libc::c_void) !=
               0 as libc::c_int {
        return check_cert_address(x, expected_name)
    } else { return check_cert_servername(x, expected_name) };
}
#[c2rust::src_loc = "288:1"]
unsafe extern "C" fn verify_callback(mut preverify_ok: libc::c_int,
                                     mut store_ctx: *mut X509_STORE_CTX)
 -> libc::c_int {
    let mut x: *mut X509 = 0 as *mut X509;
    let mut ssl: *mut SSL = 0 as *mut SSL;
    let mut bio: *mut BIO = 0 as *mut BIO;
    let mut context: krb5_context = 0 as *mut _krb5_context;
    let mut err: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    let mut handle: k5_tls_handle = 0 as *mut k5_tls_handle_st;
    let mut cert: *const libc::c_char = 0 as *const libc::c_char;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut expected_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut count: size_t = 0;
    ssl =
        X509_STORE_CTX_get_ex_data(store_ctx,
                                   SSL_get_ex_data_X509_STORE_CTX_idx()) as
            *mut SSL;
    context = SSL_get_ex_data(ssl, ex_context_id) as krb5_context;
    handle = SSL_get_ex_data(ssl, ex_handle_id) as k5_tls_handle;
    if !context.is_null() && !handle.is_null() {
    } else {
        __assert_fail(b"context != NULL && handle != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"openssl.c\x00" as *const u8 as *const libc::c_char,
                      304 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 43],
                                                &[libc::c_char; 43]>(b"int verify_callback(int, X509_STORE_CTX *)\x00")).as_ptr());
    }
    /* We do have the peer's cert, right? */
    x = X509_STORE_CTX_get_current_cert(store_ctx);
    if x.is_null() {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"TLS server certificate not received\x00" as
                              *const u8 as *const libc::c_char);
        }
        return 0 as libc::c_int
    }
    /* Figure out where we are. */
    depth = X509_STORE_CTX_get_error_depth(store_ctx);
    if depth < 0 as libc::c_int { return 0 as libc::c_int }
    /* If there's an error at this level that we're not ignoring, fail. */
    err = X509_STORE_CTX_get_error(store_ctx);
    if err != 0 as libc::c_int {
        bio = BIO_new(BIO_s_mem());
        if !bio.is_null() {
            X509_NAME_print_ex(bio, X509_get_subject_name(x),
                               0 as libc::c_int,
                               0 as libc::c_int as libc::c_ulong);
            count =
                BIO_ctrl(bio, 3 as libc::c_int,
                         0 as libc::c_int as libc::c_long,
                         &mut cert as *mut *const libc::c_char as
                             *mut libc::c_char as *mut libc::c_void) as
                    size_t;
            errstr = X509_verify_cert_error_string(err as libc::c_long);
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"TLS certificate error at {int} ({lenstr}): {int} ({str})\x00"
                                  as *const u8 as *const libc::c_char, depth,
                              count, cert, err, errstr);
            }
            BIO_free(bio);
        }
        return 0 as libc::c_int
    }
    /* If we're not looking at the peer, we're done and everything's ok. */
    if depth != 0 as libc::c_int { return 1 as libc::c_int }
    /* Check if the name we expect to find is in the certificate. */
    expected_name = (*handle).servername;
    if check_cert_name_or_ip(x, expected_name) != 0 {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"TLS certificate name matched \"{str}\"\x00" as
                              *const u8 as *const libc::c_char,
                          expected_name);
        }
        return 1 as libc::c_int
    } else {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"TLS certificate name mismatch: server certificate is not for \"{str}\"\x00"
                              as *const u8 as *const libc::c_char,
                          expected_name);
        }
    }
    /* The name didn't match. */
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "343:1"]
unsafe extern "C" fn load_anchor_file(mut store: *mut X509_STORE,
                                      mut path: *const libc::c_char)
 -> krb5_error_code {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut sk: *mut stack_st_X509_INFO = 0 as *mut stack_st_X509_INFO;
    let mut xi: *mut X509_INFO = 0 as *mut X509_INFO;
    let mut i: libc::c_int = 0;
    fp = fopen(path, b"r\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() { return *__errno_location() }
    sk =
        PEM_X509_INFO_read(fp, 0 as *mut stack_st_X509_INFO, None,
                           0 as *mut libc::c_void);
    fclose(fp);
    if sk.is_null() { return 2 as libc::c_int }
    i = 0 as libc::c_int;
    while i < sk_X509_INFO_num(sk) {
        xi = sk_X509_INFO_value(sk, i);
        if !(*xi).x509.is_null() { X509_STORE_add_cert(store, (*xi).x509); }
        i += 1
    }
    sk_X509_INFO_pop_free(sk,
                          Some(X509_INFO_free as
                                   unsafe extern "C" fn(_: *mut X509_INFO)
                                       -> ()));
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "367:1"]
unsafe extern "C" fn load_anchor_dir(mut store: *mut X509_STORE,
                                     mut path: *const libc::c_char)
 -> krb5_error_code {
    let mut d: *mut DIR = 0 as *mut DIR;
    let mut dentry: *mut dirent = 0 as *mut dirent;
    let mut filename: [libc::c_char; 1024] = [0; 1024];
    let mut found_any: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    d = opendir(path);
    if d.is_null() { return 2 as libc::c_int }
    loop  {
        dentry = readdir(d);
        if dentry.is_null() { break ; }
        if (*dentry).d_name[0 as libc::c_int as usize] as libc::c_int !=
               '.' as i32 {
            snprintf(filename.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong,
                     b"%s/%s\x00" as *const u8 as *const libc::c_char, path,
                     (*dentry).d_name.as_mut_ptr());
            if load_anchor_file(store, filename.as_mut_ptr()) ==
                   0 as libc::c_int {
                found_any = 1 as libc::c_int as krb5_boolean
            }
        }
    }
    closedir(d);
    return if found_any != 0 { 0 as libc::c_int } else { 2 as libc::c_int };
}
#[c2rust::src_loc = "390:1"]
unsafe extern "C" fn load_anchor(mut ctx: *mut SSL_CTX,
                                 mut location: *const libc::c_char)
 -> krb5_error_code {
    let mut store: *mut X509_STORE = 0 as *mut X509_STORE;
    let mut envloc: *const libc::c_char = 0 as *const libc::c_char;
    store = SSL_CTX_get_cert_store(ctx);
    if strncmp(location, b"FILE:\x00" as *const u8 as *const libc::c_char,
               5 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
        return load_anchor_file(store,
                                location.offset(5 as libc::c_int as isize))
    } else {
        if strncmp(location, b"DIR:\x00" as *const u8 as *const libc::c_char,
                   4 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
            return load_anchor_dir(store,
                                   location.offset(4 as libc::c_int as isize))
        } else {
            if strncmp(location,
                       b"ENV:\x00" as *const u8 as *const libc::c_char,
                       4 as libc::c_int as libc::c_ulong) == 0 as libc::c_int
               {
                envloc =
                    secure_getenv(location.offset(4 as libc::c_int as isize));
                if envloc.is_null() { return 2 as libc::c_int }
                return load_anchor(ctx, envloc)
            }
        }
    }
    return 22 as libc::c_int;
}
#[c2rust::src_loc = "410:1"]
unsafe extern "C" fn load_anchors(mut context: krb5_context,
                                  mut anchors: *mut *mut libc::c_char,
                                  mut sctx: *mut SSL_CTX) -> krb5_error_code {
    let mut i: libc::c_uint = 0;
    let mut ret: krb5_error_code = 0;
    if !anchors.is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while !(*anchors.offset(i as isize)).is_null() {
            ret = load_anchor(sctx, *anchors.offset(i as isize));
            if ret != 0 { return ret }
            i = i.wrapping_add(1)
        }
    } else if SSL_CTX_set_default_verify_paths(sctx) != 1 as libc::c_int {
        return 2 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "431:1"]
unsafe extern "C" fn setup(mut context: krb5_context, mut fd: libc::c_int,
                           mut servername: *const libc::c_char,
                           mut anchors: *mut *mut libc::c_char,
                           mut handle_out: *mut k5_tls_handle)
 -> krb5_error_code {
    let mut e: libc::c_int = 0;
    let mut options: libc::c_long = 0;
    let mut ctx: *mut SSL_CTX = 0 as *mut SSL_CTX;
    let mut ssl: *mut SSL = 0 as *mut SSL;
    let mut handle: k5_tls_handle = 0 as k5_tls_handle;
    *handle_out = 0 as k5_tls_handle;
    ({
         let mut k5int_i: *mut k5_init_t = &mut init_openssl__once;
         let mut k5int_err: libc::c_int =
             k5_once(&mut (*k5int_i).once, (*k5int_i).fn_0);
         if k5int_err != 0 {
         } else {
             if (*k5int_i).did_run != 0 as libc::c_int {
             } else {
                 __assert_fail(b"k5int_i->did_run != 0\x00" as *const u8 as
                                   *const libc::c_char,
                               b"openssl.c\x00" as *const u8 as
                                   *const libc::c_char,
                               443 as libc::c_int as libc::c_uint,
                               (*::std::mem::transmute::<&[u8; 81],
                                                         &[libc::c_char; 81]>(b"krb5_error_code setup(krb5_context, int, const char *, char **, k5_tls_handle *)\x00")).as_ptr());
             }
         };
         compile_error!("Conditional expression is not supposed to be used")
     });
    if ex_context_id == -(1 as libc::c_int) ||
           ex_handle_id == -(1 as libc::c_int) {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    /* Use the library defaults. */
    /* Do general SSL library setup. */
    ctx = SSL_CTX_new(TLS_client_method());
    if !ctx.is_null() {
        options = SSL_CTX_get_options(ctx) as libc::c_long;
        SSL_CTX_set_options(ctx,
                            (options | 0 as libc::c_int as libc::c_long) as
                                libc::c_ulong);
        SSL_CTX_set_verify(ctx, 0x1 as libc::c_int,
                           Some(verify_callback as
                                    unsafe extern "C" fn(_: libc::c_int,
                                                         _:
                                                             *mut X509_STORE_CTX)
                                        -> libc::c_int));
        X509_STORE_set_flags(SSL_CTX_get_cert_store(ctx),
                             0 as libc::c_int as libc::c_ulong);
        e = load_anchors(context, anchors, ctx);
        if !(e != 0 as libc::c_int) {
            ssl = SSL_new(ctx);
            if !ssl.is_null() {
                if !(SSL_set_fd(ssl, fd) == 0) {
                    if !(SSL_ctrl(ssl, 55 as libc::c_int,
                                  0 as libc::c_int as libc::c_long,
                                  servername as *mut libc::c_void) == 0) {
                        SSL_set_connect_state(ssl);
                        /* Create a handle and allow verify_callback to access it. */
                        handle =
                            malloc(::std::mem::size_of::<k5_tls_handle_st>()
                                       as libc::c_ulong) as k5_tls_handle;
                        if !(handle.is_null() ||
                                 SSL_set_ex_data(ssl, ex_handle_id,
                                                 handle as *mut libc::c_void)
                                     == 0) {
                            (*handle).ssl = ssl;
                            (*handle).servername = strdup(servername);
                            if !(*handle).servername.is_null() {
                                *handle_out = handle;
                                SSL_CTX_free(ctx);
                                return 0 as libc::c_int
                            }
                        }
                    }
                }
            }
        }
    }
    flush_errors(context);
    free(handle as *mut libc::c_void);
    SSL_free(ssl);
    SSL_CTX_free(ctx);
    return -(1765328134 as libc::c_long) as krb5_error_code;
}
#[c2rust::src_loc = "493:1"]
unsafe extern "C" fn write_tls(mut context: krb5_context,
                               mut handle: k5_tls_handle,
                               mut data: *const libc::c_void, mut len: size_t)
 -> k5_tls_status {
    let mut nwritten: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    /* Try to transmit our request; allow verify_callback to access context. */
    if SSL_set_ex_data((*handle).ssl, ex_context_id,
                       context as *mut libc::c_void) == 0 {
        return ERROR_TLS
    }
    nwritten = SSL_write((*handle).ssl, data, len as libc::c_int);
    SSL_set_ex_data((*handle).ssl, ex_context_id, 0 as *mut libc::c_void);
    if nwritten > 0 as libc::c_int { return DONE }
    e = SSL_get_error((*handle).ssl, nwritten);
    if e == 2 as libc::c_int {
        return WANT_READ
    } else { if e == 3 as libc::c_int { return WANT_WRITE } }
    flush_errors(context);
    return ERROR_TLS;
}
#[c2rust::src_loc = "516:1"]
unsafe extern "C" fn read_tls(mut context: krb5_context,
                              mut handle: k5_tls_handle,
                              mut data: *mut libc::c_void,
                              mut data_size: size_t, mut len_out: *mut size_t)
 -> k5_tls_status {
    let mut nread: ssize_t = 0;
    let mut e: libc::c_int = 0;
    *len_out = 0 as libc::c_int as size_t;
    /* Try to read response data; allow verify_callback to access context. */
    if SSL_set_ex_data((*handle).ssl, ex_context_id,
                       context as *mut libc::c_void) == 0 {
        return ERROR_TLS
    }
    nread =
        SSL_read((*handle).ssl, data, data_size as libc::c_int) as ssize_t;
    SSL_set_ex_data((*handle).ssl, ex_context_id, 0 as *mut libc::c_void);
    if nread > 0 as libc::c_int as libc::c_long {
        *len_out = nread as size_t;
        return DATA_READ
    }
    e = SSL_get_error((*handle).ssl, nread as libc::c_int);
    if e == 2 as libc::c_int {
        return WANT_READ
    } else { if e == 3 as libc::c_int { return WANT_WRITE } }
    if e == 6 as libc::c_int ||
           e == 5 as libc::c_int && nread == 0 as libc::c_int as libc::c_long
       {
        return DONE
    }
    flush_errors(context);
    return ERROR_TLS;
}
#[c2rust::src_loc = "548:1"]
unsafe extern "C" fn free_handle(mut context: krb5_context,
                                 mut handle: k5_tls_handle) {
    SSL_free((*handle).ssl);
    free((*handle).servername as *mut libc::c_void);
    free(handle as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "560:1"]
pub unsafe extern "C" fn tls_k5tls_initvt(mut context: krb5_context,
                                          mut maj_ver: libc::c_int,
                                          mut min_ver: libc::c_int,
                                          mut vtable: krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: k5_tls_vtable = 0 as *mut k5_tls_vtable_st;
    vt = vtable as k5_tls_vtable;
    (*vt).setup =
        Some(setup as
                 unsafe extern "C" fn(_: krb5_context, _: libc::c_int,
                                      _: *const libc::c_char,
                                      _: *mut *mut libc::c_char,
                                      _: *mut k5_tls_handle)
                     -> krb5_error_code);
    (*vt).write =
        Some(write_tls as
                 unsafe extern "C" fn(_: krb5_context, _: k5_tls_handle,
                                      _: *const libc::c_void, _: size_t)
                     -> k5_tls_status);
    (*vt).read =
        Some(read_tls as
                 unsafe extern "C" fn(_: krb5_context, _: k5_tls_handle,
                                      _: *mut libc::c_void, _: size_t,
                                      _: *mut size_t) -> k5_tls_status);
    (*vt).free_handle =
        Some(free_handle as
                 unsafe extern "C" fn(_: krb5_context, _: k5_tls_handle)
                     -> ());
    return 0 as libc::c_int;
}
/* TLS_IMPL_OPENSSL */
