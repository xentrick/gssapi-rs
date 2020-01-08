#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub tqe_next: *mut crate::gssapiP_negoex_h::negoex_auth_mech,
    pub tqe_prev: *mut *mut crate::gssapiP_negoex_h::negoex_auth_mech,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_2 {
    pub n: crate::gssapiP_negoex_h::nego_message,
    pub e: crate::gssapiP_negoex_h::exchange_message,
    pub v: crate::gssapiP_negoex_h::verify_message,
    pub a: crate::gssapiP_negoex_h::alert_message,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */

/*
 * Copyright (C) 2011-2018 PADL Software Pty Ltd.
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
 * { iso(1) identified-organization(3) dod(6) internet(1) private(4)
 *   enterprise(1) microsoft (311) security(2) mechanisms(2) negoex(30) }
 */
pub type conversation_id = [crate::stdlib::uint8_t; 16];
/* NEGO_MESSAGE */

/* NEGO_MESSAGE */

/* EXCHANGE_MESSAGE */

/* EXCHANGE_MESSAGE */

/* EXCHANGE_MESSAGE */

/* EXCHANGE_MESSAGE */

/* VERIFY_MESSAGE */

/* ALERT */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct negoex_auth_mech {
    pub links: crate::gssapiP_negoex_h::C2RustUnnamed_1,
    pub oid: crate::gssapi_h::gss_OID,
    pub scheme: crate::gssapiP_negoex_h::auth_scheme,
    pub mech_context: crate::gssapi_h::gss_ctx_id_t,
    pub metadata: crate::gssapi_h::gss_buffer_desc,
    pub key: crate::krb5_h::krb5_keyblock,
    pub verify_key: crate::krb5_h::krb5_keyblock,
    pub complete: i32,
    pub sent_checksum: i32,
    pub verified_checksum: i32,
}
pub type auth_scheme = [crate::stdlib::uint8_t; 16];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nego_message {
    pub random: [crate::stdlib::uint8_t; 32],
    pub schemes: *const crate::stdlib::uint8_t,
    pub nschemes: crate::stdlib::uint16_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exchange_message {
    pub scheme: crate::gssapiP_negoex_h::auth_scheme,
    pub token: crate::gssapi_h::gss_buffer_desc,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct verify_message {
    pub scheme: crate::gssapiP_negoex_h::auth_scheme,
    pub cksum_type: crate::stdlib::uint32_t,
    pub cksum: *const crate::stdlib::uint8_t,
    pub cksum_len: crate::stddef_h::size_t,
    pub offset_in_token: crate::stddef_h::size_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct negoex_message {
    pub type_0: crate::stdlib::uint32_t,
    pub u: crate::gssapiP_negoex_h::C2RustUnnamed_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alert_message {
    pub scheme: crate::gssapiP_negoex_h::auth_scheme,
    pub verify_no_key: i32,
}
pub type message_type = u32;
pub const ALERT: crate::gssapiP_negoex_h::message_type = 7;
pub const VERIFY: crate::gssapiP_negoex_h::message_type = 6;
pub const AP_REQUEST: crate::gssapiP_negoex_h::message_type = 5;
pub const CHALLENGE: crate::gssapiP_negoex_h::message_type = 4;
pub const ACCEPTOR_META_DATA: crate::gssapiP_negoex_h::message_type = 3;
pub const INITIATOR_META_DATA: crate::gssapiP_negoex_h::message_type = 2;
pub const ACCEPTOR_NEGO: crate::gssapiP_negoex_h::message_type = 1;
pub const INITIATOR_NEGO: crate::gssapiP_negoex_h::message_type = 0;
