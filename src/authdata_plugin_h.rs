pub type authdata_client_plugin_fini_proc =
    Option<unsafe extern "C" fn(_: crate::krb5_h::krb5_context, _: *mut libc::c_void) -> ()>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5plugin_authdata_client_ftable_v0 {
    pub name: *mut i8,
    pub ad_type_list: *mut crate::krb5_h::krb5_authdatatype,
    pub init: crate::authdata_plugin_h::authdata_client_plugin_init_proc,
    pub fini: crate::authdata_plugin_h::authdata_client_plugin_fini_proc,
    pub flags: crate::authdata_plugin_h::authdata_client_plugin_flags_proc,
    pub request_init: crate::authdata_plugin_h::authdata_client_request_init_proc,
    pub request_fini: crate::authdata_plugin_h::authdata_client_request_fini_proc,
    pub get_attribute_types: crate::authdata_plugin_h::authdata_client_get_attribute_types_proc,
    pub get_attribute: crate::authdata_plugin_h::authdata_client_get_attribute_proc,
    pub set_attribute: crate::authdata_plugin_h::authdata_client_set_attribute_proc,
    pub delete_attribute: crate::authdata_plugin_h::authdata_client_delete_attribute_proc,
    pub export_authdata: crate::authdata_plugin_h::authdata_client_export_authdata_proc,
    pub import_authdata: crate::authdata_plugin_h::authdata_client_import_authdata_proc,
    pub export_internal: crate::authdata_plugin_h::authdata_client_export_internal_proc,
    pub free_internal: crate::authdata_plugin_h::authdata_client_free_internal_proc,
    pub verify: crate::authdata_plugin_h::authdata_client_verify_proc,
    pub size: crate::authdata_plugin_h::authdata_client_size_proc,
    pub externalize: crate::authdata_plugin_h::authdata_client_externalize_proc,
    pub internalize: crate::authdata_plugin_h::authdata_client_internalize_proc,
    pub copy: crate::authdata_plugin_h::authdata_client_copy_proc,
}
pub type authdata_client_request_init_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::_krb5_authdata_context,
        _: *mut libc::c_void,
        _: *mut *mut libc::c_void,
    ) -> crate::krb5_h::krb5_error_code,
>;
pub type authdata_client_request_fini_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::_krb5_authdata_context,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
    ) -> (),
>;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */

/*
 * Copyright (C) 2007 Apple Inc.  All Rights Reserved.
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
 * Authorization data plugin definitions for Kerberos 5.
 * This is considered an INTERNAL interface at this time.
 *
 * Some work is needed before exporting it:
 *
 * + Documentation.
 * + Sample code.
 * + Test cases (preferably automated testing under "make check").
 *
 * Other changes that would be nice to have, but not necessarily
 * before making this interface public:
 *
 * + Library support for AD-IF-RELEVANT and similar wrappers.  (We can
 *   make the plugin construct them if it wants them.)
 * + KDC could combine/optimize wrapped AD elements provided by
 *   multiple plugins, e.g., two IF-RELEVANT sequences could be
 *   merged.  (The preauth plugin API also has this bug, we're going
 *   to need a general fix.)
 */
pub type authdata_client_plugin_init_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut *mut libc::c_void,
    ) -> crate::krb5_h::krb5_error_code,
>;
pub type authdata_client_copy_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::_krb5_authdata_context,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
    ) -> crate::krb5_h::krb5_error_code,
>;
pub type authdata_client_internalize_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::_krb5_authdata_context,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *mut *mut crate::krb5_h::krb5_octet,
        _: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code,
>;
pub type authdata_client_externalize_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::_krb5_authdata_context,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *mut *mut crate::krb5_h::krb5_octet,
        _: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code,
>;
pub type authdata_client_size_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::_krb5_authdata_context,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code,
>;
pub type authdata_client_verify_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::_krb5_authdata_context,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *const crate::krb5_h::krb5_auth_context,
        _: *const crate::krb5_h::krb5_keyblock,
        _: *const crate::krb5_h::krb5_ap_req,
    ) -> crate::krb5_h::krb5_error_code,
>;
pub type authdata_client_free_internal_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::_krb5_authdata_context,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
    ) -> (),
>;
pub type authdata_client_export_internal_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::_krb5_authdata_context,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: crate::krb5_h::krb5_boolean,
        _: *mut *mut libc::c_void,
    ) -> crate::krb5_h::krb5_error_code,
>;
pub type authdata_client_import_authdata_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::_krb5_authdata_context,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *mut *mut crate::krb5_h::krb5_authdata,
        _: crate::krb5_h::krb5_boolean,
        _: crate::krb5_h::krb5_const_principal,
    ) -> crate::krb5_h::krb5_error_code,
>;
pub type authdata_client_export_authdata_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::_krb5_authdata_context,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: crate::krb5_h::krb5_flags,
        _: *mut *mut *mut crate::krb5_h::krb5_authdata,
    ) -> crate::krb5_h::krb5_error_code,
>;
pub type authdata_client_delete_attribute_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::_krb5_authdata_context,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *const crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code,
>;
pub type authdata_client_set_attribute_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::_krb5_authdata_context,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: crate::krb5_h::krb5_boolean,
        _: *const crate::krb5_h::krb5_data,
        _: *const crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code,
>;
pub type authdata_client_get_attribute_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::_krb5_authdata_context,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *const crate::krb5_h::krb5_data,
        _: *mut crate::krb5_h::krb5_boolean,
        _: *mut crate::krb5_h::krb5_boolean,
        _: *mut crate::krb5_h::krb5_data,
        _: *mut crate::krb5_h::krb5_data,
        _: *mut i32,
    ) -> crate::krb5_h::krb5_error_code,
>;
pub type authdata_client_get_attribute_types_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::_krb5_authdata_context,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *mut *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code,
>;
pub type authdata_client_plugin_flags_proc = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut libc::c_void,
        _: crate::krb5_h::krb5_authdatatype,
        _: *mut crate::krb5_h::krb5_flags,
    ) -> (),
>;
