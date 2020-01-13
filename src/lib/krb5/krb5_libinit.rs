use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:3"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:3"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:3"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:3"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:3"]
pub mod k5_thread_h {
    /* Values:
   2 - function has not been run
   3 - function has been run
   4 - function is being run -- deadlock detected */
    #[c2rust::src_loc = "188:1"]
    pub type k5_os_nothread_once_t = libc::c_uchar;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "256:9"]
    pub struct k5_once_t {
        pub o: pthread_once_t,
        pub n: k5_os_nothread_once_t,
    }
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    #[inline]
    #[c2rust::src_loc = "360:1"]
    pub unsafe extern "C" fn k5_mutex_finish_init(mut m: *mut k5_mutex_t)
     -> libc::c_int {
        return 0 as libc::c_int;
    }
    use super::pthreadtypes_h::{pthread_once_t, pthread_mutex_t};
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:3"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:3"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    use super::stdint_intn_h::int32_t;
    use super::com_err_h::error_table;
    extern "C" {
        /* Don't use this!  We're going to phase it out.  It's just here to keep
   applications from breaking right away.  */
        /* * @} */
        /* end of KRB5_H group */
        /* KRB5_GENERAL__ */
        /*
 * et-h-krb5_err.h:
 * This file is automatically generated; please do not edit it.
 */
        #[no_mangle]
        #[c2rust::src_loc = "8833:33"]
        pub static et_krb5_error_table: error_table;
        /*@modifies internalState@*/
        /*
 * et-h-k5e1_err.h:
 * This file is automatically generated; please do not edit it.
 */
        #[no_mangle]
        #[c2rust::src_loc = "8867:33"]
        pub static et_k5e1_error_table: error_table;
        /*@modifies internalState@*/
        /*
 * et-h-kdb5_err.h:
 * This file is automatically generated; please do not edit it.
 */
        #[no_mangle]
        #[c2rust::src_loc = "8935:33"]
        pub static et_kdb5_error_table: error_table;
        /*@modifies internalState@*/
        /*
 * et-h-kv5m_err.h:
 * This file is automatically generated; please do not edit it.
 */
        #[no_mangle]
        #[c2rust::src_loc = "9018:33"]
        pub static et_kv5m_error_table: error_table;
        /*@modifies internalState@*/
        /*
 * et-h-krb524_err.h:
 * This file is automatically generated; please do not edit it.
 */
        #[no_mangle]
        #[c2rust::src_loc = "9049:33"]
        pub static et_k524_error_table: error_table;
        /*@modifies internalState@*/
        /*
 * et-h-asn1_err.h:
 * This file is automatically generated; please do not edit it.
 */
        #[no_mangle]
        #[c2rust::src_loc = "9085:33"]
        pub static et_asn1_error_table: error_table;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:3"]
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
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn error_message(_: errcode_t) -> *const libc::c_char;
        /*@modifies internalState@*/
        #[no_mangle]
        #[c2rust::src_loc = "57:1"]
        pub fn add_error_table(_: *const error_table) -> errcode_t;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/usr/include/libintl.h:3"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "86:1"]
        pub fn bindtextdomain(__domainname: *const libc::c_char,
                              __dirname: *const libc::c_char)
         -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:3"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:3"]
pub mod k5_err_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn k5_set_error_info_callout_fn(f:
                                                Option<unsafe extern "C" fn(_:
                                                                                libc::c_long)
                                                           ->
                                                               *const libc::c_char>);
    }
    /* K5_ERR_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/ccache/cc-int.h:11"]
pub mod cc_int_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "48:1"]
        pub fn krb5int_cc_initialize() -> libc::c_int;
    }
    /* __KRB5_CCACHE_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/keytab/kt-int.h:12"]
pub mod kt_int_h {
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/keytab/kt-int.h */
/*
 * Copyright 2004 by the Massachusetts Institute of Technology.
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
 * This file contains constant and function declarations used in the
 * file-based credential cache routines.
 */
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn krb5int_kt_initialize() -> libc::c_int;
    }
    /* __KRB5_KEYTAB_INT_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/os/os-proto.h:13"]
pub mod os_proto_h {
    use super::k5_thread_h::k5_mutex_t;
    use super::thread_shared_types_h::{__pthread_mutex_s, __pthread_list_t,
                                       __pthread_internal_list};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "189:19"]
        pub static mut krb5int_us_time_mutex: k5_mutex_t;
    }
    /* KRB5_LIBOS_INT_PROTO__ */
}
pub use self::types_h::__int32_t;
pub use self::stdint_intn_h::int32_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::{pthread_once_t, pthread_mutex_t};
pub use self::k5_thread_h::{k5_os_nothread_once_t, k5_once_t, k5_os_mutex,
                            k5_mutex_t, k5_mutex_finish_init, k5_once};
pub use self::k5_platform_h::k5_init_t;
pub use self::krb5_h::{krb5_int32, krb5_error_code, et_krb5_error_table,
                       et_k5e1_error_table, et_kdb5_error_table,
                       et_kv5m_error_table, et_k524_error_table,
                       et_asn1_error_table};
pub use self::com_err_h::{errcode_t, error_table, error_message,
                          add_error_table};
use self::libintl_h::bindtextdomain;
use self::assert_h::__assert_fail;
use self::k5_err_h::k5_set_error_info_callout_fn;
use self::cc_int_h::krb5int_cc_initialize;
use self::kt_int_h::krb5int_kt_initialize;
use self::os_proto_h::krb5int_us_time_mutex;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Initialize the Kerberos v5 library.
 */
#[c2rust::src_loc = "19:1"]
unsafe extern "C" fn krb5int_lib_init__aux() {
    krb5int_lib_init__once.did_run = 1 as libc::c_int;
    krb5int_lib_init__once.error = krb5int_lib_init();
}
#[c2rust::src_loc = "19:1"]
static mut krb5int_lib_init__once: k5_init_t =
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
                              Some(krb5int_lib_init__aux as
                                       unsafe extern "C" fn() -> ()),};
            init
        }
    };
/* Possibly load-time initialization -- mutexes, etc.  */
#[c2rust::src_loc = "23:1"]
unsafe extern "C" fn krb5int_lib_init() -> libc::c_int {
    let mut err: libc::c_int = 0;
    k5_set_error_info_callout_fn(Some(error_message as
                                          unsafe extern "C" fn(_: errcode_t)
                                              -> *const libc::c_char));
    add_error_table(&et_krb5_error_table);
    add_error_table(&et_k5e1_error_table);
    add_error_table(&et_kv5m_error_table);
    add_error_table(&et_kdb5_error_table);
    add_error_table(&et_asn1_error_table);
    add_error_table(&et_k524_error_table);
    bindtextdomain(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"/usr/local/share/locale\x00" as *const u8 as
                       *const libc::c_char);
    err = krb5int_kt_initialize();
    if err != 0 { return err }
    /* LEAN_CLIENT */
    err = krb5int_cc_initialize();
    if err != 0 { return err }
    err = k5_mutex_finish_init(&mut krb5int_us_time_mutex);
    if err != 0 { return err }
    return 0 as libc::c_int;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* Always-delayed initialization -- error table linkage, etc.  */
#[no_mangle]
#[c2rust::src_loc = "58:1"]
pub unsafe extern "C" fn krb5int_initialize_library() -> krb5_error_code {
    return ({
                let mut k5int_i: *mut k5_init_t = &mut krb5int_lib_init__once;
                let mut k5int_err: libc::c_int =
                    k5_once(&mut (*k5int_i).once, (*k5int_i).fn_0);
                if k5int_err != 0 {
                    k5int_err
                } else {
                    if (*k5int_i).did_run != 0 as libc::c_int {
                    } else {
                        __assert_fail(b"k5int_i->did_run != 0\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"krb5_libinit.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      60 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 49],
                                                                &[libc::c_char; 49]>(b"krb5_error_code krb5int_initialize_library(void)\x00")).as_ptr());
                    }
                    (*k5int_i).error
                }
            });
}
/* Still exists because it went into the export list on Windows.  But
   since the above function should be invoked at unload time, we don't
   actually want to do anything here.  */
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn krb5int_cleanup_library() { }
