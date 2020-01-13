use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:8"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:8"]
pub mod types_h {
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:8"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::__uint64_t;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:8"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:8"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:8"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:8"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:8"]
pub mod k5_plugin_h {
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
    }
    /* K5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:8"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/profile/prof_int.h:8"]
pub mod prof_int_h {
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
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn profile_node_iterator(iter_p: *mut *mut libc::c_void,
                                     ret_node: *mut *mut profile_node,
                                     ret_name: *mut *mut libc::c_char,
                                     ret_value: *mut *mut libc::c_char)
         -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "189:1"]
        pub fn profile_node_iterator_create(profile: profile_t,
                                            names: *const *const libc::c_char,
                                            flags: libc::c_int,
                                            ret_iter: *mut *mut libc::c_void)
         -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "193:1"]
        pub fn profile_node_iterator_free(iter_p: *mut *mut libc::c_void);
    }
    /* prof_set.c -- included from profile.h */
    /* Others included from profile.h */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/profile/profile.h:8"]
pub mod profile_h {
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
    use super::prof_int_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/string.h:8"]
pub mod string_h {
    extern "C" {
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
    }
}
#[c2rust::header_src = "/usr/include/strings.h:8"]
pub mod strings_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "116:12"]
        pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:8"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "176:17"]
        pub fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
                      _: libc::c_int) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/errno.h:8"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint64_t, __time_t};
pub use self::stdint_uintn_h::uint64_t;
pub use self::time_t_h::time_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t};
use self::k5_plugin_h::plugin_file_handle;
pub use self::com_err_h::errcode_t;
pub use self::prof_int_h::{_profile_t, prf_lib_handle_t, _prf_lib_handle_t,
                           prf_file_t, _prf_file_t, _prf_data_t,
                           C2RustUnnamed, prf_magic_t, profile_node,
                           profile_node_iterator,
                           profile_node_iterator_create,
                           profile_node_iterator_free};
pub use self::profile_h::{profile_vtable, profile_flush_fn,
                          profile_add_relation_fn, profile_rename_section_fn,
                          profile_update_relation_fn, profile_modified_fn,
                          profile_writable_fn, profile_free_string_fn,
                          profile_iterator_free_fn, profile_iterator_fn,
                          profile_iterator_create_fn, profile_copy_fn,
                          profile_cleanup_fn, profile_free_values_fn,
                          profile_get_values_fn, profile_t};
use self::string_h::{strcmp, strdup, strlen};
use self::strings_h::strcasecmp;
use self::stdlib_h::{strtol, malloc, realloc, free};
use self::errno_h::__errno_location;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * prof_get.c --- routines that expose the public interfaces for
 *      querying items from the profile.
 *
 */
/*
 * These functions --- init_list(), end_list(), and add_to_list() are
 * internal functions used to build up a null-terminated char ** list
 * of strings to be returned by functions like profile_get_values.
 *
 * The profile_string_list structure is used for internal booking
 * purposes to build up the list, which is returned in *ret_list by
 * the end_list() function.
 *
 * The publicly exported interface for freeing char** list is
 * profile_free_list().
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "30:8"]
pub struct profile_string_list {
    pub list: *mut *mut libc::c_char,
    pub num: libc::c_uint,
    pub max: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "477:8"]
pub struct profile_iterator {
    pub magic: prf_magic_t,
    pub profile: profile_t,
    pub idata: *mut libc::c_void,
}
/*
 * Initialize the string list abstraction.
 */
#[c2rust::src_loc = "39:1"]
unsafe extern "C" fn init_list(mut list: *mut profile_string_list)
 -> errcode_t {
    (*list).num = 0 as libc::c_int as libc::c_uint;
    (*list).max = 10 as libc::c_int as libc::c_uint;
    (*list).list =
        malloc(((*list).max as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *mut libc::c_char;
    if (*list).list.is_null() { return 12 as libc::c_int as errcode_t }
    let ref mut fresh0 = *(*list).list.offset(0 as libc::c_int as isize);
    *fresh0 = 0 as *mut libc::c_char;
    return 0 as libc::c_int as errcode_t;
}
/*
 * Free any memory left over in the string abstraction, returning the
 * built up list in *ret_list if it is non-null.
 */
#[c2rust::src_loc = "54:1"]
unsafe extern "C" fn end_list(mut list: *mut profile_string_list,
                              mut ret_list: *mut *mut *mut libc::c_char) {
    let mut cp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if list.is_null() { return }
    if !ret_list.is_null() {
        *ret_list = (*list).list;
        return
    } else {
        cp = (*list).list;
        while !(*cp).is_null() {
            free(*cp as *mut libc::c_void);
            cp = cp.offset(1)
        }
        free((*list).list as *mut libc::c_void);
    }
    (*list).max = 0 as libc::c_int as libc::c_uint;
    (*list).num = (*list).max;
    (*list).list = 0 as *mut *mut libc::c_char;
}
/*
 * Add a string to the list.
 */
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn add_to_list(mut list: *mut profile_string_list,
                                 mut str: *const libc::c_char) -> errcode_t {
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newlist: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut newmax: libc::c_uint = 0;
    if (*list).num.wrapping_add(1 as libc::c_int as libc::c_uint) >=
           (*list).max {
        newmax = (*list).max.wrapping_add(10 as libc::c_int as libc::c_uint);
        newlist =
            realloc((*list).list as *mut libc::c_void,
                    (newmax as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                         as libc::c_ulong)) as
                *mut *mut libc::c_char;
        if newlist.is_null() { return 12 as libc::c_int as errcode_t }
        (*list).max = newmax;
        (*list).list = newlist
    }
    newstr = strdup(str);
    if newstr.is_null() { return 12 as libc::c_int as errcode_t }
    let fresh1 = (*list).num;
    (*list).num = (*list).num.wrapping_add(1);
    let ref mut fresh2 = *(*list).list.offset(fresh1 as isize);
    *fresh2 = newstr;
    let ref mut fresh3 = *(*list).list.offset((*list).num as isize);
    *fresh3 = 0 as *mut libc::c_char;
    return 0 as libc::c_int as errcode_t;
}
/*
 * Return TRUE if the string is already a member of the list.
 */
#[c2rust::src_loc = "101:1"]
unsafe extern "C" fn is_list_member(mut list: *mut profile_string_list,
                                    mut str: *const libc::c_char)
 -> libc::c_int {
    let mut cpp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if (*list).list.is_null() { return 0 as libc::c_int }
    cpp = (*list).list;
    while !(*cpp).is_null() {
        if strcmp(*cpp, str) == 0 { return 1 as libc::c_int }
        cpp = cpp.offset(1)
    }
    return 0 as libc::c_int;
}
/*
 * This function frees a null-terminated list as returned by
 * profile_get_values.
 */
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn profile_free_list(mut list: *mut *mut libc::c_char) {
    let mut cp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if list.is_null() { return }
    cp = list;
    while !(*cp).is_null() {
        free(*cp as *mut libc::c_void);
        cp = cp.offset(1)
    }
    free(list as *mut libc::c_void);
}
/* Look up a relation in a vtable profile. */
#[c2rust::src_loc = "132:1"]
unsafe extern "C" fn get_values_vt(mut profile: profile_t,
                                   mut names: *const *const libc::c_char,
                                   mut ret_values:
                                       *mut *mut *mut libc::c_char)
 -> errcode_t {
    let mut retval: errcode_t = 0;
    let mut vtvalues: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut val: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut values: profile_string_list =
        profile_string_list{list: 0 as *mut *mut libc::c_char,
                            num: 0,
                            max: 0,};
    retval =
        (*(*profile).vt).get_values.expect("non-null function pointer")((*profile).cbdata,
                                                                        names,
                                                                        &mut vtvalues);
    if retval != 0 { return retval }
    /* Copy the result into memory we can free. */
    retval = init_list(&mut values);
    if retval == 0 as libc::c_int as libc::c_long {
        val = vtvalues;
        while !(*val).is_null() {
            add_to_list(&mut values, *val);
            val = val.offset(1)
        }
        end_list(&mut values, ret_values);
    }
    (*(*profile).vt).free_values.expect("non-null function pointer")((*profile).cbdata,
                                                                     vtvalues);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "155:1"]
pub unsafe extern "C" fn profile_get_values(mut profile: profile_t,
                                            mut names:
                                                *const *const libc::c_char,
                                            mut ret_values:
                                                *mut *mut *mut libc::c_char)
 -> libc::c_long {
    let mut current_block: u64;
    let mut retval: errcode_t = 0;
    let mut state: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut values: profile_string_list =
        profile_string_list{list: 0 as *mut *mut libc::c_char,
                            num: 0,
                            max: 0,};
    *ret_values = 0 as *mut *mut libc::c_char;
    if profile.is_null() { return -(1429577704 as libc::c_long) }
    if !(*profile).vt.is_null() {
        return get_values_vt(profile, names, ret_values)
    }
    retval =
        profile_node_iterator_create(profile, names, 0x4 as libc::c_int,
                                     &mut state);
    if retval != 0 { return retval }
    retval = init_list(&mut values);
    if retval != 0 { return retval }
    loop  {
        retval =
            profile_node_iterator(&mut state, 0 as *mut *mut profile_node,
                                  0 as *mut *mut libc::c_char, &mut value);
        if retval != 0 { current_block = 10200478277861437639; break ; }
        if !value.is_null() { add_to_list(&mut values, value); }
        if state.is_null() { current_block = 12599329904712511516; break ; }
    }
    match current_block {
        12599329904712511516 => {
            if values.num == 0 as libc::c_int as libc::c_uint {
                retval = -(1429577725 as libc::c_long)
            } else {
                end_list(&mut values, ret_values);
                return 0 as libc::c_int as libc::c_long
            }
        }
        _ => { }
    }
    end_list(&mut values, 0 as *mut *mut *mut libc::c_char);
    return retval;
}
/* Look up a relation in a vtable profile and return the first value in the
 * result. */
#[c2rust::src_loc = "200:1"]
unsafe extern "C" fn get_value_vt(mut profile: profile_t,
                                  mut names: *const *const libc::c_char,
                                  mut ret_value: *mut *mut libc::c_char)
 -> errcode_t {
    let mut retval: errcode_t = 0;
    let mut vtvalues: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    retval =
        (*(*profile).vt).get_values.expect("non-null function pointer")((*profile).cbdata,
                                                                        names,
                                                                        &mut vtvalues);
    if retval != 0 { return retval }
    *ret_value = strdup(*vtvalues);
    if (*ret_value).is_null() { retval = 12 as libc::c_int as errcode_t }
    (*(*profile).vt).free_values.expect("non-null function pointer")((*profile).cbdata,
                                                                     vtvalues);
    return retval;
}
/*
 * prof-int.h
 */
/*
 * This is the structure which stores the profile information for a
 * particular configuration file.
 *
 * Locking strategy:
 * - filespec, fslen are fixed after creation
 * - refcount and next should only be tweaked with the global lock held
 * - other fields can be tweaked after grabbing the in-struct lock
 */
/* time tree was last updated from file */
/* fractional part of timestamp, if any */
/* r/w, dirty */
/* incremented when data changes */
/* Some separation between fields controlled by different
	   mutexes.  Theoretically, both could be accessed at the same
	   time from different threads on different CPUs with separate
	   caches.  Don't let the threads clobber each other's
	   changes.  One mutex controlling the whole thing would be
	   better, but sufficient separation might suffice.

	   This is icky.  I just hope it's adequate.

	   For next major release, fix this.  */
/* prf_file_t references */
/* Was: "profile_filespec_t filespec".  Now: flexible char
	   array ... except, we need to work in C89, so an array
	   length must be specified.  */
/*
 * The profile flags
 */
/*
 * This structure defines the high-level, user visible profile_t
 * object, which is used as a handle by users who need to query some
 * configuration file(s)
 */
/* If non-null, use vtable operations instead of native ones. */
/*
 * Used by the profile iterator in prof_get.c
 */
/*
 * Check if a filespec is last in a list (NULL on UNIX, invalid FSSpec on MacOS
 */
/* profile_parse.c */
/* prof_tree.c */
/* prof_file.c */
/* prof_init.c -- included from profile.h */
/* prof_get.c */
/*
 * This function only gets the first value from the file; it is a
 * helper function for profile_get_string, profile_get_integer, etc.
 */
#[no_mangle]
#[c2rust::src_loc = "220:1"]
pub unsafe extern "C" fn profile_get_value(mut profile: profile_t,
                                           mut names:
                                               *mut *const libc::c_char,
                                           mut ret_value:
                                               *mut *mut libc::c_char)
 -> errcode_t {
    let mut retval: errcode_t = 0;
    let mut state: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    *ret_value = 0 as *mut libc::c_char;
    if profile.is_null() { return -(1429577704 as libc::c_long) }
    if !(*profile).vt.is_null() {
        return get_value_vt(profile, names, ret_value)
    }
    retval =
        profile_iterator_create(profile, names, 0x4 as libc::c_int,
                                &mut state);
    if retval != 0 { return retval }
    retval =
        profile_iterator(&mut state, 0 as *mut *mut libc::c_char, &mut value);
    if !(retval != 0) {
        if !value.is_null() {
            *ret_value = value
        } else { retval = -(1429577725 as libc::c_long) }
    }
    profile_iterator_free(&mut state);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "252:1"]
pub unsafe extern "C" fn profile_get_string(mut profile: profile_t,
                                            mut name: *const libc::c_char,
                                            mut subname: *const libc::c_char,
                                            mut subsubname:
                                                *const libc::c_char,
                                            mut def_val: *const libc::c_char,
                                            mut ret_string:
                                                *mut *mut libc::c_char)
 -> libc::c_long {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: errcode_t = 0;
    let mut names: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
    if !profile.is_null() {
        names[0 as libc::c_int as usize] = name;
        names[1 as libc::c_int as usize] = subname;
        names[2 as libc::c_int as usize] = subsubname;
        names[3 as libc::c_int as usize] = 0 as *const libc::c_char;
        retval = profile_get_value(profile, names.as_mut_ptr(), &mut value);
        if retval == 0 as libc::c_int as libc::c_long {
            *ret_string = value;
            return 0 as libc::c_int as libc::c_long
        } else {
            if retval != -(1429577726 as libc::c_long) &&
                   retval != -(1429577725 as libc::c_long) {
                return retval
            }
        }
    }
    if !def_val.is_null() {
        *ret_string = strdup(def_val);
        if (*ret_string).is_null() {
            return 12 as libc::c_int as libc::c_long
        }
    } else { *ret_string = 0 as *mut libc::c_char }
    return 0 as libc::c_int as libc::c_long;
}
#[c2rust::src_loc = "283:1"]
unsafe extern "C" fn parse_int(mut value: *const libc::c_char,
                               mut ret_int: *mut libc::c_int) -> errcode_t {
    let mut end_value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret_long: libc::c_long = 0;
    if *value.offset(0 as libc::c_int as isize) as libc::c_int ==
           0 as libc::c_int {
        /* Empty string is no good.  */
        return -(1429577699 as libc::c_long)
    }
    *__errno_location() = 0 as libc::c_int;
    ret_long = strtol(value, &mut end_value, 10 as libc::c_int);
    /* Overflow or underflow.  */
    if (ret_long == -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
            || ret_long == 9223372036854775807 as libc::c_long) &&
           *__errno_location() != 0 as libc::c_int {
        return -(1429577699 as libc::c_long)
    }
    /* Value outside "int" range.  */
    if ret_long as libc::c_int as libc::c_long != ret_long {
        return -(1429577699 as libc::c_long)
    }
    /* Garbage in string.  */
    if end_value != value.offset(strlen(value) as isize) as *mut libc::c_char
       {
        return -(1429577699 as libc::c_long)
    }
    *ret_int = ret_long as libc::c_int;
    return 0 as libc::c_int as errcode_t;
}
#[no_mangle]
#[c2rust::src_loc = "309:1"]
pub unsafe extern "C" fn profile_get_integer(mut profile: profile_t,
                                             mut name: *const libc::c_char,
                                             mut subname: *const libc::c_char,
                                             mut subsubname:
                                                 *const libc::c_char,
                                             mut def_val: libc::c_int,
                                             mut ret_int: *mut libc::c_int)
 -> libc::c_long {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: errcode_t = 0;
    let mut names: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
    *ret_int = def_val;
    if profile.is_null() { return 0 as libc::c_int as libc::c_long }
    names[0 as libc::c_int as usize] = name;
    names[1 as libc::c_int as usize] = subname;
    names[2 as libc::c_int as usize] = subsubname;
    names[3 as libc::c_int as usize] = 0 as *const libc::c_char;
    retval = profile_get_value(profile, names.as_mut_ptr(), &mut value);
    if retval == -(1429577726 as libc::c_long) ||
           retval == -(1429577725 as libc::c_long) {
        *ret_int = def_val;
        return 0 as libc::c_int as libc::c_long
    } else { if retval != 0 { return retval } }
    retval = parse_int(value, ret_int);
    free(value as *mut libc::c_void);
    return retval;
}
#[c2rust::src_loc = "337:26"]
static mut conf_yes: [*const libc::c_char; 7] =
    [b"y\x00" as *const u8 as *const libc::c_char,
     b"yes\x00" as *const u8 as *const libc::c_char,
     b"true\x00" as *const u8 as *const libc::c_char,
     b"t\x00" as *const u8 as *const libc::c_char,
     b"1\x00" as *const u8 as *const libc::c_char,
     b"on\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
#[c2rust::src_loc = "342:26"]
static mut conf_no: [*const libc::c_char; 7] =
    [b"n\x00" as *const u8 as *const libc::c_char,
     b"no\x00" as *const u8 as *const libc::c_char,
     b"false\x00" as *const u8 as *const libc::c_char,
     b"nil\x00" as *const u8 as *const libc::c_char,
     b"0\x00" as *const u8 as *const libc::c_char,
     b"off\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "347:1"]
unsafe extern "C" fn profile_parse_boolean(mut s: *const libc::c_char,
                                           mut ret_boolean: *mut libc::c_int)
 -> errcode_t {
    let mut p: *const *const libc::c_char = 0 as *const *const libc::c_char;
    if ret_boolean.is_null() { return -(1429577717 as libc::c_long) }
    p = conf_yes.as_ptr();
    while !(*p).is_null() {
        if strcasecmp(*p, s) == 0 {
            *ret_boolean = 1 as libc::c_int;
            return 0 as libc::c_int as errcode_t
        }
        p = p.offset(1)
    }
    p = conf_no.as_ptr();
    while !(*p).is_null() {
        if strcasecmp(*p, s) == 0 {
            *ret_boolean = 0 as libc::c_int;
            return 0 as libc::c_int as errcode_t
        }
        p = p.offset(1)
    }
    return -(1429577700 as libc::c_long);
}
#[no_mangle]
#[c2rust::src_loc = "372:1"]
pub unsafe extern "C" fn profile_get_boolean(mut profile: profile_t,
                                             mut name: *const libc::c_char,
                                             mut subname: *const libc::c_char,
                                             mut subsubname:
                                                 *const libc::c_char,
                                             mut def_val: libc::c_int,
                                             mut ret_boolean:
                                                 *mut libc::c_int)
 -> libc::c_long {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: errcode_t = 0;
    let mut names: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
    if profile.is_null() {
        *ret_boolean = def_val;
        return 0 as libc::c_int as libc::c_long
    }
    names[0 as libc::c_int as usize] = name;
    names[1 as libc::c_int as usize] = subname;
    names[2 as libc::c_int as usize] = subsubname;
    names[3 as libc::c_int as usize] = 0 as *const libc::c_char;
    retval = profile_get_value(profile, names.as_mut_ptr(), &mut value);
    if retval == -(1429577726 as libc::c_long) ||
           retval == -(1429577725 as libc::c_long) {
        *ret_boolean = def_val;
        return 0 as libc::c_int as libc::c_long
    } else { if retval != 0 { return retval } }
    retval = profile_parse_boolean(value, ret_boolean);
    free(value as *mut libc::c_void);
    return retval;
}
/*
 * This function will return the list of the names of subections in the
 * under the specified section name.
 */
#[no_mangle]
#[c2rust::src_loc = "405:1"]
pub unsafe extern "C" fn profile_get_subsection_names(mut profile: profile_t,
                                                      mut names:
                                                          *mut *const libc::c_char,
                                                      mut ret_names:
                                                          *mut *mut *mut libc::c_char)
 -> libc::c_long {
    let mut current_block: u64;
    let mut retval: errcode_t = 0;
    let mut state: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut values: profile_string_list =
        profile_string_list{list: 0 as *mut *mut libc::c_char,
                            num: 0,
                            max: 0,};
    retval =
        profile_iterator_create(profile, names,
                                0x1 as libc::c_int | 0x2 as libc::c_int,
                                &mut state);
    if retval != 0 { return retval }
    retval = init_list(&mut values);
    if retval != 0 { return retval }
    loop  {
        retval =
            profile_iterator(&mut state, &mut name,
                             0 as *mut *mut libc::c_char);
        if retval != 0 { current_block = 16236312898130099128; break ; }
        if !name.is_null() { add_to_list(&mut values, name); }
        free(name as *mut libc::c_void);
        if state.is_null() { current_block = 13536709405535804910; break ; }
    }
    match current_block {
        16236312898130099128 => {
            end_list(&mut values, 0 as *mut *mut *mut libc::c_char);
            return retval
        }
        _ => {
            end_list(&mut values, ret_names);
            return 0 as libc::c_int as libc::c_long
        }
    };
}
/*
 * This function will return the list of the names of relations in the
 * under the specified section name.
 */
#[no_mangle]
#[c2rust::src_loc = "443:1"]
pub unsafe extern "C" fn profile_get_relation_names(mut profile: profile_t,
                                                    mut names:
                                                        *mut *const libc::c_char,
                                                    mut ret_names:
                                                        *mut *mut *mut libc::c_char)
 -> libc::c_long {
    let mut current_block: u64;
    let mut retval: errcode_t = 0;
    let mut state: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut values: profile_string_list =
        profile_string_list{list: 0 as *mut *mut libc::c_char,
                            num: 0,
                            max: 0,};
    retval =
        profile_iterator_create(profile, names,
                                0x1 as libc::c_int | 0x4 as libc::c_int,
                                &mut state);
    if retval != 0 { return retval }
    retval = init_list(&mut values);
    if retval != 0 { return retval }
    loop  {
        retval =
            profile_iterator(&mut state, &mut name,
                             0 as *mut *mut libc::c_char);
        if retval != 0 { current_block = 815861999143219977; break ; }
        if !name.is_null() && is_list_member(&mut values, name) == 0 {
            add_to_list(&mut values, name);
        }
        free(name as *mut libc::c_void);
        if state.is_null() { current_block = 13536709405535804910; break ; }
    }
    match current_block {
        815861999143219977 => {
            end_list(&mut values, 0 as *mut *mut *mut libc::c_char);
            return retval
        }
        _ => {
            end_list(&mut values, ret_names);
            return 0 as libc::c_int as libc::c_long
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "483:1"]
pub unsafe extern "C" fn profile_iterator_create(mut profile: profile_t,
                                                 mut names:
                                                     *const *const libc::c_char,
                                                 mut flags: libc::c_int,
                                                 mut ret_iter:
                                                     *mut *mut libc::c_void)
 -> libc::c_long {
    let mut iter: *mut profile_iterator = 0 as *mut profile_iterator;
    let mut retval: errcode_t = 0;
    *ret_iter = 0 as *mut libc::c_void;
    if profile.is_null() { return -(1429577704 as libc::c_long) }
    iter =
        malloc(::std::mem::size_of::<profile_iterator>() as libc::c_ulong) as
            *mut profile_iterator;
    if iter.is_null() { return 12 as libc::c_int as libc::c_long }
    (*iter).magic = -(1429577719 as libc::c_long);
    (*iter).profile = profile;
    /* Create the underlying iterator representation using the vtable or the
     * built-in node iterator. */
    if !(*profile).vt.is_null() {
        if (*(*profile).vt).iterator_create.is_none() {
            retval = -(1429577695 as libc::c_long)
        } else {
            retval =
                (*(*profile).vt).iterator_create.expect("non-null function pointer")((*profile).cbdata,
                                                                                     names,
                                                                                     flags,
                                                                                     &mut (*iter).idata)
        }
    } else {
        retval =
            profile_node_iterator_create(profile, names, flags,
                                         &mut (*iter).idata)
    }
    if retval != 0 { free(iter as *mut libc::c_void); return retval }
    *ret_iter = iter as *mut libc::c_void;
    return 0 as libc::c_int as libc::c_long;
}
#[no_mangle]
#[c2rust::src_loc = "521:1"]
pub unsafe extern "C" fn profile_iterator_free(mut iter_p:
                                                   *mut *mut libc::c_void) {
    let mut iter: *mut profile_iterator = 0 as *mut profile_iterator;
    let mut profile: profile_t = 0 as *mut _profile_t;
    if iter_p.is_null() { return }
    iter = *iter_p as *mut profile_iterator;
    if iter.is_null() || (*iter).magic != -(1429577719 as libc::c_long) {
        return
    }
    profile = (*iter).profile;
    if !(*profile).vt.is_null() {
        (*(*profile).vt).iterator_free.expect("non-null function pointer")((*profile).cbdata,
                                                                           (*iter).idata);
    } else { profile_node_iterator_free(&mut (*iter).idata); }
    free(iter as *mut libc::c_void);
    *iter_p = 0 as *mut libc::c_void;
}
/* Make copies of name and value into *ret_name and *ret_value.  Handle null
 * values of any argument. */
#[c2rust::src_loc = "543:1"]
unsafe extern "C" fn set_results(mut name: *const libc::c_char,
                                 mut value: *const libc::c_char,
                                 mut ret_name: *mut *mut libc::c_char,
                                 mut ret_value: *mut *mut libc::c_char)
 -> errcode_t {
    let mut current_block: u64;
    let mut name_copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value_copy: *mut libc::c_char = 0 as *mut libc::c_char;
    if !ret_name.is_null() && !name.is_null() {
        name_copy = strdup(name);
        if name_copy.is_null() {
            current_block = 6492784773663475637;
        } else { current_block = 17778012151635330486; }
    } else { current_block = 17778012151635330486; }
    match current_block {
        17778012151635330486 => {
            if !ret_value.is_null() && !value.is_null() {
                value_copy = strdup(value);
                if value_copy.is_null() {
                    current_block = 6492784773663475637;
                } else { current_block = 7815301370352969686; }
            } else { current_block = 7815301370352969686; }
            match current_block {
                6492784773663475637 => { }
                _ => {
                    if !ret_name.is_null() { *ret_name = name_copy }
                    if !ret_value.is_null() { *ret_value = value_copy }
                    return 0 as libc::c_int as errcode_t
                }
            }
        }
        _ => { }
    }
    free(name_copy as *mut libc::c_void);
    free(value_copy as *mut libc::c_void);
    return 12 as libc::c_int as errcode_t;
}
#[no_mangle]
#[c2rust::src_loc = "570:1"]
pub unsafe extern "C" fn profile_iterator(mut iter_p: *mut *mut libc::c_void,
                                          mut ret_name:
                                              *mut *mut libc::c_char,
                                          mut ret_value:
                                              *mut *mut libc::c_char)
 -> libc::c_long {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: errcode_t = 0;
    let mut iter: *mut profile_iterator = *iter_p as *mut profile_iterator;
    let mut profile: profile_t = 0 as *mut _profile_t;
    if !ret_name.is_null() { *ret_name = 0 as *mut libc::c_char }
    if !ret_value.is_null() { *ret_value = 0 as *mut libc::c_char }
    if iter.is_null() || (*iter).magic != -(1429577719 as libc::c_long) {
        return -(1429577719 as libc::c_long)
    }
    profile = (*iter).profile;
    if !(*profile).vt.is_null() {
        retval =
            (*(*profile).vt).iterator.expect("non-null function pointer")((*profile).cbdata,
                                                                          (*iter).idata,
                                                                          &mut name,
                                                                          &mut value);
        if retval != 0 { return retval }
        if name.is_null() {
            (*(*profile).vt).iterator_free.expect("non-null function pointer")((*profile).cbdata,
                                                                               (*iter).idata);
            free(iter as *mut libc::c_void);
            *iter_p = 0 as *mut libc::c_void
        }
        retval = set_results(name, value, ret_name, ret_value);
        if !name.is_null() {
            (*(*profile).vt).free_string.expect("non-null function pointer")((*profile).cbdata,
                                                                             name);
        }
        if !value.is_null() {
            (*(*profile).vt).free_string.expect("non-null function pointer")((*profile).cbdata,
                                                                             value);
        }
        return retval
    }
    retval =
        profile_node_iterator(&mut (*iter).idata, 0 as *mut *mut profile_node,
                              &mut name, &mut value);
    if (*iter).idata.is_null() {
        free(iter as *mut libc::c_void);
        *iter_p = 0 as *mut libc::c_void
    }
    if retval != 0 { return retval }
    return set_results(name, value, ret_name, ret_value);
}
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
#[no_mangle]
#[c2rust::src_loc = "614:1"]
pub unsafe extern "C" fn profile_release_string(mut str: *mut libc::c_char) {
    free(str as *mut libc::c_void);
}
