extern "C" {
    #[no_mangle]
    pub fn krb5_sendto_kdc(
        _: crate::krb5_h::krb5_context,
        _: *const crate::krb5_h::krb5_data,
        _: *const crate::krb5_h::krb5_data,
        _: *mut crate::krb5_h::krb5_data,
        _: *mut i32,
        _: i32,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_free_iakerb_header(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::krb5_iakerb_header,
    );

    #[no_mangle]
    pub fn krb5_free_iakerb_finished(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::k5_int_h::krb5_iakerb_finished,
    );

    #[no_mangle]
    pub fn krb5_authdata_export_authdata(
        kcontext: crate::krb5_h::krb5_context,
        context: crate::k5_int_h::krb5_authdata_context,
        usage: crate::krb5_h::krb5_flags,
        pauthdata: *mut *mut *mut crate::krb5_h::krb5_authdata,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn encode_krb5_iakerb_header(
        _: *const crate::k5_int_h::krb5_iakerb_header,
        _: *mut *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn encode_krb5_iakerb_finished(
        _: *const crate::k5_int_h::krb5_iakerb_finished,
        _: *mut *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn decode_krb5_error(
        output: *const crate::krb5_h::krb5_data,
        rep: *mut *mut crate::krb5_h::krb5_error,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn decode_krb5_iakerb_header(
        _: *const crate::krb5_h::krb5_data,
        _: *mut *mut crate::k5_int_h::krb5_iakerb_header,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn decode_krb5_iakerb_finished(
        _: *const crate::krb5_h::krb5_data,
        _: *mut *mut crate::k5_int_h::krb5_iakerb_finished,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_get_credentials_for_user(
        context: crate::krb5_h::krb5_context,
        options: crate::krb5_h::krb5_flags,
        ccache: crate::krb5_h::krb5_ccache,
        in_creds: *mut crate::krb5_h::krb5_creds,
        cert: *mut crate::krb5_h::krb5_data,
        out_creds: *mut *mut crate::krb5_h::krb5_creds,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5int_free_data_list(
        context: crate::krb5_h::krb5_context,
        data: *mut crate::krb5_h::krb5_data,
    );

    #[no_mangle]
    pub fn krb5_authdata_get_attribute_types(
        kcontext: crate::krb5_h::krb5_context,
        context: crate::k5_int_h::krb5_authdata_context,
        attrs: *mut *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_authdata_get_attribute(
        kcontext: crate::krb5_h::krb5_context,
        context: crate::k5_int_h::krb5_authdata_context,
        attribute: *const crate::krb5_h::krb5_data,
        authenticated: *mut crate::krb5_h::krb5_boolean,
        complete: *mut crate::krb5_h::krb5_boolean,
        value: *mut crate::krb5_h::krb5_data,
        display_value: *mut crate::krb5_h::krb5_data,
        more: *mut i32,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_authdata_set_attribute(
        kcontext: crate::krb5_h::krb5_context,
        context: crate::k5_int_h::krb5_authdata_context,
        complete: crate::krb5_h::krb5_boolean,
        attribute: *const crate::krb5_h::krb5_data,
        value: *const crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_authdata_delete_attribute(
        kcontext: crate::krb5_h::krb5_context,
        context: crate::k5_int_h::krb5_authdata_context,
        attribute: *const crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_authdata_export_attributes(
        kcontext: crate::krb5_h::krb5_context,
        context: crate::k5_int_h::krb5_authdata_context,
        usage: crate::krb5_h::krb5_flags,
        pattributes: *mut *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_authdata_export_internal(
        kcontext: crate::krb5_h::krb5_context,
        context: crate::k5_int_h::krb5_authdata_context,
        restrict_authenticated: crate::krb5_h::krb5_boolean,
        module: *const i8,
        ptr: *mut *mut libc::c_void,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_authdata_context_copy(
        kcontext: crate::krb5_h::krb5_context,
        src: crate::k5_int_h::krb5_authdata_context,
        dst: *mut crate::k5_int_h::krb5_authdata_context,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_authdata_free_internal(
        kcontext: crate::krb5_h::krb5_context,
        context: crate::k5_int_h::krb5_authdata_context,
        module: *const i8,
        ptr: *mut libc::c_void,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_size_authdata_context(
        kcontext: crate::krb5_h::krb5_context,
        context: crate::k5_int_h::krb5_authdata_context,
        sizep: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_externalize_authdata_context(
        kcontext: crate::krb5_h::krb5_context,
        context: crate::k5_int_h::krb5_authdata_context,
        buffer: *mut *mut crate::krb5_h::krb5_octet,
        lenremain: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_internalize_authdata_context(
        kcontext: crate::krb5_h::krb5_context,
        ptr: *mut crate::k5_int_h::krb5_authdata_context,
        buffer: *mut *mut crate::krb5_h::krb5_octet,
        lenremain: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_size_auth_context(
        auth_context: crate::krb5_h::krb5_auth_context,
        sizep: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_externalize_auth_context(
        auth_context: crate::krb5_h::krb5_auth_context,
        buffer: *mut *mut crate::krb5_h::krb5_octet,
        lenremain: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_internalize_auth_context(
        argp: *mut crate::krb5_h::krb5_auth_context,
        buffer: *mut *mut crate::krb5_h::krb5_octet,
        lenremain: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_size_authdata(
        authdata: *mut crate::krb5_h::krb5_authdata,
        sizep: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_externalize_authdata(
        authdata: *mut crate::krb5_h::krb5_authdata,
        buffer: *mut *mut crate::krb5_h::krb5_octet,
        lenremain: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_internalize_authdata(
        authdata: *mut *mut crate::krb5_h::krb5_authdata,
        buffer: *mut *mut crate::krb5_h::krb5_octet,
        lenremain: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_size_context(
        context: crate::krb5_h::krb5_context,
        sizep: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_externalize_context(
        context: crate::krb5_h::krb5_context,
        buffer: *mut *mut crate::krb5_h::krb5_octet,
        lenremain: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_internalize_context(
        argp: *mut crate::krb5_h::krb5_context,
        buffer: *mut *mut crate::krb5_h::krb5_octet,
        lenremain: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_size_keyblock(
        keyblock: *mut crate::krb5_h::krb5_keyblock,
        sizep: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_externalize_keyblock(
        keyblock: *mut crate::krb5_h::krb5_keyblock,
        buffer: *mut *mut crate::krb5_h::krb5_octet,
        lenremain: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_internalize_keyblock(
        argp: *mut *mut crate::krb5_h::krb5_keyblock,
        buffer: *mut *mut crate::krb5_h::krb5_octet,
        lenremain: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_size_principal(
        principal: crate::krb5_h::krb5_principal,
        sizep: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_externalize_principal(
        principal: crate::krb5_h::krb5_principal,
        buffer: *mut *mut crate::krb5_h::krb5_octet,
        lenremain: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_internalize_principal(
        argp: *mut crate::krb5_h::krb5_principal,
        buffer: *mut *mut crate::krb5_h::krb5_octet,
        lenremain: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_ser_pack_int32(
        _: crate::krb5_h::krb5_int32,
        _: *mut *mut crate::krb5_h::krb5_octet,
        _: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_ser_unpack_int32(
        _: *mut crate::krb5_h::krb5_int32,
        _: *mut *mut crate::krb5_h::krb5_octet,
        _: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_ser_pack_bytes(
        _: *mut crate::krb5_h::krb5_octet,
        _: crate::stddef_h::size_t,
        _: *mut *mut crate::krb5_h::krb5_octet,
        _: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_ser_unpack_bytes(
        _: *mut crate::krb5_h::krb5_octet,
        _: crate::stddef_h::size_t,
        _: *mut *mut crate::krb5_h::krb5_octet,
        _: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_authdata_context_init(
        kcontext: crate::krb5_h::krb5_context,
        pcontext: *mut crate::k5_int_h::krb5_authdata_context,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_authdata_context_free(
        kcontext: crate::krb5_h::krb5_context,
        context: crate::k5_int_h::krb5_authdata_context,
    );

    #[no_mangle]
    pub fn krb5_authdata_import_attributes(
        kcontext: crate::krb5_h::krb5_context,
        context: crate::k5_int_h::krb5_authdata_context,
        usage: crate::krb5_h::krb5_flags,
        attributes: *const crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_rc_get_name(
        context: crate::krb5_h::krb5_context,
        rc: crate::krb5_h::krb5_rcache,
    ) -> *const i8;

    #[no_mangle]
    pub fn krb5int_arcfour_gsscrypt(
        keyblock: *const crate::krb5_h::krb5_keyblock,
        usage: crate::krb5_h::krb5_keyusage,
        kd_data: *const crate::krb5_h::krb5_data,
        data: *mut crate::krb5_h::krb5_crypto_iov,
        num_data: crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5int_c_mandatory_cksumtype(
        _: crate::krb5_h::krb5_context,
        _: crate::krb5_h::krb5_enctype,
        _: *mut crate::krb5_h::krb5_cksumtype,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5int_copy_data_contents_add0(
        _: crate::krb5_h::krb5_context,
        _: *const crate::krb5_h::krb5_data,
        _: *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5int_cc_default(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::krb5_h::krb5_ccache,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_rc_resolve(
        context: crate::krb5_h::krb5_context,
        name: *const i8,
        rc_out: *mut crate::krb5_h::krb5_rcache,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_rc_close(context: crate::krb5_h::krb5_context, rc: crate::krb5_h::krb5_rcache);

    #[no_mangle]
    pub fn k5_kt_get_principal(
        context: crate::krb5_h::krb5_context,
        keytab: crate::krb5_h::krb5_keytab,
        princ_out: *mut crate::krb5_h::krb5_principal,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn k5_change_error_message_code(
        ctx: crate::krb5_h::krb5_context,
        oldcode: crate::krb5_h::krb5_error_code,
        newcode: crate::krb5_h::krb5_error_code,
    );

    #[no_mangle]
    pub fn k5_enctype_to_ssf(
        enctype: crate::krb5_h::krb5_enctype,
        ssf_out: *mut u32,
    ) -> crate::krb5_h::krb5_error_code;

    pub type plugin_mapping;

    pub type _kdb_log_context;

    pub type k5_tls_vtable_st;

    pub type hostrealm_module_handle;

    pub type localauth_module_handle;

    pub type ccselect_module_handle;

    pub type krb5_preauth_context_st;

    pub type _kdb5_dal_handle;

    pub type _krb5_key_data;

    #[no_mangle]
    pub fn encode_krb5_ticket(
        rep: *const crate::krb5_h::krb5_ticket,
        code: *mut *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn decode_krb5_ap_req(
        output: *const crate::krb5_h::krb5_data,
        rep: *mut *mut crate::krb5_h::krb5_ap_req,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5int_accessor(
        _: *mut crate::k5_int_h::krb5int_access,
        _: crate::krb5_h::krb5_int32,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_free_ap_req(_: crate::krb5_h::krb5_context, _: *mut crate::krb5_h::krb5_ap_req);

    #[no_mangle]
    pub fn krb5_rd_req_decoded(
        _: crate::krb5_h::krb5_context,
        _: *mut crate::krb5_h::krb5_auth_context,
        _: *const crate::krb5_h::krb5_ap_req,
        _: crate::krb5_h::krb5_const_principal,
        _: crate::krb5_h::krb5_keytab,
        _: *mut crate::krb5_h::krb5_flags,
        _: *mut *mut crate::krb5_h::krb5_ticket,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_auth_con_setpermetypes(
        _: crate::krb5_h::krb5_context,
        _: crate::krb5_h::krb5_auth_context,
        _: *const crate::krb5_h::krb5_enctype,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_auth_con_get_authdata_context(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        ad_context: *mut crate::k5_int_h::krb5_authdata_context,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_auth_con_set_authdata_context(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        ad_context: crate::k5_int_h::krb5_authdata_context,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5int_init_context_kdc(
        _: *mut crate::krb5_h::krb5_context,
    ) -> crate::krb5_h::krb5_error_code;
}
// =============== BEGIN k5_int_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_iakerb_header {
    pub target_realm: crate::krb5_h::krb5_data,
    pub cookie: *mut crate::krb5_h::krb5_data,
}
pub type krb5_iakerb_header = crate::k5_int_h::_krb5_iakerb_header;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_iakerb_finished {
    pub checksum: crate::krb5_h::krb5_checksum,
}
pub type krb5_iakerb_finished = crate::k5_int_h::_krb5_iakerb_finished;
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_context {
    pub magic: crate::krb5_h::krb5_magic,
    pub in_tkt_etypes: *mut crate::krb5_h::krb5_enctype,
    pub tgs_etypes: *mut crate::krb5_h::krb5_enctype,
    pub os_context: crate::k5_int_h::_krb5_os_context,
    pub default_realm: *mut i8,
    pub profile: crate::profile_h::profile_t,
    pub dal_handle: *mut crate::k5_int_h::kdb5_dal_handle,
    pub clockskew: crate::krb5_h::krb5_deltat,
    pub kdc_default_options: crate::krb5_h::krb5_flags,
    pub library_options: crate::krb5_h::krb5_flags,
    pub profile_secure: crate::krb5_h::krb5_boolean,
    pub fcc_default_format: i32,
    pub prompt_types: *mut crate::krb5_h::krb5_prompt_type,
    pub udp_pref_limit: i32,
    pub use_conf_ktypes: crate::krb5_h::krb5_boolean,
    pub libkrb5_plugins: crate::k5_plugin_h::plugin_dir_handle,
    pub preauth_context: crate::k5_int_h::krb5_preauth_context,
    pub ccselect_handles: *mut *mut crate::k5_int_h::ccselect_module_handle,
    pub localauth_handles: *mut *mut crate::k5_int_h::localauth_module_handle,
    pub hostrealm_handles: *mut *mut crate::k5_int_h::hostrealm_module_handle,
    pub tls: *mut crate::k5_int_h::k5_tls_vtable_st,
    pub err: crate::k5_err_h::errinfo,
    pub err_fmt: *mut i8,
    pub kdblog_context: *mut crate::k5_int_h::_kdb_log_context,
    pub allow_weak_crypto: crate::krb5_h::krb5_boolean,
    pub ignore_acceptor_hostname: crate::krb5_h::krb5_boolean,
    pub enforce_ok_as_delegate: crate::krb5_h::krb5_boolean,
    pub dns_canonicalize_hostname: crate::k5_int_h::dns_canonhost,
    pub trace_callback: crate::krb5_h::krb5_trace_callback,
    pub trace_callback_data: *mut libc::c_void,
    pub kdc_send_hook: crate::krb5_h::krb5_pre_send_fn,
    pub kdc_send_hook_data: *mut libc::c_void,
    pub kdc_recv_hook: crate::krb5_h::krb5_post_recv_fn,
    pub kdc_recv_hook_data: *mut libc::c_void,
    pub plugins: [crate::k5_int_h::plugin_interface; 13],
    pub plugin_base_dir: *mut i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_key_st {
    pub keyblock: crate::krb5_h::krb5_keyblock,
    pub refcount: i32,
    pub derived: *mut crate::k5_int_h::derived_key,
    pub cache: *mut libc::c_void,
}
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_kt {
    pub magic: crate::krb5_h::krb5_magic,
    pub ops: *const crate::k5_int_h::_krb5_kt_ops,
    pub data: crate::krb5_h::krb5_pointer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plugin_interface {
    pub modules: *mut *mut crate::k5_int_h::plugin_mapping,
    pub configured: crate::krb5_h::krb5_boolean,
}
pub type dns_canonhost = u32;
pub type krb5_preauth_context = *mut crate::k5_int_h::krb5_preauth_context_st;
pub type kdb5_dal_handle = crate::k5_int_h::_kdb5_dal_handle;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_os_context {
    pub magic: crate::krb5_h::krb5_magic,
    pub time_offset: crate::krb5_h::krb5_int32,
    pub usec_offset: crate::krb5_h::krb5_int32,
    pub os_flags: crate::krb5_h::krb5_int32,
    pub default_ccname: *mut i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct derived_key {
    pub constant: crate::krb5_h::krb5_data,
    pub dkey: crate::krb5_h::krb5_key,
    pub next: *mut crate::k5_int_h::derived_key,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_kt_ops {
    pub magic: crate::krb5_h::krb5_magic,
    pub prefix: *mut i8,
    pub resolve: Option<
        unsafe extern "C" fn(
            _: crate::krb5_h::krb5_context,
            _: *const i8,
            _: *mut crate::krb5_h::krb5_keytab,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub get_name: Option<
        unsafe extern "C" fn(
            _: crate::krb5_h::krb5_context,
            _: crate::krb5_h::krb5_keytab,
            _: *mut i8,
            _: u32,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub close: Option<
        unsafe extern "C" fn(
            _: crate::krb5_h::krb5_context,
            _: crate::krb5_h::krb5_keytab,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub get: Option<
        unsafe extern "C" fn(
            _: crate::krb5_h::krb5_context,
            _: crate::krb5_h::krb5_keytab,
            _: crate::krb5_h::krb5_const_principal,
            _: crate::krb5_h::krb5_kvno,
            _: crate::krb5_h::krb5_enctype,
            _: *mut crate::krb5_h::krb5_keytab_entry,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub start_seq_get: Option<
        unsafe extern "C" fn(
            _: crate::krb5_h::krb5_context,
            _: crate::krb5_h::krb5_keytab,
            _: *mut crate::krb5_h::krb5_kt_cursor,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub get_next: Option<
        unsafe extern "C" fn(
            _: crate::krb5_h::krb5_context,
            _: crate::krb5_h::krb5_keytab,
            _: *mut crate::krb5_h::krb5_keytab_entry,
            _: *mut crate::krb5_h::krb5_kt_cursor,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub end_get: Option<
        unsafe extern "C" fn(
            _: crate::krb5_h::krb5_context,
            _: crate::krb5_h::krb5_keytab,
            _: *mut crate::krb5_h::krb5_kt_cursor,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub add: Option<
        unsafe extern "C" fn(
            _: crate::krb5_h::krb5_context,
            _: crate::krb5_h::krb5_keytab,
            _: *mut crate::krb5_h::krb5_keytab_entry,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub remove: Option<
        unsafe extern "C" fn(
            _: crate::krb5_h::krb5_context,
            _: crate::krb5_h::krb5_keytab,
            _: *mut crate::krb5_h::krb5_keytab_entry,
        ) -> crate::krb5_h::krb5_error_code,
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_authdata_context {
    pub magic: crate::krb5_h::krb5_magic,
    pub n_modules: i32,
    pub modules: *mut crate::k5_int_h::_krb5_authdata_context_module,
    pub plugins: crate::k5_plugin_h::plugin_dir_handle,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_authdata_context_module {
    pub ad_type: crate::krb5_h::krb5_authdatatype,
    pub plugin_context: *mut libc::c_void,
    pub client_fini: crate::authdata_plugin_h::authdata_client_plugin_fini_proc,
    pub flags: crate::krb5_h::krb5_flags,
    pub ftable: *mut crate::authdata_plugin_h::krb5plugin_authdata_client_ftable_v0,
    pub client_req_init: crate::authdata_plugin_h::authdata_client_request_init_proc,
    pub client_req_fini: crate::authdata_plugin_h::authdata_client_request_fini_proc,
    pub name: *const i8,
    pub request_context: *mut libc::c_void,
    pub request_context_pp: *mut *mut libc::c_void,
}
pub type krb5_authdata_context = *mut crate::k5_int_h::_krb5_authdata_context;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ldap_seqof_key_data {
    pub mkvno: crate::krb5_h::krb5_int32,
    pub kvno: crate::krb5_h::krb5_ui_2,
    pub key_data: *mut crate::k5_int_h::_krb5_key_data,
    pub n_key_data: crate::krb5_h::krb5_int16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5int_access {
    pub auth_con_get_subkey_enctype: Option<
        unsafe extern "C" fn(
            _: crate::krb5_h::krb5_context,
            _: crate::krb5_h::krb5_auth_context,
            _: *mut crate::krb5_h::krb5_enctype,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub mandatory_cksumtype: Option<
        unsafe extern "C" fn(
            _: crate::krb5_h::krb5_context,
            _: crate::krb5_h::krb5_enctype,
            _: *mut crate::krb5_h::krb5_cksumtype,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub ser_pack_int64: Option<
        unsafe extern "C" fn(
            _: crate::stdlib::int64_t,
            _: *mut *mut crate::krb5_h::krb5_octet,
            _: *mut crate::stddef_h::size_t,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub ser_unpack_int64: Option<
        unsafe extern "C" fn(
            _: *mut crate::stdlib::int64_t,
            _: *mut *mut crate::krb5_h::krb5_octet,
            _: *mut crate::stddef_h::size_t,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub asn1_ldap_encode_sequence_of_keys: Option<
        unsafe extern "C" fn(
            _: *const crate::k5_int_h::ldap_seqof_key_data,
            _: *mut *mut crate::krb5_h::krb5_data,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub asn1_ldap_decode_sequence_of_keys: Option<
        unsafe extern "C" fn(
            _: *const crate::krb5_h::krb5_data,
            _: *mut *mut crate::k5_int_h::ldap_seqof_key_data,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub encode_krb5_auth_pack: Option<
        unsafe extern "C" fn(
            _: *const crate::k5_int_pkinit_h::krb5_auth_pack,
            _: *mut *mut crate::krb5_h::krb5_data,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub encode_krb5_kdc_dh_key_info: Option<
        unsafe extern "C" fn(
            _: *const crate::k5_int_pkinit_h::krb5_kdc_dh_key_info,
            _: *mut *mut crate::krb5_h::krb5_data,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub encode_krb5_pa_pk_as_rep: Option<
        unsafe extern "C" fn(
            _: *const crate::k5_int_pkinit_h::krb5_pa_pk_as_rep,
            _: *mut *mut crate::krb5_h::krb5_data,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub encode_krb5_pa_pk_as_req: Option<
        unsafe extern "C" fn(
            _: *const crate::k5_int_pkinit_h::krb5_pa_pk_as_req,
            _: *mut *mut crate::krb5_h::krb5_data,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub encode_krb5_reply_key_pack: Option<
        unsafe extern "C" fn(
            _: *const crate::k5_int_pkinit_h::krb5_reply_key_pack,
            _: *mut *mut crate::krb5_h::krb5_data,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub encode_krb5_td_dh_parameters: Option<
        unsafe extern "C" fn(
            _: *const *mut crate::k5_int_pkinit_h::krb5_algorithm_identifier,
            _: *mut *mut crate::krb5_h::krb5_data,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub encode_krb5_td_trusted_certifiers: Option<
        unsafe extern "C" fn(
            _: *const *mut crate::k5_int_pkinit_h::krb5_external_principal_identifier,
            _: *mut *mut crate::krb5_h::krb5_data,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub decode_krb5_auth_pack: Option<
        unsafe extern "C" fn(
            _: *const crate::krb5_h::krb5_data,
            _: *mut *mut crate::k5_int_pkinit_h::krb5_auth_pack,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub decode_krb5_pa_pk_as_req: Option<
        unsafe extern "C" fn(
            _: *const crate::krb5_h::krb5_data,
            _: *mut *mut crate::k5_int_pkinit_h::krb5_pa_pk_as_req,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub decode_krb5_pa_pk_as_rep: Option<
        unsafe extern "C" fn(
            _: *const crate::krb5_h::krb5_data,
            _: *mut *mut crate::k5_int_pkinit_h::krb5_pa_pk_as_rep,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub decode_krb5_kdc_dh_key_info: Option<
        unsafe extern "C" fn(
            _: *const crate::krb5_h::krb5_data,
            _: *mut *mut crate::k5_int_pkinit_h::krb5_kdc_dh_key_info,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub decode_krb5_principal_name: Option<
        unsafe extern "C" fn(
            _: *const crate::krb5_h::krb5_data,
            _: *mut *mut crate::krb5_h::krb5_principal_data,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub decode_krb5_reply_key_pack: Option<
        unsafe extern "C" fn(
            _: *const crate::krb5_h::krb5_data,
            _: *mut *mut crate::k5_int_pkinit_h::krb5_reply_key_pack,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub decode_krb5_td_dh_parameters: Option<
        unsafe extern "C" fn(
            _: *const crate::krb5_h::krb5_data,
            _: *mut *mut *mut crate::k5_int_pkinit_h::krb5_algorithm_identifier,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub decode_krb5_td_trusted_certifiers: Option<
        unsafe extern "C" fn(
            _: *const crate::krb5_h::krb5_data,
            _: *mut *mut *mut crate::k5_int_pkinit_h::krb5_external_principal_identifier,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub encode_krb5_kdc_req_body: Option<
        unsafe extern "C" fn(
            _: *const crate::krb5_h::krb5_kdc_req,
            _: *mut *mut crate::krb5_h::krb5_data,
        ) -> crate::krb5_h::krb5_error_code,
    >,
    pub free_kdc_req: Option<
        unsafe extern "C" fn(
            _: crate::krb5_h::krb5_context,
            _: *mut crate::krb5_h::krb5_kdc_req,
        ) -> (),
    >,
    pub set_prompt_types: Option<
        unsafe extern "C" fn(
            _: crate::krb5_h::krb5_context,
            _: *mut crate::krb5_h::krb5_prompt_type,
        ) -> (),
    >,
}
pub type krb5int_access = crate::k5_int_h::_krb5int_access;
pub const CANONHOST_FALLBACK: crate::k5_int_h::dns_canonhost = 2;
pub const CANONHOST_TRUE: crate::k5_int_h::dns_canonhost = 1;
pub const CANONHOST_FALSE: crate::k5_int_h::dns_canonhost = 0;
