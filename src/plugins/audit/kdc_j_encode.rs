use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:33"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:33"]
pub mod krb5_h {
    /* typedef struct _profile_t *profile_t; */
    /*
 * begin wordsize.h
 */
    /*
 * Word-size related definition.
 */
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "175:1"]
    pub type krb5_msgtype = libc::c_uint;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "181:1"]
    pub type krb5_authdatatype = krb5_int32;
    #[c2rust::src_loc = "185:1"]
    pub type krb5_preauthtype = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    /* *
 * Represents a timestamp in seconds since the POSIX epoch.  This legacy type
 * is used frequently in the ABI, but cannot represent timestamps after 2038 as
 * a positive number.  Code which uses this type should cast values of it to
 * uint32_t so that negative values are treated as timestamps between 2038 and
 * 2106 on platforms with 64-bit time_t.
 */
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "323:16"]
    pub struct _krb5_address {
        pub magic: krb5_magic,
        pub addrtype: krb5_addrtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< Anonymous realm */
    /* *< Anonymous principal name */
    /*
 * end "base-defs.h"
 */
    /*
 * begin "hostaddr.h"
 */
    /* * Structure for address */
    #[c2rust::src_loc = "323:1"]
    pub type krb5_address = _krb5_address;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /*
 * begin "encryption.h"
 */
    /* * Exposed contents of a key. */
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "398:16"]
    pub struct _krb5_enc_data {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub kvno: krb5_kvno,
        pub ciphertext: krb5_data,
    }
    #[c2rust::src_loc = "398:1"]
    pub type krb5_enc_data = _krb5_enc_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
    /* *< RFC 6560 section 4.3 */
    /* *< RFC 6112 */
    /* *< RFC 6806 */
    /* *< RFC 8070 */
    /* *< MS-KILE and MS-SFU */
    /* *< currently must be zero */
    /* * Transited encoding types */
    /* * alternate authentication types */
    /* authorization data types. See RFC 4120 section 5.2.6 */
    /* * @defgroup KRB5_AUTHDATA KRB5_AUTHDATA
 * @{
 */
    /* *< RFC 4537 */
    /* *< formerly 142 in krb5 1.8 */
    /* * @} */
    /* end of KRB5_AUTHDATA group */
    /* password change constants */
    /* *< Success */
    /* *< Malformed request */
    /* *< Server error */
    /* *< Authentication error */
    /* *< Password change rejected */
    /* These are Microsoft's extensions in RFC 3244, and it looks like
   they'll become standardized, possibly with other additions.  */
    /* *< Not authorized */
    /* *< Unknown RPC version */
    /* * The presented credentials were not obtained using a password directly */
    /*
 * end "proto.h"
 */
    /* Time set */
/* * Ticket start time, end time, and renewal duration. */
    #[c2rust::src_loc = "1936:1"]
    pub type krb5_ticket_times = _krb5_ticket_times;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1946:16"]
    pub struct _krb5_authdata {
        pub magic: krb5_magic,
        pub ad_type: krb5_authdatatype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< Time at which KDC issued the initial ticket that corresponds to this ticket */
    /* XXX ? should ktime in KDC_REP == authtime
       in ticket? otherwise client can't get this */
    /* *< optional in ticket, if not present, use @a authtime */
    /* *< Ticket expiration time */
    /* *< Latest time at which renewal of ticket can be valid */
    /* * Structure for auth data */
    #[c2rust::src_loc = "1946:1"]
    pub type krb5_authdata = _krb5_authdata;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1954:16"]
    pub struct _krb5_transited {
        pub magic: krb5_magic,
        pub tr_type: krb5_octet,
        pub tr_contents: krb5_data,
    }
    /* *< ADTYPE */
    /* *< Length of data  */
    /* *< Data */
    /* * Structure for transited encoding */
    #[c2rust::src_loc = "1954:1"]
    pub type krb5_transited = _krb5_transited;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1961:16"]
    pub struct _krb5_enc_tkt_part {
        pub magic: krb5_magic,
        pub flags: krb5_flags,
        pub session: *mut krb5_keyblock,
        pub client: krb5_principal,
        pub transited: krb5_transited,
        pub times: krb5_ticket_times,
        pub caddrs: *mut *mut krb5_address,
        pub authorization_data: *mut *mut krb5_authdata,
    }
    /* *< Transited encoding type */
    /* *< Contents */
    /* * Encrypted part of ticket. */
    #[c2rust::src_loc = "1961:1"]
    pub type krb5_enc_tkt_part = _krb5_enc_tkt_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1979:16"]
    pub struct _krb5_ticket {
        pub magic: krb5_magic,
        pub server: krb5_principal,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_enc_tkt_part,
    }
    /* to-be-encrypted portion */
    /* *< flags */
    /* *< session key: includes enctype */
    /* *< client name/realm */
    /* *< list of transited realms */
    /* *< auth, start, end, renew_till */
    /* *< array of ptrs to addresses */
    /* *< auth data */
    /* *
 * Ticket structure.
 *
 * The C representation of the ticket message, with a pointer to the
 * C representation of the encrypted part.
 */
    #[c2rust::src_loc = "1979:1"]
    pub type krb5_ticket = _krb5_ticket;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2031:16"]
    pub struct _krb5_last_req_entry {
        pub magic: krb5_magic,
        pub lr_type: krb5_int32,
        pub value: krb5_timestamp,
    }
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
    /* * Last request entry */
    #[c2rust::src_loc = "2031:1"]
    pub type krb5_last_req_entry = _krb5_last_req_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2038:16"]
    pub struct _krb5_pa_data {
        pub magic: krb5_magic,
        pub pa_type: krb5_preauthtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< LR type */
    /* *< Timestamp */
    /* * Pre-authentication data */
    #[c2rust::src_loc = "2038:1"]
    pub type krb5_pa_data = _krb5_pa_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2054:16"]
    pub struct _krb5_kdc_req {
        pub magic: krb5_magic,
        pub msg_type: krb5_msgtype,
        pub padata: *mut *mut krb5_pa_data,
        pub kdc_options: krb5_flags,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub from: krb5_timestamp,
        pub till: krb5_timestamp,
        pub rtime: krb5_timestamp,
        pub nonce: krb5_int32,
        pub nktypes: libc::c_int,
        pub ktype: *mut krb5_enctype,
        pub addresses: *mut *mut krb5_address,
        pub authorization_data: krb5_enc_data,
        pub unenc_authdata: *mut *mut krb5_authdata,
        pub second_ticket: *mut *mut krb5_ticket,
    }
    /* *< Preauthentication data type */
    /* *< Length of data */
    /* *< Data */
    /* * C representation of KDC-REQ protocol message, including KDC-REQ-BODY */
    #[c2rust::src_loc = "2054:1"]
    pub type krb5_kdc_req = _krb5_kdc_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2079:16"]
    pub struct _krb5_enc_kdc_rep_part {
        pub magic: krb5_magic,
        pub msg_type: krb5_msgtype,
        pub session: *mut krb5_keyblock,
        pub last_req: *mut *mut krb5_last_req_entry,
        pub nonce: krb5_int32,
        pub key_exp: krb5_timestamp,
        pub flags: krb5_flags,
        pub times: krb5_ticket_times,
        pub server: krb5_principal,
        pub caddrs: *mut *mut krb5_address,
        pub enc_padata: *mut *mut krb5_pa_data,
    }
    /* *< KRB5_AS_REQ or KRB5_TGS_REQ */
    /* *< Preauthentication data */
    /* real body */
    /* *< Requested options */
    /* *< Client principal and realm */
    /* *< Server principal and realm */
    /* *< Requested start time */
    /* *< Requested end time */
    /* *< Requested renewable end time */
    /* *< Nonce to match request and response */
    /* *< Number of enctypes */
    /* *< Requested enctypes */
    /* *< Requested addresses (optional) */
    /* *< Encrypted authz data (optional) */
    /* *< Unencrypted authz data */
    /* *< Second ticket array (optional) */
    /* *
 * C representation of @c EncKDCRepPart protocol message.
 *
 * This is the cleartext message that is encrypted and inserted in @c KDC-REP.
 */
    #[c2rust::src_loc = "2079:1"]
    pub type krb5_enc_kdc_rep_part = _krb5_enc_kdc_rep_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2095:16"]
    pub struct _krb5_kdc_rep {
        pub magic: krb5_magic,
        pub msg_type: krb5_msgtype,
        pub padata: *mut *mut krb5_pa_data,
        pub client: krb5_principal,
        pub ticket: *mut krb5_ticket,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_enc_kdc_rep_part,
    }
    /* encrypted part: */
    /* *< krb5 message type */
    /* *< Session key */
    /* *< Array of pointers to entries */
    /* *< Nonce from request */
    /* *< Expiration date */
    /* *< Ticket flags */
    /* *< Lifetime info */
    /* *< Server's principal identifier */
    /* *< Array of ptrs to addrs, optional */
    /* *< Encrypted preauthentication data */
    /* * Representation of the @c KDC-REP protocol message. */
    #[c2rust::src_loc = "2095:1"]
    pub type krb5_kdc_rep = _krb5_kdc_rep;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    /* cleartext part: */
    /* *< KRB5_AS_REP or KRB5_KDC_REP */
    /* *< Preauthentication data from KDC */
    /* *< Client principal and realm */
    /* *< Ticket */
    /* *< Encrypted part of reply */
    /* *< Unencrypted version, if available */
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-json.h:34"]
pub mod k5_json_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-json.h - JSON declarations */
/*
 * Copyright (c) 2010 Kungliga Tekniska Högskolan
 * (Royal Institute of Technology, Stockholm, Sweden).
 * All rights reserved.
 *
 * Portions Copyright (c) 2010 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the Institute nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE INSTITUTE AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE INSTITUTE OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
/*
 * Copyright (C) 2012 by the Massachusetts Institute of Technology.
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
 * The k5_json_value C type can represent any kind of JSON value.  It has no
 * static type safety since it is represented using a void pointer, so be
 * careful with it.  Its type can be checked dynamically with k5_json_get_tid()
 * and the above constants.
 */
    #[c2rust::src_loc = "86:1"]
    pub type k5_json_value = *mut libc::c_void;
    /*
 * Boolean
 */
    #[c2rust::src_loc = "119:1"]
    pub type k5_json_bool = *mut k5_json_bool_st;
    /*
 * Array
 */
    #[c2rust::src_loc = "128:1"]
    pub type k5_json_array = *mut k5_json_array_st;
    /*
 * Object
 */
    #[c2rust::src_loc = "160:1"]
    pub type k5_json_object = *mut k5_json_object_st;
    /*
 * String
 */
    #[c2rust::src_loc = "186:1"]
    pub type k5_json_string = *mut k5_json_string_st;
    /*
 * Number
 */
    #[c2rust::src_loc = "206:1"]
    pub type k5_json_number = *mut k5_json_number_st;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "119:16"]
        pub type k5_json_bool_st;
        #[c2rust::src_loc = "128:16"]
        pub type k5_json_array_st;
        #[c2rust::src_loc = "160:16"]
        pub type k5_json_object_st;
        #[c2rust::src_loc = "186:16"]
        pub type k5_json_string_st;
        #[c2rust::src_loc = "206:16"]
        pub type k5_json_number_st;
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn k5_json_release(val: k5_json_value);
        #[no_mangle]
        #[c2rust::src_loc = "121:1"]
        pub fn k5_json_bool_create(truth: libc::c_int,
                                   val_out: *mut k5_json_bool) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "130:1"]
        pub fn k5_json_array_create(val_out: *mut k5_json_array)
         -> libc::c_int;
        /* Both of these functions increment the reference count on val. */
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn k5_json_array_add(array: k5_json_array, val: k5_json_value)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "164:1"]
        pub fn k5_json_object_create(val_out: *mut k5_json_object)
         -> libc::c_int;
        /*
 * Store val into object at key, incrementing val's reference count and
 * releasing any previous value at key.  If val is NULL, key is removed from
 * obj if it exists, and obj remains unchanged if it does not.
 */
        #[no_mangle]
        #[c2rust::src_loc = "176:1"]
        pub fn k5_json_object_set(obj: k5_json_object,
                                  key: *const libc::c_char,
                                  val: k5_json_value) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "188:1"]
        pub fn k5_json_string_create(cstring: *const libc::c_char,
                                     val_out: *mut k5_json_string)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "189:1"]
        pub fn k5_json_string_create_len(data: *const libc::c_void,
                                         len: size_t,
                                         val_out: *mut k5_json_string)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "208:1"]
        pub fn k5_json_number_create(number: libc::c_longlong,
                                     val_out: *mut k5_json_number)
         -> libc::c_int;
        /*
 * JSON encoding and decoding
 */
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn k5_json_encode(val: k5_json_value,
                              json_out: *mut *mut libc::c_char)
         -> libc::c_int;
    }
    /* K5_JSON_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/audit_plugin.h:35"]
pub mod audit_plugin_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "80:16"]
    pub struct _krb5_audit_state {
        pub request: *mut krb5_kdc_req,
        pub reply: *mut krb5_kdc_rep,
        pub cl_addr: *mut krb5_address,
        pub cl_port: krb5_ui_4,
        pub stage: libc::c_int,
        pub status: *const libc::c_char,
        pub tkt_in_id: *mut libc::c_char,
        pub tkt_out_id: *mut libc::c_char,
        pub evid_tkt_id: *mut libc::c_char,
        pub req_id: [libc::c_char; 32],
        pub cl_realm: *mut krb5_data,
        pub s4u2self_user: krb5_principal,
        pub violation: libc::c_int,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/krb5/audit_plugin.h - Audit plugin interface */
/*
 * Copyright (C) 2013 by the Massachusetts Institute of Technology.
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
 * NOTE: This is a private interface and may change incompatibly
 *       between versions.
 */
/*
 * Declarations for KDC audit plugin module implementers.  Audit modules allow
 * the KDC to produce log output or audit records in any desired form.
 *
 * The audit interface has a single supported major version, which is 1.  Major
 * version 1 has a current minor version of 1.  Audit modules should define a
 * function named audit_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   audit_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                        krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for the interface and maj_ver:
 *   maj_ver == 1: Cast to krb5_audit_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* * KDC processing steps */
    /* *< Authenticate request and client */
    /* *< Determine service principal */
    /* *< Validate local and protocol policies */
    /* *< Issue ticket */
    /* *< Encrypt reply */
    /* * Types of violations */
    /* *< Protocol constraint */
    /* *< Local policy violation */
    /* Size of the alpha-numeric request ID */
    /* * KDC audit state structure and declarations */
    #[c2rust::src_loc = "80:1"]
    pub type krb5_audit_state = _krb5_audit_state;
    use super::krb5_h::{krb5_kdc_req, krb5_kdc_rep, krb5_address, krb5_ui_4,
                        krb5_data, krb5_principal};
    /* *< client address */
    /* *< client port */
    /* *< step in KDC processing */
    /* *< KDC status message */
    /* *< primary (TGT) ticket ID */
    /* *< derived (service or referral TGT) ticket ID */
    /* * for s4u2proxy - evidence ticket ID; for u2u - second ticket ID */
    /* *< request ID */
    /* *< referrals: remote client's realm */
    /* *< impersonated user */
    /* *< local or protocol policy problem */
    /* KRB5_AU_PLUGIN_H_INCLUDED */
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_msgtype, krb5_kvno, krb5_addrtype, krb5_enctype,
                       krb5_authdatatype, krb5_preauthtype, krb5_flags,
                       krb5_timestamp, krb5_error_code, krb5_magic,
                       _krb5_data, krb5_data, krb5_principal_data,
                       krb5_principal, _krb5_address, krb5_address,
                       _krb5_keyblock, krb5_keyblock, _krb5_enc_data,
                       krb5_enc_data, _krb5_ticket_times, krb5_ticket_times,
                       _krb5_authdata, krb5_authdata, _krb5_transited,
                       krb5_transited, _krb5_enc_tkt_part, krb5_enc_tkt_part,
                       _krb5_ticket, krb5_ticket, _krb5_last_req_entry,
                       krb5_last_req_entry, _krb5_pa_data, krb5_pa_data,
                       _krb5_kdc_req, krb5_kdc_req, _krb5_enc_kdc_rep_part,
                       krb5_enc_kdc_rep_part, _krb5_kdc_rep, krb5_kdc_rep};
pub use self::k5_json_h::{k5_json_value, k5_json_bool, k5_json_array,
                          k5_json_object, k5_json_string, k5_json_number,
                          k5_json_bool_st, k5_json_array_st,
                          k5_json_object_st, k5_json_string_st,
                          k5_json_number_st, k5_json_release,
                          k5_json_bool_create, k5_json_array_create,
                          k5_json_array_add, k5_json_object_create,
                          k5_json_object_set, k5_json_string_create,
                          k5_json_string_create_len, k5_json_number_create,
                          k5_json_encode};
pub use self::audit_plugin_h::{_krb5_audit_state, krb5_audit_state};
use self::string_h::strlen;
/* Map preauth numeric type to the naming string. */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "893:8"]
pub struct _patype_str {
    pub id: krb5_preauthtype,
    pub name: *mut libc::c_char,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*  plugins/audit/kdc_j_encode.h - Declarations for KDC audit json encoders */
/*
 * Copyright 2013 by the Massachusetts Institute of Technology.
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
/* Maximum length of the name of preauth type. */
/* KDC server STOP. Returns 0 on success. */
#[no_mangle]
#[c2rust::src_loc = "75:1"]
pub unsafe extern "C" fn kau_j_kdc_stop(ev_success: krb5_boolean,
                                        mut jout: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut obj: k5_json_object = 0 as k5_json_object;
    *jout = 0 as *mut libc::c_char;
    /* Main object. */
    if k5_json_object_create(&mut obj) != 0 { return 12 as libc::c_int }
    /* Audit event_ID and ev_success. */
    ret =
        string_to_value(b"KDC_STOP\x00" as *const u8 as *const libc::c_char,
                        obj,
                        b"event_name\x00" as *const u8 as
                            *const libc::c_char);
    if ret == 0 {
        ret =
            bool_to_value(ev_success, obj,
                          b"event_success\x00" as *const u8 as
                              *const libc::c_char)
    }
    if ret == 0 { ret = k5_json_encode(obj as k5_json_value, jout) }
    k5_json_release(obj as k5_json_value);
    return ret;
}
/* KDC server START. Returns 0 on success. */
#[no_mangle]
#[c2rust::src_loc = "99:1"]
pub unsafe extern "C" fn kau_j_kdc_start(ev_success: krb5_boolean,
                                         mut jout: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut obj: k5_json_object = 0 as k5_json_object;
    *jout = 0 as *mut libc::c_char;
    /* Main object. */
    if k5_json_object_create(&mut obj) != 0 { return 12 as libc::c_int }
    /* Audit event_ID and ev_success. */
    ret =
        string_to_value(b"KDC_START\x00" as *const u8 as *const libc::c_char,
                        obj,
                        b"event_name\x00" as *const u8 as
                            *const libc::c_char);
    if ret == 0 {
        ret =
            bool_to_value(ev_success, obj,
                          b"event_success\x00" as *const u8 as
                              *const libc::c_char)
    }
    if ret == 0 { ret = k5_json_encode(obj as k5_json_value, jout) }
    k5_json_release(obj as k5_json_value);
    return ret;
}
/* AS-REQ. Returns 0 on success. */
#[no_mangle]
#[c2rust::src_loc = "123:1"]
pub unsafe extern "C" fn kau_j_as_req(ev_success: krb5_boolean,
                                      mut state: *mut krb5_audit_state,
                                      mut jout: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut obj: k5_json_object = 0 as k5_json_object;
    *jout = 0 as *mut libc::c_char;
    if state.is_null() {
        *jout =
            b"state is NULL\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        return 0 as libc::c_int
    }
    /* Main object. */
    if k5_json_object_create(&mut obj) != 0 { return 12 as libc::c_int }
    /* Audit event_ID and ev_success. */
    ret =
        eventinfo_to_value(obj,
                           b"AS_REQ\x00" as *const u8 as *const libc::c_char,
                           (*state).stage, ev_success);
    if !(ret != 0) {
        /* TGT ticket ID */
        ret =
            string_to_value((*state).tkt_out_id, obj,
                            b"tkt_out_id\x00" as *const u8 as
                                *const libc::c_char);
        if !(ret != 0) {
            /* Request ID. */
            ret =
                string_to_value((*state).req_id.as_mut_ptr(), obj,
                                b"req_id\x00" as *const u8 as
                                    *const libc::c_char);
            if !(ret != 0) {
                /* Client's port and address. */
                ret =
                    int32_to_value((*state).cl_port as krb5_int32, obj,
                                   b"fromport\x00" as *const u8 as
                                       *const libc::c_char);
                if !(ret != 0) {
                    ret =
                        addr_to_value((*state).cl_addr, obj,
                                      b"fromaddr\x00" as *const u8 as
                                          *const libc::c_char);
                    if !(ret != 0) {
                        /* KDC status msg */
                        ret =
                            string_to_value((*state).status, obj,
                                            b"kdc_status\x00" as *const u8 as
                                                *const libc::c_char);
                        if !(ret != 0) {
                            /* non-local client's referral realm. */
                            ret =
                                data_to_value((*state).cl_realm, obj,
                                              b"clreferral_realm\x00" as
                                                  *const u8 as
                                                  *const libc::c_char);
                            if !(ret != 0) {
                                /* Request. */
                                ret =
                                    req_to_value((*state).request, ev_success,
                                                 obj);
                                if !(ret == 12 as libc::c_int) {
                                    /* Reply/ticket info. */
                                    ret =
                                        rep_to_value((*state).reply,
                                                     ev_success, obj);
                                    if !(ret == 12 as libc::c_int) {
                                        ret =
                                            k5_json_encode(obj as
                                                               k5_json_value,
                                                           jout)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    k5_json_release(obj as k5_json_value);
    return ret;
}
/* TGS-REQ. Returns 0 on success. */
#[no_mangle]
#[c2rust::src_loc = "183:1"]
pub unsafe extern "C" fn kau_j_tgs_req(ev_success: krb5_boolean,
                                       mut state: *mut krb5_audit_state,
                                       mut jout: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut obj: k5_json_object = 0 as k5_json_object;
    let mut req: *mut krb5_kdc_req = (*state).request;
    let mut tkt_validated: libc::c_int = 0 as libc::c_int;
    let mut tkt_renewed: libc::c_int = 0 as libc::c_int;
    *jout = 0 as *mut libc::c_char;
    if state.is_null() {
        *jout =
            b"state is NULL\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        return 0 as libc::c_int
    }
    /* Main object. */
    if k5_json_object_create(&mut obj) != 0 { return 12 as libc::c_int }
    /* Audit Event ID and ev_success. */
    ret =
        eventinfo_to_value(obj,
                           b"TGS_REQ\x00" as *const u8 as *const libc::c_char,
                           (*state).stage, ev_success);
    if !(ret != 0) {
        /* Primary and derived ticket IDs. */
        ret =
            string_to_value((*state).tkt_in_id, obj,
                            b"tkt_in_id\x00" as *const u8 as
                                *const libc::c_char);
        if !(ret != 0) {
            ret =
                string_to_value((*state).tkt_out_id, obj,
                                b"tkt_out_id\x00" as *const u8 as
                                    *const libc::c_char);
            if !(ret != 0) {
                /* Request ID */
                ret =
                    string_to_value((*state).req_id.as_mut_ptr(), obj,
                                    b"req_id\x00" as *const u8 as
                                        *const libc::c_char);
                if !(ret != 0) {
                    /* client’s address and port. */
                    ret =
                        int32_to_value((*state).cl_port as krb5_int32, obj,
                                       b"fromport\x00" as *const u8 as
                                           *const libc::c_char);
                    if !(ret != 0) {
                        ret =
                            addr_to_value((*state).cl_addr, obj,
                                          b"fromaddr\x00" as *const u8 as
                                              *const libc::c_char);
                        if !(ret != 0) {
                            /* Ticket was renewed, validated. */
                            if ev_success == 1 as libc::c_int as libc::c_uint
                                   && !req.is_null() {
                                tkt_renewed =
                                    if (*req).kdc_options & 0x2 as libc::c_int
                                           != 0 {
                                        1 as libc::c_int
                                    } else { 2 as libc::c_int };
                                tkt_validated =
                                    if (*req).kdc_options & 0x1 as libc::c_int
                                           != 0 {
                                        1 as libc::c_int
                                    } else { 2 as libc::c_int }
                            }
                            ret =
                                int32_to_value(tkt_renewed, obj,
                                               b"tkt_renewed\x00" as *const u8
                                                   as *const libc::c_char);
                            if !(ret != 0) {
                                ret =
                                    int32_to_value(tkt_validated, obj,
                                                   b"tkt_validated\x00" as
                                                       *const u8 as
                                                       *const libc::c_char);
                                if !(ret != 0) {
                                    /* KDC status msg, including "ISSUE". */
                                    ret =
                                        string_to_value((*state).status, obj,
                                                        b"kdc_status\x00" as
                                                            *const u8 as
                                                            *const libc::c_char);
                                    if !(ret != 0) {
                                        /* request */
                                        ret =
                                            req_to_value(req, ev_success,
                                                         obj);
                                        if !(ret == 12 as libc::c_int) {
                                            /* reply/ticket */
                                            ret =
                                                rep_to_value((*state).reply,
                                                             ev_success, obj);
                                            if !(ret == 12 as libc::c_int) {
                                                ret =
                                                    k5_json_encode(obj as
                                                                       k5_json_value,
                                                                   jout)
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
    }
    k5_json_release(obj as k5_json_value);
    return ret;
}
/* S4U2Self protocol extension. Returns 0 on success. */
#[no_mangle]
#[c2rust::src_loc = "258:1"]
pub unsafe extern "C" fn kau_j_tgs_s4u2self(ev_success: krb5_boolean,
                                            mut state: *mut krb5_audit_state,
                                            mut jout: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut obj: k5_json_object = 0 as k5_json_object;
    *jout = 0 as *mut libc::c_char;
    if state.is_null() {
        *jout =
            b"state is NULL\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        return 0 as libc::c_int
    }
    /* Main object. */
    if k5_json_object_create(&mut obj) != 0 { return 12 as libc::c_int }
    /* Audit Event ID and ev_success. */
    ret =
        eventinfo_to_value(obj,
                           b"S4U2SELF\x00" as *const u8 as
                               *const libc::c_char, (*state).stage,
                           ev_success);
    if !(ret != 0) {
        /* Front-end server's TGT ticket ID. */
        ret =
            string_to_value((*state).tkt_in_id, obj,
                            b"tkt_in_id\x00" as *const u8 as
                                *const libc::c_char);
        if !(ret != 0) {
            /* service "to self" ticket or referral TGT ticket ID. */
            ret =
                string_to_value((*state).tkt_out_id, obj,
                                b"tkt_out_id\x00" as *const u8 as
                                    *const libc::c_char);
            if !(ret != 0) {
                /* Request ID. */
                ret =
                    string_to_value((*state).req_id.as_mut_ptr(), obj,
                                    b"req_id\x00" as *const u8 as
                                        *const libc::c_char);
                if !(ret != 0) {
                    if ev_success == 0 as libc::c_int as libc::c_uint {
                        /* KDC status msg. */
                        ret =
                            string_to_value((*state).status, obj,
                                            b"kdc_status\x00" as *const u8 as
                                                *const libc::c_char);
                        if ret != 0 {
                            current_block = 12044166946535675237;
                        } else {
                            /* Local policy or S4U protocol constraints. */
                            ret =
                                int32_to_value((*state).violation, obj,
                                               b"violation\x00" as *const u8
                                                   as *const libc::c_char);
                            if ret != 0 {
                                current_block = 12044166946535675237;
                            } else { current_block = 4808432441040389987; }
                        }
                    } else { current_block = 4808432441040389987; }
                    match current_block {
                        12044166946535675237 => { }
                        _ => {
                            /* Impersonated user. */
                            ret =
                                princ_to_value((*state).s4u2self_user, obj,
                                               b"s4u2self_user\x00" as
                                                   *const u8 as
                                                   *const libc::c_char);
                            if !(ret != 0) {
                                ret =
                                    k5_json_encode(obj as k5_json_value, jout)
                            }
                        }
                    }
                }
            }
        }
    }
    k5_json_release(obj as k5_json_value);
    return ret;
}
/* S4U2Proxy protocol extension. Returns 0 on success. */
#[no_mangle]
#[c2rust::src_loc = "315:1"]
pub unsafe extern "C" fn kau_j_tgs_s4u2proxy(ev_success: krb5_boolean,
                                             mut state: *mut krb5_audit_state,
                                             mut jout: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut obj: k5_json_object = 0 as k5_json_object;
    let mut req: *mut krb5_kdc_req = (*state).request;
    *jout = 0 as *mut libc::c_char;
    if state.is_null() {
        *jout =
            b"state is NULL\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        return 0 as libc::c_int
    }
    /* Main object. */
    if k5_json_object_create(&mut obj) != 0 { return 12 as libc::c_int }
    /* Audit Event ID and ev_success. */
    ret =
        eventinfo_to_value(obj,
                           b"S4U2PROXY\x00" as *const u8 as
                               *const libc::c_char, (*state).stage,
                           ev_success);
    if !(ret != 0) {
        /* Front-end server's TGT ticket ID. */
        ret =
            string_to_value((*state).tkt_in_id, obj,
                            b"tkt_in_id\x00" as *const u8 as
                                *const libc::c_char);
        if !(ret != 0) {
            /* Resource service or referral TGT ticket ID. */
            ret =
                string_to_value((*state).tkt_out_id, obj,
                                b"tkt_out_id\x00" as *const u8 as
                                    *const libc::c_char);
            if !(ret != 0) {
                /* User's evidence ticket ID. */
                ret =
                    string_to_value((*state).evid_tkt_id, obj,
                                    b"evidence_tkt_id\x00" as *const u8 as
                                        *const libc::c_char);
                if !(ret != 0) {
                    /* Request ID. */
                    ret =
                        string_to_value((*state).req_id.as_mut_ptr(), obj,
                                        b"req_id\x00" as *const u8 as
                                            *const libc::c_char);
                    if !(ret != 0) {
                        if ev_success == 0 as libc::c_int as libc::c_uint {
                            /* KDC status msg. */
                            ret =
                                string_to_value((*state).status, obj,
                                                b"kdc_status\x00" as *const u8
                                                    as *const libc::c_char);
                            if ret != 0 {
                                current_block = 16396731022070410122;
                            } else {
                                /* Local policy or S4U protocol constraints. */
                                ret =
                                    int32_to_value((*state).violation, obj,
                                                   b"violation\x00" as
                                                       *const u8 as
                                                       *const libc::c_char);
                                if ret != 0 {
                                    current_block = 16396731022070410122;
                                } else {
                                    current_block = 1109700713171191020;
                                }
                            }
                        } else { current_block = 1109700713171191020; }
                        match current_block {
                            16396731022070410122 => { }
                            _ =>
                            /* Delegated user. */
                            {
                                if !req.is_null() {
                                    ret =
                                        princ_to_value((*(**(*req).second_ticket.offset(0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize)).enc_part2).client,
                                                       obj,
                                                       b"s4u2proxy_user\x00"
                                                           as *const u8 as
                                                           *const libc::c_char);
                                    if ret != 0 {
                                        current_block = 16396731022070410122;
                                    } else {
                                        current_block = 4068382217303356765;
                                    }
                                } else {
                                    current_block = 4068382217303356765;
                                }
                                match current_block {
                                    16396731022070410122 => { }
                                    _ => {
                                        ret =
                                            k5_json_encode(obj as
                                                               k5_json_value,
                                                           jout)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    k5_json_release(obj as k5_json_value);
    return ret;
}
/* U2U. Returns 0 on success. */
#[no_mangle]
#[c2rust::src_loc = "380:1"]
pub unsafe extern "C" fn kau_j_tgs_u2u(ev_success: krb5_boolean,
                                       mut state: *mut krb5_audit_state,
                                       mut jout: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut obj: k5_json_object = 0 as k5_json_object;
    let mut req: *mut krb5_kdc_req = (*state).request;
    if state.is_null() {
        *jout =
            b"state is NULL\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        return 0 as libc::c_int
    }
    *jout = 0 as *mut libc::c_char;
    /* Main object. */
    if k5_json_object_create(&mut obj) != 0 { return 12 as libc::c_int }
    /* Audit Event ID and ev_success. */
    ret =
        eventinfo_to_value(obj,
                           b"U2U\x00" as *const u8 as *const libc::c_char,
                           (*state).stage, ev_success);
    if !(ret != 0) {
        /* Front-end server's TGT ticket ID. */
        ret =
            string_to_value((*state).tkt_in_id, obj,
                            b"tkt_in_id\x00" as *const u8 as
                                *const libc::c_char);
        if !(ret != 0) {
            /* Service ticket ID. */
            ret =
                string_to_value((*state).tkt_out_id, obj,
                                b"tkt_out_id\x00" as *const u8 as
                                    *const libc::c_char);
            if !(ret != 0) {
                /* Request ID. */
                ret =
                    string_to_value((*state).req_id.as_mut_ptr(), obj,
                                    b"req_id\x00" as *const u8 as
                                        *const libc::c_char);
                if !(ret != 0) {
                    if ev_success == 0 as libc::c_int as libc::c_uint {
                        /* KDC status msg. */
                        ret =
                            string_to_value((*state).status, obj,
                                            b"kdc_status\x00" as *const u8 as
                                                *const libc::c_char);
                        if ret != 0 {
                            current_block = 2922615986599441907;
                        } else { current_block = 6057473163062296781; }
                    } else { current_block = 6057473163062296781; }
                    match current_block {
                        2922615986599441907 => { }
                        _ =>
                        /* Client in the second ticket. */
                        {
                            if !req.is_null() {
                                ret =
                                    princ_to_value((*(**(*req).second_ticket.offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).enc_part2).client,
                                                   obj,
                                                   b"u2u_user\x00" as
                                                       *const u8 as
                                                       *const libc::c_char);
                                if ret != 0 {
                                    current_block = 2922615986599441907;
                                } else {
                                    current_block = 18317007320854588510;
                                }
                            } else { current_block = 18317007320854588510; }
                            match current_block {
                                2922615986599441907 => { }
                                _ => {
                                    /* Enctype of a session key of the second ticket. */
                                    ret =
                                        int32_to_value((*(*(**(*req).second_ticket.offset(0
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize)).enc_part2).session).enctype,
                                                       obj,
                                                       b"srv_etype\x00" as
                                                           *const u8 as
                                                           *const libc::c_char);
                                    if !(ret != 0) {
                                        ret =
                                            k5_json_encode(obj as
                                                               k5_json_value,
                                                           jout)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    k5_json_release(obj as k5_json_value);
    return ret;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/audit/kdc_j_encode.c - Utilities to json encode KDC audit stuff */
/*
 * Copyright (C) 2013 by the Massachusetts Institute of Technology.
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
/* Low level utilities */
/* Converts string into a property of a JSON object. Returns 0 on success.*/
#[c2rust::src_loc = "444:1"]
unsafe extern "C" fn string_to_value(mut in_0: *const libc::c_char,
                                     mut obj: k5_json_object,
                                     mut key: *const libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut str: k5_json_string = 0 as k5_json_string;
    if in_0.is_null() { return 0 as libc::c_int }
    ret = k5_json_string_create(in_0, &mut str);
    if ret != 0 { return ret }
    ret = k5_json_object_set(obj, key, str as k5_json_value);
    k5_json_release(str as k5_json_value);
    return ret;
}
/*
 * Converts a krb5_data struct into a property of a JSON object.
 * (Borrowed from preauth_otp.c)
 * Returns 0 on success.
 */
#[c2rust::src_loc = "467:1"]
unsafe extern "C" fn data_to_value(mut data: *mut krb5_data,
                                   mut obj: k5_json_object,
                                   mut key: *const libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut str: k5_json_string = 0 as k5_json_string;
    if data.is_null() || (*data).data.is_null() ||
           (*data).length < 1 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    ret =
        k5_json_string_create_len((*data).data as *const libc::c_void,
                                  (*data).length as size_t, &mut str);
    if ret != 0 { return ret }
    ret = k5_json_object_set(obj, key, str as k5_json_value);
    k5_json_release(str as k5_json_value);
    return ret;
}
/*
 * Converts krb5_int32 into a property of a JSON object.
 * Returns 0 on success.
 */
#[c2rust::src_loc = "489:1"]
unsafe extern "C" fn int32_to_value(mut int32: krb5_int32,
                                    mut obj: k5_json_object,
                                    mut key: *const libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut num: k5_json_number = 0 as k5_json_number;
    ret = k5_json_number_create(int32 as libc::c_longlong, &mut num);
    if ret != 0 { return 12 as libc::c_int }
    ret = k5_json_object_set(obj, key, num as k5_json_value);
    k5_json_release(num as k5_json_value);
    return ret;
}
/*
 * Converts krb5_boolean into a property of a JSON object.
 * Returns 0 on success.
 */
#[c2rust::src_loc = "508:1"]
unsafe extern "C" fn bool_to_value(mut in_0: krb5_boolean,
                                   mut obj: k5_json_object,
                                   mut key: *const libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut b: k5_json_bool = 0 as k5_json_bool;
    ret = k5_json_bool_create(in_0 as libc::c_int, &mut b);
    if ret != 0 { return 12 as libc::c_int }
    ret = k5_json_object_set(obj, key, b as k5_json_value);
    k5_json_release(b as k5_json_value);
    return ret;
}
/* Wrapper-level utilities */
/* Wrapper for stage and event_status tags. Returns 0 on success. */
#[c2rust::src_loc = "527:1"]
unsafe extern "C" fn eventinfo_to_value(mut obj: k5_json_object,
                                        mut name: *const libc::c_char,
                                        stage: libc::c_int,
                                        ev_success: krb5_boolean)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    ret =
        string_to_value(name, obj,
                        b"event_name\x00" as *const u8 as
                            *const libc::c_char);
    if ret != 0 { return ret }
    ret =
        int32_to_value(stage, obj,
                       b"stage\x00" as *const u8 as *const libc::c_char);
    if ret == 0 {
        ret =
            bool_to_value(ev_success, obj,
                          b"event_success\x00" as *const u8 as
                              *const libc::c_char)
    }
    return ret;
}
/*
 * Converts krb5_principal into a property of a JSON object.
 * Returns 0 on success.
 */
#[c2rust::src_loc = "547:1"]
unsafe extern "C" fn princ_to_value(mut princ: krb5_principal,
                                    mut obj: k5_json_object,
                                    mut key: *const libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut tmp: k5_json_object = 0 as k5_json_object;
    let mut arr: k5_json_array = 0 as k5_json_array;
    let mut str: k5_json_string = 0 as k5_json_string;
    let mut i: libc::c_int = 0 as libc::c_int;
    if princ.is_null() || (*princ).data.is_null() { return 0 as libc::c_int }
    /* Main object. */
    if k5_json_object_create(&mut tmp) != 0 { return 12 as libc::c_int }
    ret = k5_json_array_create(&mut arr);
    if !(ret != 0) {
        i = 0 as libc::c_int;
        loop  {
            if !(i < (*princ).length) {
                current_block = 17407779659766490442;
                break ;
            }
            ret =
                k5_json_string_create_len((*(*princ).data.offset(i as
                                                                     isize)).data
                                              as *const libc::c_void,
                                          (*(*princ).data.offset(i as
                                                                     isize)).length
                                              as size_t, &mut str);
            if ret != 0 { current_block = 4668288037983582731; break ; }
            ret = k5_json_array_add(arr, str as k5_json_value);
            k5_json_release(str as k5_json_value);
            if ret != 0 { current_block = 4668288037983582731; break ; }
            i += 1
        }
        match current_block {
            4668288037983582731 => { }
            _ => {
                ret =
                    k5_json_object_set(tmp,
                                       b"components\x00" as *const u8 as
                                           *const libc::c_char,
                                       arr as k5_json_value);
                if !(ret != 0) {
                    ret =
                        data_to_value(&mut (*princ).realm, tmp,
                                      b"realm\x00" as *const u8 as
                                          *const libc::c_char);
                    if !(ret != 0) {
                        ret =
                            int32_to_value((*princ).length, tmp,
                                           b"length\x00" as *const u8 as
                                               *const libc::c_char);
                        if !(ret != 0) {
                            ret =
                                int32_to_value((*princ).type_0, tmp,
                                               b"type\x00" as *const u8 as
                                                   *const libc::c_char);
                            if !(ret != 0) {
                                ret =
                                    k5_json_object_set(obj, key,
                                                       tmp as k5_json_value)
                            }
                        }
                    }
                }
            }
        }
    }
    k5_json_release(tmp as k5_json_value);
    k5_json_release(arr as k5_json_value);
    return ret;
}
/*
 * Helper for JSON encoding of krb5_address.
 * Returns 0 on success.
 */
#[c2rust::src_loc = "601:1"]
unsafe extern "C" fn addr_to_obj(mut a: *mut krb5_address,
                                 mut obj: k5_json_object) -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut num: k5_json_number = 0 as k5_json_number;
    let mut arr: k5_json_array = 0 as k5_json_array;
    let mut i: libc::c_int = 0;
    if a.is_null() || (*a).contents.is_null() ||
           (*a).length <= 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    ret =
        int32_to_value((*a).addrtype, obj,
                       b"type\x00" as *const u8 as *const libc::c_char);
    if !(ret != 0) {
        ret =
            int32_to_value((*a).length as krb5_int32, obj,
                           b"length\x00" as *const u8 as *const libc::c_char);
        if !(ret != 0) {
            if (*a).addrtype == 0x2 as libc::c_int ||
                   (*a).addrtype == 0x18 as libc::c_int {
                ret = k5_json_array_create(&mut arr);
                if !(ret != 0) {
                    i = 0 as libc::c_int;
                    loop  {
                        if !(i < (*a).length as libc::c_int) {
                            current_block = 5689001924483802034;
                            break ;
                        }
                        ret =
                            k5_json_number_create(*(*a).contents.offset(i as
                                                                            isize)
                                                      as libc::c_longlong,
                                                  &mut num);
                        if ret != 0 {
                            current_block = 13895296802411494809;
                            break ;
                        }
                        ret = k5_json_array_add(arr, num as k5_json_value);
                        k5_json_release(num as k5_json_value);
                        if ret != 0 {
                            current_block = 13895296802411494809;
                            break ;
                        }
                        i += 1
                    }
                    match current_block {
                        13895296802411494809 => { }
                        _ => {
                            ret =
                                k5_json_object_set(obj,
                                                   b"ip\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   arr as k5_json_value);
                            (ret) != 0;
                        }
                    }
                }
            }
        }
    }
    k5_json_release(arr as k5_json_value);
    return ret;
}
/*
 * Converts krb5_fulladdr into a property of a JSON object.
 * Returns 0 on success.
 */
#[c2rust::src_loc = "646:1"]
unsafe extern "C" fn addr_to_value(mut address: *const krb5_address,
                                   mut obj: k5_json_object,
                                   mut key: *const libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut addr_obj: k5_json_object = 0 as k5_json_object;
    if address.is_null() { return 0 as libc::c_int }
    ret = k5_json_object_create(&mut addr_obj);
    if ret != 0 { return ret }
    ret = addr_to_obj(address as *mut krb5_address, addr_obj);
    if ret == 0 {
        ret = k5_json_object_set(obj, key, addr_obj as k5_json_value)
    }
    k5_json_release(addr_obj as k5_json_value);
    return ret;
}
/*
 * Helper for JSON encoding of krb5_kdc_req.
 * Returns 0 on success.
 */
#[c2rust::src_loc = "670:1"]
unsafe extern "C" fn req_to_value(mut req: *mut krb5_kdc_req,
                                  ev_success: krb5_boolean,
                                  mut obj: k5_json_object)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut num: k5_json_number = 0 as k5_json_number;
    let mut str: k5_json_string = 0 as k5_json_string;
    let mut tmpa: k5_json_object = 0 as k5_json_object;
    let mut arr: k5_json_array = 0 as k5_json_array;
    let mut arra: k5_json_array = 0 as k5_json_array;
    let mut arrpa: k5_json_array = 0 as k5_json_array;
    let mut padata: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut i: libc::c_int = 0 as libc::c_int;
    if req.is_null() { return 0 as libc::c_int }
    ret =
        princ_to_value((*req).client, obj,
                       b"req.client\x00" as *const u8 as *const libc::c_char);
    if !(ret != 0) {
        ret =
            princ_to_value((*req).server, obj,
                           b"req.server\x00" as *const u8 as
                               *const libc::c_char);
        if !(ret != 0) {
            ret =
                int32_to_value((*req).kdc_options, obj,
                               b"req.kdc_options\x00" as *const u8 as
                                   *const libc::c_char);
            if !(ret != 0) {
                ret =
                    int32_to_value((*req).from, obj,
                                   b"req.tkt_start\x00" as *const u8 as
                                       *const libc::c_char);
                if !(ret != 0) {
                    ret =
                        int32_to_value((*req).till, obj,
                                       b"req.tkt_end\x00" as *const u8 as
                                           *const libc::c_char);
                    if !(ret != 0) {
                        ret =
                            int32_to_value((*req).rtime, obj,
                                           b"req.tkt_renew_till\x00" as
                                               *const u8 as
                                               *const libc::c_char);
                        if !(ret != 0) {
                            /* Available/requested enctypes. */
                            ret = k5_json_array_create(&mut arr);
                            if !(ret != 0) {
                                i = 0 as libc::c_int;
                                loop  {
                                    if !(i < (*req).nktypes) {
                                        current_block = 14359455889292382949;
                                        break ;
                                    }
                                    if *(*req).ktype.offset(i as isize) >
                                           0 as libc::c_int {
                                        ret =
                                            k5_json_number_create(*(*req).ktype.offset(i
                                                                                           as
                                                                                           isize)
                                                                      as
                                                                      libc::c_longlong,
                                                                  &mut num);
                                        if ret != 0 {
                                            current_block =
                                                9281771316357078290;
                                            break ;
                                        }
                                        ret =
                                            k5_json_array_add(arr,
                                                              num as
                                                                  k5_json_value);
                                        k5_json_release(num as k5_json_value);
                                        if ret != 0 {
                                            current_block =
                                                9281771316357078290;
                                            break ;
                                        }
                                    }
                                    i += 1
                                }
                                match current_block {
                                    9281771316357078290 => { }
                                    _ => {
                                        ret =
                                            k5_json_object_set(obj,
                                                               b"req.avail_etypes\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               arr as
                                                                   k5_json_value);
                                        if !(ret != 0) {
                                            /* Pre-auth types. */
                                            if ev_success ==
                                                   1 as libc::c_int as
                                                       libc::c_uint &&
                                                   !(*req).padata.is_null() {
                                                ret =
                                                    k5_json_array_create(&mut arrpa);
                                                if ret != 0 {
                                                    current_block =
                                                        9281771316357078290;
                                                } else {
                                                    padata = (*req).padata;
                                                    loop  {
                                                        if (*padata).is_null()
                                                           {
                                                            current_block =
                                                                10758786907990354186;
                                                            break ;
                                                        }
                                                        if strlen(map_patype((**padata).pa_type))
                                                               >
                                                               1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong
                                                           {
                                                            ret =
                                                                k5_json_string_create(map_patype((**padata).pa_type),
                                                                                      &mut str);
                                                            if ret != 0 {
                                                                current_block
                                                                    =
                                                                    9281771316357078290;
                                                                break ;
                                                            }
                                                            ret =
                                                                k5_json_array_add(arrpa,
                                                                                  str
                                                                                      as
                                                                                      k5_json_value);
                                                            k5_json_release(str
                                                                                as
                                                                                k5_json_value);
                                                            if ret != 0 {
                                                                current_block
                                                                    =
                                                                    9281771316357078290;
                                                                break ;
                                                            }
                                                        }
                                                        padata =
                                                            padata.offset(1)
                                                    }
                                                    match current_block {
                                                        9281771316357078290 =>
                                                        {
                                                        }
                                                        _ => {
                                                            ret =
                                                                k5_json_object_set(obj,
                                                                                   b"req.pa_type\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const libc::c_char,
                                                                                   arrpa
                                                                                       as
                                                                                       k5_json_value);
                                                            current_block =
                                                                1356832168064818221;
                                                        }
                                                    }
                                                }
                                            } else {
                                                current_block =
                                                    1356832168064818221;
                                            }
                                            match current_block {
                                                9281771316357078290 => { }
                                                _ =>
                                                /* List of requested addresses. */
                                                {
                                                    if !(*req).addresses.is_null()
                                                       {
                                                        ret =
                                                            k5_json_array_create(&mut arra);
                                                        if !(ret != 0) {
                                                            i =
                                                                0 as
                                                                    libc::c_int;
                                                            loop  {
                                                                if (*(*req).addresses.offset(i
                                                                                                 as
                                                                                                 isize)).is_null()
                                                                   {
                                                                    current_block
                                                                        =
                                                                        16415152177862271243;
                                                                    break ;
                                                                }
                                                                ret =
                                                                    k5_json_object_create(&mut tmpa);
                                                                if ret != 0 {
                                                                    current_block
                                                                        =
                                                                        9281771316357078290;
                                                                    break ;
                                                                }
                                                                ret =
                                                                    addr_to_obj(*(*req).addresses.offset(i
                                                                                                             as
                                                                                                             isize),
                                                                                tmpa);
                                                                if ret != 0 {
                                                                    current_block
                                                                        =
                                                                        9281771316357078290;
                                                                    break ;
                                                                }
                                                                ret =
                                                                    k5_json_array_add(arra,
                                                                                      tmpa
                                                                                          as
                                                                                          k5_json_value);
                                                                k5_json_release(tmpa
                                                                                    as
                                                                                    k5_json_value);
                                                                if ret != 0 {
                                                                    current_block
                                                                        =
                                                                        9281771316357078290;
                                                                    break ;
                                                                }
                                                                i += 1
                                                            }
                                                            match current_block
                                                                {
                                                                9281771316357078290
                                                                => {
                                                                }
                                                                _ => {
                                                                    ret =
                                                                        k5_json_object_set(obj,
                                                                                           b"req.addresses\x00"
                                                                                               as
                                                                                               *const u8
                                                                                               as
                                                                                               *const libc::c_char,
                                                                                           arra
                                                                                               as
                                                                                               k5_json_value);
                                                                    (ret) !=
                                                                        0;
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
                        }
                    }
                }
            }
        }
    }
    k5_json_release(arr as k5_json_value);
    k5_json_release(arra as k5_json_value);
    k5_json_release(arrpa as k5_json_value);
    return ret;
}
/*
 * Helper for JSON encoding of krb5_kdc_rep.
 * Returns 0 on success.
 */
#[c2rust::src_loc = "773:1"]
unsafe extern "C" fn rep_to_value(mut rep: *mut krb5_kdc_rep,
                                  ev_success: krb5_boolean,
                                  mut obj: k5_json_object)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut padata: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut arrpa: k5_json_array = 0 as k5_json_array;
    let mut str: k5_json_string = 0 as k5_json_string;
    if rep.is_null() { return 0 as libc::c_int }
    if ev_success == 1 as libc::c_int as libc::c_uint {
        ret =
            tkt_to_value((*rep).ticket, obj,
                         b"rep.ticket\x00" as *const u8 as
                             *const libc::c_char);
        /* Enctype of the reply-encrypting key. */
        ret =
            int32_to_value((*rep).enc_part.enctype, obj,
                           b"rep_etype\x00" as *const u8 as
                               *const libc::c_char);
        (ret) != 0;
    } else {
        if !(*rep).padata.is_null() {
            ret = k5_json_array_create(&mut arrpa);
            if ret != 0 {
                current_block = 9995863188919453887;
            } else {
                padata = (*rep).padata;
                loop  {
                    if (*padata).is_null() {
                        current_block = 224731115979188411;
                        break ;
                    }
                    if strlen(map_patype((**padata).pa_type)) >
                           1 as libc::c_int as libc::c_ulong {
                        ret =
                            k5_json_string_create(map_patype((**padata).pa_type),
                                                  &mut str);
                        if ret != 0 {
                            current_block = 9995863188919453887;
                            break ;
                        }
                        ret = k5_json_array_add(arrpa, str as k5_json_value);
                        k5_json_release(str as k5_json_value);
                        if ret != 0 {
                            current_block = 9995863188919453887;
                            break ;
                        }
                    }
                    padata = padata.offset(1)
                }
            }
        } else { current_block = 224731115979188411; }
        match current_block {
            9995863188919453887 => { }
            _ => {
                ret =
                    k5_json_object_set(obj,
                                       b"rep.pa_type\x00" as *const u8 as
                                           *const libc::c_char,
                                       arrpa as k5_json_value)
            }
        }
    }
    k5_json_release(arrpa as k5_json_value);
    return ret;
}
/*
 * Converts krb5_ticket into a property of a JSON object.
 * Returns 0 on success.
 */
#[c2rust::src_loc = "821:1"]
unsafe extern "C" fn tkt_to_value(mut tkt: *mut krb5_ticket,
                                  mut obj: k5_json_object,
                                  mut key: *const libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut tmp: k5_json_object = 0 as k5_json_object;
    let mut part2: *mut krb5_enc_tkt_part = 0 as *mut krb5_enc_tkt_part;
    if tkt.is_null() { return 0 as libc::c_int }
    /* Main object. */
    if k5_json_object_create(&mut tmp) != 0 { return 12 as libc::c_int }
    /*
     * CNAME - potentially redundant data...
     * ...but it is part of the ticket. So, record it as such.
     */
    ret =
        princ_to_value((*tkt).server, tmp,
                       b"cname\x00" as *const u8 as *const libc::c_char);
    if !(ret != 0) {
        ret =
            princ_to_value((*tkt).server, tmp,
                           b"sname\x00" as *const u8 as *const libc::c_char);
        if !(ret != 0) {
            /* Enctype of a long-term key of service. */
            if (*tkt).enc_part.enctype != 0 {
                ret =
                    int32_to_value((*tkt).enc_part.enctype, tmp,
                                   b"srv_etype\x00" as *const u8 as
                                       *const libc::c_char)
            } /* part2 != NULL */
            if !(ret != 0) {
                if !(*tkt).enc_part2.is_null() { part2 = (*tkt).enc_part2 }
                if !part2.is_null() {
                    ret =
                        princ_to_value((*part2).client, tmp,
                                       b"cname\x00" as *const u8 as
                                           *const libc::c_char);
                    if ret != 0 {
                        current_block = 5152980791761092907;
                    } else {
                        ret =
                            int32_to_value((*part2).flags, tmp,
                                           b"flags\x00" as *const u8 as
                                               *const libc::c_char);
                        if ret != 0 {
                            current_block = 5152980791761092907;
                        } else {
                            /* Chosen by KDC session key enctype (short-term key). */
                            ret =
                                int32_to_value((*(*part2).session).enctype,
                                               tmp,
                                               b"sess_etype\x00" as *const u8
                                                   as *const libc::c_char);
                            if ret != 0 {
                                current_block = 5152980791761092907;
                            } else {
                                ret =
                                    int32_to_value((*part2).times.starttime,
                                                   tmp,
                                                   b"start\x00" as *const u8
                                                       as
                                                       *const libc::c_char);
                                if ret != 0 {
                                    current_block = 5152980791761092907;
                                } else {
                                    ret =
                                        int32_to_value((*part2).times.endtime,
                                                       tmp,
                                                       b"end\x00" as *const u8
                                                           as
                                                           *const libc::c_char);
                                    if ret != 0 {
                                        current_block = 5152980791761092907;
                                    } else {
                                        ret =
                                            int32_to_value((*part2).times.renew_till,
                                                           tmp,
                                                           b"renew_till\x00"
                                                               as *const u8 as
                                                               *const libc::c_char);
                                        if ret != 0 {
                                            current_block =
                                                5152980791761092907;
                                        } else {
                                            ret =
                                                int32_to_value((*part2).times.authtime,
                                                               tmp,
                                                               b"authtime\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
                                            if ret != 0 {
                                                current_block =
                                                    5152980791761092907;
                                            } else if (*part2).transited.tr_contents.length
                                                          >
                                                          0 as libc::c_int as
                                                              libc::c_uint {
                                                ret =
                                                    data_to_value(&mut (*part2).transited.tr_contents,
                                                                  tmp,
                                                                  b"tr_contents\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char);
                                                if ret != 0 {
                                                    current_block =
                                                        5152980791761092907;
                                                } else {
                                                    current_block =
                                                        1538046216550696469;
                                                }
                                            } else {
                                                current_block =
                                                    1538046216550696469;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else { current_block = 1538046216550696469; }
                match current_block {
                    5152980791761092907 => { }
                    _ => {
                        if ret == 0 {
                            ret =
                                k5_json_object_set(obj, key,
                                                   tmp as k5_json_value)
                        }
                    }
                }
            }
        }
    }
    k5_json_release(tmp as k5_json_value);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "897:21"]
pub static mut patype_str: [_patype_str; 19] =
    [{
         let mut init =
             _patype_str{id: 2 as libc::c_int,
                         name:
                             b"ENC_TIMESTAMP\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 3 as libc::c_int,
                         name:
                             b"PW_SALT\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 5 as libc::c_int,
                         name:
                             b"ENC_UNIX_TIME\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 12 as libc::c_int,
                         name:
                             b"SAM_CHALLENGE\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 13 as libc::c_int,
                         name:
                             b"SAM_RESPONSE\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 14 as libc::c_int,
                         name:
                             b"PK_AS_REQ_OLD\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 15 as libc::c_int,
                         name:
                             b"PK_AS_REP_OLD\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 16 as libc::c_int,
                         name:
                             b"PK_AS_REQ\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 17 as libc::c_int,
                         name:
                             b"PK_AS_REP\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 19 as libc::c_int,
                         name:
                             b"ETYPE_INFO2\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 30 as libc::c_int,
                         name:
                             b"SAM_CHALLENGE_2\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 31 as libc::c_int,
                         name:
                             b"SAM_RESPONSE_2\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 128 as libc::c_int,
                         name:
                             b"PAC_REQUEST\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 129 as libc::c_int,
                         name:
                             b"FOR_USER\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 130 as libc::c_int,
                         name:
                             b"S4U_X509_USER\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 138 as libc::c_int,
                         name:
                             b"ENCRYPTED_CHALLENGE\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 141 as libc::c_int,
                         name:
                             b"OTP_CHALLENGE\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 142 as libc::c_int,
                         name:
                             b"OTP_REQUEST\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _patype_str{id: 144 as libc::c_int,
                         name:
                             b"OTP_PIN_CHANGE\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     }];
#[c2rust::src_loc = "920:1"]
unsafe extern "C" fn map_patype(mut pa_type: krb5_preauthtype)
 -> *mut libc::c_char {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int =
        (::std::mem::size_of::<[_patype_str; 19]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<_patype_str>()
                                             as libc::c_ulong) as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if pa_type == patype_str[i as usize].id {
            return patype_str[i as usize].name
        }
        i += 1
    }
    return b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
