use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:53"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:53"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:53"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:53"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:53"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:53"]
pub mod krb5_h {
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
    #[c2rust::src_loc = "180:1"]
    pub type krb5_cksumtype = krb5_int32;
    #[c2rust::src_loc = "181:1"]
    pub type krb5_authdatatype = krb5_int32;
    #[c2rust::src_loc = "185:1"]
    pub type krb5_preauthtype = krb5_int32;
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
    #[c2rust::src_loc = "215:16"]
    pub struct _krb5_octet_data {
        pub magic: krb5_magic,
        pub length: libc::c_uint,
        pub data: *mut krb5_octet,
    }
    #[c2rust::src_loc = "215:1"]
    pub type krb5_octet_data = _krb5_octet_data;
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
    #[c2rust::src_loc = "323:1"]
    pub type krb5_address = _krb5_address;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "391:16"]
    pub struct _krb5_checksum {
        pub magic: krb5_magic,
        pub checksum_type: krb5_cksumtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "391:1"]
    pub type krb5_checksum = _krb5_checksum;
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
    #[c2rust::src_loc = "1979:1"]
    pub type krb5_ticket = _krb5_ticket;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1993:16"]
    pub struct _krb5_authenticator {
        pub magic: krb5_magic,
        pub client: krb5_principal,
        pub checksum: *mut krb5_checksum,
        pub cusec: krb5_int32,
        pub ctime: krb5_timestamp,
        pub subkey: *mut krb5_keyblock,
        pub seq_number: krb5_ui_4,
        pub authorization_data: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "1993:1"]
    pub type krb5_authenticator = _krb5_authenticator;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2005:16"]
    pub struct _krb5_tkt_authent {
        pub magic: krb5_magic,
        pub ticket: *mut krb5_ticket,
        pub authenticator: *mut krb5_authenticator,
        pub ap_options: krb5_flags,
    }
    #[c2rust::src_loc = "2005:1"]
    pub type krb5_tkt_authent = _krb5_tkt_authent;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2013:16"]
    pub struct _krb5_creds {
        pub magic: krb5_magic,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub keyblock: krb5_keyblock,
        pub times: krb5_ticket_times,
        pub is_skey: krb5_boolean,
        pub ticket_flags: krb5_flags,
        pub addresses: *mut *mut krb5_address,
        pub ticket: krb5_data,
        pub second_ticket: krb5_data,
        pub authdata: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "2013:1"]
    pub type krb5_creds = _krb5_creds;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2031:16"]
    pub struct _krb5_last_req_entry {
        pub magic: krb5_magic,
        pub lr_type: krb5_int32,
        pub value: krb5_timestamp,
    }
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
    #[c2rust::src_loc = "2095:1"]
    pub type krb5_kdc_rep = _krb5_kdc_rep;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2107:16"]
    pub struct _krb5_error {
        pub magic: krb5_magic,
        pub ctime: krb5_timestamp,
        pub cusec: krb5_int32,
        pub susec: krb5_int32,
        pub stime: krb5_timestamp,
        pub error: krb5_ui_4,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub text: krb5_data,
        pub e_data: krb5_data,
    }
    #[c2rust::src_loc = "2107:1"]
    pub type krb5_error = _krb5_error;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2122:16"]
    pub struct _krb5_ap_req {
        pub magic: krb5_magic,
        pub ap_options: krb5_flags,
        pub ticket: *mut krb5_ticket,
        pub authenticator: krb5_enc_data,
    }
    #[c2rust::src_loc = "2122:1"]
    pub type krb5_ap_req = _krb5_ap_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2134:16"]
    pub struct _krb5_ap_rep {
        pub magic: krb5_magic,
        pub enc_part: krb5_enc_data,
    }
    #[c2rust::src_loc = "2134:1"]
    pub type krb5_ap_rep = _krb5_ap_rep;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2140:16"]
    pub struct _krb5_ap_rep_enc_part {
        pub magic: krb5_magic,
        pub ctime: krb5_timestamp,
        pub cusec: krb5_int32,
        pub subkey: *mut krb5_keyblock,
        pub seq_number: krb5_ui_4,
    }
    #[c2rust::src_loc = "2140:1"]
    pub type krb5_ap_rep_enc_part = _krb5_ap_rep_enc_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2158:16"]
    pub struct _krb5_cred_info {
        pub magic: krb5_magic,
        pub session: *mut krb5_keyblock,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub flags: krb5_flags,
        pub times: krb5_ticket_times,
        pub caddrs: *mut *mut krb5_address,
    }
    #[c2rust::src_loc = "2158:1"]
    pub type krb5_cred_info = _krb5_cred_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2169:16"]
    pub struct _krb5_cred_enc_part {
        pub magic: krb5_magic,
        pub nonce: krb5_int32,
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub s_address: *mut krb5_address,
        pub r_address: *mut krb5_address,
        pub ticket_info: *mut *mut krb5_cred_info,
    }
    #[c2rust::src_loc = "2169:1"]
    pub type krb5_cred_enc_part = _krb5_cred_enc_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2180:16"]
    pub struct _krb5_cred {
        pub magic: krb5_magic,
        pub tickets: *mut *mut krb5_ticket,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_cred_enc_part,
    }
    #[c2rust::src_loc = "2180:1"]
    pub type krb5_cred = _krb5_cred;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2216:16"]
    pub struct _krb5_pa_pac_req {
        pub include_pac: krb5_boolean,
    }
    #[c2rust::src_loc = "2216:1"]
    pub type krb5_pa_pac_req = _krb5_pa_pac_req;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        #[no_mangle]
        #[c2rust::src_loc = "4633:1"]
        pub fn krb5_free_authdata(context: krb5_context,
                                  val: *mut *mut krb5_authdata);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:53"]
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
    /* check logon hour restrictions */
    /* sign with usage 27 instead of 26 */
    #[c2rust::src_loc = "767:1"]
    pub type krb5_pa_s4u_x509_user = _krb5_pa_s4u_x509_user;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "767:16"]
    pub struct _krb5_pa_s4u_x509_user {
        pub user_id: krb5_s4u_userid,
        pub cksum: krb5_checksum,
    }
    #[c2rust::src_loc = "757:1"]
    pub type krb5_s4u_userid = _krb5_s4u_userid;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "757:16"]
    pub struct _krb5_s4u_userid {
        pub nonce: krb5_int32,
        pub user: krb5_principal,
        pub subject_cert: krb5_data,
        pub options: krb5_flags,
    }
    #[c2rust::src_loc = "780:1"]
    pub type krb5_fast_armored_req = _krb5_fast_armored_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "780:16"]
    pub struct _krb5_fast_armored_req {
        pub magic: krb5_magic,
        pub armor: *mut krb5_fast_armor,
        pub req_checksum: krb5_checksum,
        pub enc_part: krb5_enc_data,
    }
    #[c2rust::src_loc = "776:1"]
    pub type krb5_fast_armor = _krb5_fast_armor;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "776:16"]
    pub struct _krb5_fast_armor {
        pub armor_type: krb5_int32,
        pub armor_value: krb5_data,
    }
    #[c2rust::src_loc = "843:1"]
    pub type krb5_verifier_mac = _krb5_verifier_mac;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "843:16"]
    pub struct _krb5_verifier_mac {
        pub princ: krb5_principal,
        pub kvno: krb5_kvno,
        pub enctype: krb5_enctype,
        pub checksum: krb5_checksum,
    }
    #[c2rust::src_loc = "535:1"]
    pub type krb5_pa_otp_req = _krb5_pa_otp_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "535:16"]
    pub struct _krb5_pa_otp_req {
        pub flags: krb5_int32,
        pub nonce: krb5_data,
        pub enc_data: krb5_enc_data,
        pub hash_alg: *mut krb5_algorithm_identifier,
        pub iteration_count: krb5_int32,
        pub otp_value: krb5_data,
        pub pin: krb5_data,
        pub challenge: krb5_data,
        pub time: krb5_timestamp,
        pub counter: krb5_data,
        pub format: krb5_int32,
        pub token_id: krb5_data,
        pub alg_id: krb5_data,
        pub vendor: krb5_data,
    }
    #[c2rust::src_loc = "751:1"]
    pub type krb5_pa_for_user = _krb5_pa_for_user;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "751:16"]
    pub struct _krb5_pa_for_user {
        pub user: krb5_principal,
        pub cksum: krb5_checksum,
        pub auth_package: krb5_data,
    }
    #[c2rust::src_loc = "839:1"]
    pub type krb5_iakerb_finished = _krb5_iakerb_finished;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "839:16"]
    pub struct _krb5_iakerb_finished {
        pub checksum: krb5_checksum,
    }
    /* -1 for unspecified */
    /* -1 for unspecified */
    /*
 * Keep the pkinit definitions in a separate file so that the plugin
 * only has to include k5-int-pkinit.h rather than k5-int.h
 */
    #[c2rust::src_loc = "515:1"]
    pub type krb5_otp_tokeninfo = _krb5_otp_tokeninfo;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "515:16"]
    pub struct _krb5_otp_tokeninfo {
        pub flags: krb5_flags,
        pub vendor: krb5_data,
        pub challenge: krb5_data,
        pub length: krb5_int32,
        pub format: krb5_int32,
        pub token_id: krb5_data,
        pub alg_id: krb5_data,
        pub supported_hash_alg: *mut *mut krb5_algorithm_identifier,
        pub iteration_count: krb5_int32,
    }
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /*
 * AD-CAMMAC's other-verifiers field is a sequence of Verifier, which is an
 * extensible choice with only one selection, Verifier-MAC.  For the time being
 * we will represent this field directly as an array of krb5_verifier_mac.
 * That will have to change if other selections are added.
 */
    #[c2rust::src_loc = "856:1"]
    pub type krb5_cammac = _krb5_cammac;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "856:16"]
    pub struct _krb5_cammac {
        pub elements: *mut *mut krb5_authdata,
        pub kdc_verifier: *mut krb5_verifier_mac,
        pub svc_verifier: *mut krb5_verifier_mac,
        pub other_verifiers: *mut *mut krb5_verifier_mac,
    }
    #[c2rust::src_loc = "813:1"]
    pub type krb5_ad_kdcissued = _krb5_ad_kdcissued;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "813:16"]
    pub struct _krb5_ad_kdcissued {
        pub ad_checksum: krb5_checksum,
        pub i_principal: krb5_principal,
        pub elements: *mut *mut krb5_authdata,
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
    #[c2rust::src_loc = "420:1"]
    pub type krb5_etype_info_entry = _krb5_etype_info_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "420:16"]
    pub struct _krb5_etype_info_entry {
        pub magic: krb5_magic,
        pub etype: krb5_enctype,
        pub length: libc::c_uint,
        pub salt: *mut krb5_octet,
        pub s2kparams: krb5_data,
    }
    #[c2rust::src_loc = "827:1"]
    pub type krb5_ad_signedpath = _krb5_ad_signedpath;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "827:16"]
    pub struct _krb5_ad_signedpath {
        pub enctype: krb5_enctype,
        pub checksum: krb5_checksum,
        pub delegated: *mut krb5_principal,
        pub method_data: *mut *mut krb5_pa_data,
    }
    #[c2rust::src_loc = "1293:1"]
    pub type krb5_priv = _krb5_priv;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1293:16"]
    pub struct _krb5_priv {
        pub magic: krb5_magic,
        pub enc_part: krb5_enc_data,
    }
    /* encrypted part */
    /*
 *  This is essentially -1 without sign extension which can screw up
 *  comparisons on 64 bit machines. If the length is this value, then
 *  the salt data is not present. This is to distinguish between not
 *  being set and being of 0 length.
 */
    #[c2rust::src_loc = "436:1"]
    pub type krb5_etype_info = *mut *mut krb5_etype_info_entry;
    #[c2rust::src_loc = "552:1"]
    pub type krb5_kkdcp_message = _krb5_kkdcp_message;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "552:16"]
    pub struct _krb5_kkdcp_message {
        pub kerb_message: krb5_data,
        pub target_domain: krb5_data,
        pub dclocator_hint: krb5_int32,
    }
    #[c2rust::src_loc = "1298:1"]
    pub type krb5_priv_enc_part = _krb5_priv_enc_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1298:16"]
    pub struct _krb5_priv_enc_part {
        pub magic: krb5_magic,
        pub user_data: krb5_data,
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub seq_number: krb5_ui_4,
        pub s_address: *mut krb5_address,
        pub r_address: *mut krb5_address,
    }
    #[c2rust::src_loc = "787:1"]
    pub type krb5_fast_req = _krb5_fast_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "787:16"]
    pub struct _krb5_fast_req {
        pub magic: krb5_magic,
        pub fast_options: krb5_flags,
        pub req_body: *mut krb5_kdc_req,
    }
    /* user data */
    /* client time, optional */
    /* microsecond portion of time, opt. */
    /* sequence #, optional */
    /* sender address */
    /* recipient address, optional */
    /* padata from req_body is used*/
    /* could be used in a table to find an etype and initialize a block */
    /* internal message representations */
    #[c2rust::src_loc = "1281:1"]
    pub type krb5_safe = _krb5_safe;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1281:16"]
    pub struct _krb5_safe {
        pub magic: krb5_magic,
        pub user_data: krb5_data,
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub seq_number: krb5_ui_4,
        pub s_address: *mut krb5_address,
        pub r_address: *mut krb5_address,
        pub checksum: *mut krb5_checksum,
    }
    #[c2rust::src_loc = "527:1"]
    pub type krb5_pa_otp_challenge = _krb5_pa_otp_challenge;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "527:16"]
    pub struct _krb5_pa_otp_challenge {
        pub nonce: krb5_data,
        pub service: krb5_data,
        pub tokeninfo: *mut *mut krb5_otp_tokeninfo,
        pub salt: krb5_data,
        pub s2kparams: krb5_data,
    }
    #[c2rust::src_loc = "834:1"]
    pub type krb5_iakerb_header = _krb5_iakerb_header;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "834:16"]
    pub struct _krb5_iakerb_header {
        pub target_realm: krb5_data,
        pub cookie: *mut krb5_data,
    }
    /* user data */
    /* client time, optional */
    /* microsecond portion of time,
                                           optional */
    /* sequence #, optional */
    /* sender address */
    /* recipient address, optional */
    /* data integrity checksum */
    /* Plain text of an encrypted PA-FX-COOKIE value produced by the KDC. */
    #[c2rust::src_loc = "559:1"]
    pub type krb5_secure_cookie = _krb5_secure_cookie;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "559:16"]
    pub struct _krb5_secure_cookie {
        pub time: time_t,
        pub data: *mut *mut krb5_pa_data,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "460:16"]
    pub struct _krb5_sam_challenge_2 {
        pub sam_challenge_2_body: krb5_data,
        pub sam_cksum: *mut *mut krb5_checksum,
    }
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
    #[c2rust::src_loc = "460:1"]
    pub type krb5_sam_challenge_2 = _krb5_sam_challenge_2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "465:16"]
    pub struct _krb5_sam_challenge_2_body {
        pub magic: krb5_magic,
        pub sam_type: krb5_int32,
        pub sam_flags: krb5_flags,
        pub sam_type_name: krb5_data,
        pub sam_track_id: krb5_data,
        pub sam_challenge_label: krb5_data,
        pub sam_challenge: krb5_data,
        pub sam_response_prompt: krb5_data,
        pub sam_pk_for_sad: krb5_data,
        pub sam_nonce: krb5_int32,
        pub sam_etype: krb5_enctype,
    }
    #[c2rust::src_loc = "465:1"]
    pub type krb5_sam_challenge_2_body = _krb5_sam_challenge_2_body;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "479:16"]
    pub struct _krb5_sam_response_2 {
        pub magic: krb5_magic,
        pub sam_type: krb5_int32,
        pub sam_flags: krb5_flags,
        pub sam_track_id: krb5_data,
        pub sam_enc_nonce_or_sad: krb5_enc_data,
        pub sam_nonce: krb5_int32,
    }
    #[c2rust::src_loc = "479:1"]
    pub type krb5_sam_response_2 = _krb5_sam_response_2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "488:16"]
    pub struct _krb5_enc_sam_response_enc_2 {
        pub magic: krb5_magic,
        pub sam_nonce: krb5_int32,
        pub sam_sad: krb5_data,
    }
    #[c2rust::src_loc = "488:1"]
    pub type krb5_enc_sam_response_enc_2 = _krb5_enc_sam_response_enc_2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "746:16"]
    pub struct _krb5_pa_enc_ts {
        pub patimestamp: krb5_timestamp,
        pub pausec: krb5_int32,
    }
    /* Array of checksums */
    /* information */
    /* KRB5_SAM_* values */
    /* informational */
    /* KRB5_SAM_* values */
    /* copied */
    /* krb5_enc_sam_response_enc */
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
    #[c2rust::src_loc = "746:1"]
    pub type krb5_pa_enc_ts = _krb5_pa_enc_ts;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "798:16"]
    pub struct _krb5_fast_finished {
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub client: krb5_principal,
        pub ticket_checksum: krb5_checksum,
    }
    /* Bits 0-15 are critical in FAST options (RFC 6113 section 7.3). */
    #[c2rust::src_loc = "798:1"]
    pub type krb5_fast_finished = _krb5_fast_finished;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "805:16"]
    pub struct _krb5_fast_response {
        pub magic: krb5_magic,
        pub padata: *mut *mut krb5_pa_data,
        pub strengthen_key: *mut krb5_keyblock,
        pub finished: *mut krb5_fast_finished,
        pub nonce: krb5_int32,
    }
    #[c2rust::src_loc = "805:1"]
    pub type krb5_fast_response = _krb5_fast_response;
    /* Convenience function: zap and free ptr if it is non-NULL. */
    #[inline]
    #[c2rust::src_loc = "656:1"]
    pub unsafe extern "C" fn zapfree(mut ptr: *mut libc::c_void,
                                     mut len: size_t) {
        if !ptr.is_null() { explicit_bzero(ptr, len); free(ptr); };
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_checksum, krb5_principal, krb5_data,
                        krb5_enc_data, krb5_kvno, krb5_timestamp,
                        krb5_authdata, krb5_octet, krb5_pa_data, krb5_ui_4,
                        krb5_address, krb5_kdc_req, krb5_keyblock,
                        krb5_context};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::k5_int_pkinit_h::krb5_algorithm_identifier;
    use super::time_t_h::time_t;
    use super::stddef_h::size_t;
    use super::string_h::explicit_bzero;
    use super::stdlib_h::free;
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
        #[no_mangle]
        #[c2rust::src_loc = "685:1"]
        pub fn krb5int_c_free_keyblock(_: krb5_context,
                                       key: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "686:1"]
        pub fn krb5int_c_free_keyblock_contents(_: krb5_context,
                                                _: *mut krb5_keyblock);
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:53"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:53"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:53"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int-pkinit.h:53"]
pub mod k5_int_pkinit_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * COPYRIGHT (C) 2006
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
    /*
 * pkinit structures
 */
    /* PKAuthenticator */
    /* (0..999999) */
    /* (0..4294967295) */
    /* AlgorithmIdentifier */
    #[c2rust::src_loc = "49:1"]
    pub type krb5_algorithm_identifier = _krb5_algorithm_identifier;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:16"]
    pub struct _krb5_algorithm_identifier {
        pub algorithm: krb5_data,
        pub parameters: krb5_data,
    }
    use super::krb5_h::krb5_data;
    /* OID */
    /* Optional */
    /* _KRB5_INT_PKINIT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-spake.h:54"]
pub mod k5_spake_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-spake.h - SPAKE preauth mech declarations */
/*
 * Copyright (C) 2015 by the Massachusetts Institute of Technology.
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
 * The SPAKE preauth mechanism allows long-term client keys to be used for
 * preauthentication without exposing them to offline dictionary attacks.  The
 * negotiated key can also be used for second-factor authentication.  This
 * header file declares structures and encoder/decoder functions for the
 * mechanism's padata messages.
 */
    /* SPAKESecondFactor is contained within a SPAKEChallenge, SPAKEResponse, or
 * EncryptedData message and contains a second-factor challenge or response. */
    #[c2rust::src_loc = "48:1"]
    pub type krb5_spake_factor = krb5_spake_factor_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "48:16"]
    pub struct krb5_spake_factor_st {
        pub type_0: int32_t,
        pub data: *mut krb5_data,
    }
    /* PA-SPAKE is a choice among the message types which can appear in a PA-SPAKE
 * padata element. */
    #[c2rust::src_loc = "85:1"]
    pub type krb5_pa_spake = krb5_pa_spake_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:16"]
    pub struct krb5_pa_spake_st {
        pub choice: krb5_spake_msgtype,
        pub u: krb5_spake_message_choices,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "87:11"]
    pub union krb5_spake_message_choices {
        pub support: krb5_spake_support,
        pub challenge: krb5_spake_challenge,
        pub response: krb5_spake_response,
        pub encdata: krb5_enc_data,
    }
    /* SPAKEResponse is sent from the client to the KDC to communicate its public
 * value and encrypted second-factor response. */
    #[c2rust::src_loc = "70:1"]
    pub type krb5_spake_response = krb5_spake_response_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:16"]
    pub struct krb5_spake_response_st {
        pub pubkey: krb5_data,
        pub factor: krb5_enc_data,
    }
    /* SPAKEChallenge is sent from the KDC to the client to communicate its group
 * selection, public value, and second-factor challenge options. */
    #[c2rust::src_loc = "62:1"]
    pub type krb5_spake_challenge = krb5_spake_challenge_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:16"]
    pub struct krb5_spake_challenge_st {
        pub group: int32_t,
        pub pubkey: krb5_data,
        pub factors: *mut *mut krb5_spake_factor,
    }
    /* SPAKESupport is sent from the client to the KDC to indicate which group the
 * client supports. */
    #[c2rust::src_loc = "55:1"]
    pub type krb5_spake_support = krb5_spake_support_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:16"]
    pub struct krb5_spake_support_st {
        pub ngroups: int32_t,
        pub groups: *mut int32_t,
    }
    #[c2rust::src_loc = "75:1"]
    pub type krb5_spake_msgtype = libc::c_int;
    #[c2rust::src_loc = "80:5"]
    pub const SPAKE_MSGTYPE_ENCDATA: krb5_spake_msgtype = 3;
    #[c2rust::src_loc = "79:5"]
    pub const SPAKE_MSGTYPE_RESPONSE: krb5_spake_msgtype = 2;
    #[c2rust::src_loc = "78:5"]
    pub const SPAKE_MSGTYPE_CHALLENGE: krb5_spake_msgtype = 1;
    #[c2rust::src_loc = "77:5"]
    pub const SPAKE_MSGTYPE_SUPPORT: krb5_spake_msgtype = 0;
    #[c2rust::src_loc = "76:5"]
    pub const SPAKE_MSGTYPE_UNKNOWN: krb5_spake_msgtype = -1;
    use super::stdint_intn_h::int32_t;
    use super::krb5_h::{krb5_data, krb5_enc_data};
    /* K5_SPAKE_H */
}
#[c2rust::header_src = "/usr/include/string.h:53"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:53"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t, __time_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_msgtype, krb5_kvno, krb5_addrtype, krb5_enctype,
                       krb5_cksumtype, krb5_authdatatype, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       _krb5_octet_data, krb5_octet_data, krb5_principal_data,
                       krb5_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       _krb5_checksum, krb5_checksum, _krb5_enc_data,
                       krb5_enc_data, _krb5_ticket_times, krb5_ticket_times,
                       _krb5_authdata, krb5_authdata, _krb5_transited,
                       krb5_transited, _krb5_enc_tkt_part, krb5_enc_tkt_part,
                       _krb5_ticket, krb5_ticket, _krb5_authenticator,
                       krb5_authenticator, _krb5_tkt_authent,
                       krb5_tkt_authent, _krb5_creds, krb5_creds,
                       _krb5_last_req_entry, krb5_last_req_entry,
                       _krb5_pa_data, krb5_pa_data, _krb5_kdc_req,
                       krb5_kdc_req, _krb5_enc_kdc_rep_part,
                       krb5_enc_kdc_rep_part, _krb5_kdc_rep, krb5_kdc_rep,
                       _krb5_error, krb5_error, _krb5_ap_req, krb5_ap_req,
                       _krb5_ap_rep, krb5_ap_rep, _krb5_ap_rep_enc_part,
                       krb5_ap_rep_enc_part, _krb5_cred_info, krb5_cred_info,
                       _krb5_cred_enc_part, krb5_cred_enc_part, _krb5_cred,
                       krb5_cred, _krb5_pa_pac_req, krb5_pa_pac_req,
                       _profile_t, krb5_free_authdata};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_pa_s4u_x509_user,
                         _krb5_pa_s4u_x509_user, krb5_s4u_userid,
                         _krb5_s4u_userid, krb5_fast_armored_req,
                         _krb5_fast_armored_req, krb5_fast_armor,
                         _krb5_fast_armor, krb5_verifier_mac,
                         _krb5_verifier_mac, krb5_pa_otp_req,
                         _krb5_pa_otp_req, krb5_pa_for_user,
                         _krb5_pa_for_user, krb5_iakerb_finished,
                         _krb5_iakerb_finished, krb5_otp_tokeninfo,
                         _krb5_otp_tokeninfo, krb5_cammac, _krb5_cammac,
                         krb5_ad_kdcissued, _krb5_ad_kdcissued,
                         krb5_etype_info_entry, _krb5_etype_info_entry,
                         krb5_ad_signedpath, _krb5_ad_signedpath, krb5_priv,
                         _krb5_priv, krb5_etype_info, krb5_kkdcp_message,
                         _krb5_kkdcp_message, krb5_priv_enc_part,
                         _krb5_priv_enc_part, krb5_fast_req, _krb5_fast_req,
                         krb5_safe, _krb5_safe, krb5_pa_otp_challenge,
                         _krb5_pa_otp_challenge, krb5_iakerb_header,
                         _krb5_iakerb_header, krb5_secure_cookie,
                         _krb5_secure_cookie, _krb5_sam_challenge_2,
                         krb5_sam_challenge_2, _krb5_sam_challenge_2_body,
                         krb5_sam_challenge_2_body, _krb5_sam_response_2,
                         krb5_sam_response_2, _krb5_enc_sam_response_enc_2,
                         krb5_enc_sam_response_enc_2, _krb5_pa_enc_ts,
                         krb5_pa_enc_ts, _krb5_fast_finished,
                         krb5_fast_finished, _krb5_fast_response,
                         krb5_fast_response, zapfree, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5int_c_free_keyblock,
                         krb5int_c_free_keyblock_contents};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::k5_int_pkinit_h::{krb5_algorithm_identifier,
                                _krb5_algorithm_identifier};
pub use self::k5_spake_h::{krb5_spake_factor, krb5_spake_factor_st,
                           krb5_pa_spake, krb5_pa_spake_st,
                           krb5_spake_message_choices, krb5_spake_response,
                           krb5_spake_response_st, krb5_spake_challenge,
                           krb5_spake_challenge_st, krb5_spake_support,
                           krb5_spake_support_st, krb5_spake_msgtype,
                           SPAKE_MSGTYPE_ENCDATA, SPAKE_MSGTYPE_RESPONSE,
                           SPAKE_MSGTYPE_CHALLENGE, SPAKE_MSGTYPE_SUPPORT,
                           SPAKE_MSGTYPE_UNKNOWN};
use self::string_h::explicit_bzero;
use self::stdlib_h::free;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/kfree.c */
/*
 * Copyright 1990-1998, 2009 by the Massachusetts Institute of Technology.
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
 * Copyright (c) 2006-2008, Novell, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright notice,
 *       this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *   * The copyright holder's name is not used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
#[no_mangle]
#[c2rust::src_loc = "57:1"]
pub unsafe extern "C" fn krb5_free_address(mut context: krb5_context,
                                           mut val: *mut krb5_address) {
    if val.is_null() { return }
    free((*val).contents as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "66:1"]
pub unsafe extern "C" fn krb5_free_addresses(mut context: krb5_context,
                                             mut val:
                                                 *mut *mut krb5_address) {
    let mut temp: *mut *mut krb5_address = 0 as *mut *mut krb5_address;
    if val.is_null() { return }
    temp = val;
    while !(*temp).is_null() {
        free((**temp).contents as *mut libc::c_void);
        free(*temp as *mut libc::c_void);
        temp = temp.offset(1)
    }
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "80:1"]
pub unsafe extern "C" fn krb5_free_ap_rep(mut context: krb5_context,
                                          mut val: *mut krb5_ap_rep) {
    if val.is_null() { return }
    free((*val).enc_part.ciphertext.data as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "89:1"]
pub unsafe extern "C" fn krb5_free_ap_req(mut context: krb5_context,
                                          mut val: *mut krb5_ap_req) {
    if val.is_null() { return }
    krb5_free_ticket(context, (*val).ticket);
    free((*val).authenticator.ciphertext.data as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "99:1"]
pub unsafe extern "C" fn krb5_free_ap_rep_enc_part(mut context: krb5_context,
                                                   mut val:
                                                       *mut krb5_ap_rep_enc_part) {
    if val.is_null() { return }
    krb5_free_keyblock(context, (*val).subkey);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "108:1"]
pub unsafe extern "C" fn krb5_free_authenticator_contents(mut context:
                                                              krb5_context,
                                                          mut val:
                                                              *mut krb5_authenticator) {
    if val.is_null() { return }
    krb5_free_checksum(context, (*val).checksum);
    (*val).checksum = 0 as *mut krb5_checksum;
    krb5_free_principal(context, (*val).client);
    (*val).client = 0 as krb5_principal;
    krb5_free_keyblock(context, (*val).subkey);
    (*val).subkey = 0 as *mut krb5_keyblock;
    krb5_free_authdata(context, (*val).authorization_data);
    (*val).authorization_data = 0 as *mut *mut krb5_authdata;
}
#[no_mangle]
#[c2rust::src_loc = "123:1"]
pub unsafe extern "C" fn krb5_free_authenticator(mut context: krb5_context,
                                                 mut val:
                                                     *mut krb5_authenticator) {
    if val.is_null() { return }
    krb5_free_authenticator_contents(context, val);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn krb5_free_checksum(mut context: krb5_context,
                                            mut val: *mut krb5_checksum) {
    if val.is_null() { return }
    krb5_free_checksum_contents(context, val);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "141:1"]
pub unsafe extern "C" fn krb5_free_checksum_contents(mut context:
                                                         krb5_context,
                                                     mut val:
                                                         *mut krb5_checksum) {
    if val.is_null() { return }
    free((*val).contents as *mut libc::c_void);
    (*val).contents = 0 as *mut krb5_octet;
}
#[no_mangle]
#[c2rust::src_loc = "150:1"]
pub unsafe extern "C" fn krb5_free_cred(mut context: krb5_context,
                                        mut val: *mut krb5_cred) {
    if val.is_null() { return }
    krb5_free_tickets(context, (*val).tickets);
    free((*val).enc_part.ciphertext.data as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
/*
 * krb5_free_cred_contents zeros out the session key, and then frees
 * the credentials structures
 */
#[no_mangle]
#[c2rust::src_loc = "165:1"]
pub unsafe extern "C" fn krb5_free_cred_contents(mut context: krb5_context,
                                                 mut val: *mut krb5_creds) {
    if val.is_null() { return }
    krb5_free_principal(context, (*val).client);
    (*val).client = 0 as krb5_principal;
    krb5_free_principal(context, (*val).server);
    (*val).server = 0 as krb5_principal;
    krb5_free_keyblock_contents(context, &mut (*val).keyblock);
    free((*val).ticket.data as *mut libc::c_void);
    (*val).ticket.data = 0 as *mut libc::c_char;
    free((*val).second_ticket.data as *mut libc::c_void);
    (*val).second_ticket.data = 0 as *mut libc::c_char;
    krb5_free_addresses(context, (*val).addresses);
    (*val).addresses = 0 as *mut *mut krb5_address;
    krb5_free_authdata(context, (*val).authdata);
    (*val).authdata = 0 as *mut *mut krb5_authdata;
}
#[no_mangle]
#[c2rust::src_loc = "185:1"]
pub unsafe extern "C" fn krb5_free_cred_enc_part(mut context: krb5_context,
                                                 mut val:
                                                     *mut krb5_cred_enc_part) {
    let mut temp: *mut *mut krb5_cred_info = 0 as *mut *mut krb5_cred_info;
    if val.is_null() { return }
    krb5_free_address(context, (*val).r_address);
    (*val).r_address = 0 as *mut krb5_address;
    krb5_free_address(context, (*val).s_address);
    (*val).s_address = 0 as *mut krb5_address;
    if !(*val).ticket_info.is_null() {
        temp = (*val).ticket_info;
        while !(*temp).is_null() {
            krb5_free_keyblock(context, (**temp).session);
            krb5_free_principal(context, (**temp).client);
            krb5_free_principal(context, (**temp).server);
            krb5_free_addresses(context, (**temp).caddrs);
            free(*temp as *mut libc::c_void);
            temp = temp.offset(1)
        }
        free((*val).ticket_info as *mut libc::c_void);
        (*val).ticket_info = 0 as *mut *mut krb5_cred_info
    };
}
#[no_mangle]
#[c2rust::src_loc = "211:1"]
pub unsafe extern "C" fn krb5_free_creds(mut context: krb5_context,
                                         mut val: *mut krb5_creds) {
    if val.is_null() { return }
    krb5_free_cred_contents(context, val);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "221:1"]
pub unsafe extern "C" fn krb5_free_data(mut context: krb5_context,
                                        mut val: *mut krb5_data) {
    if val.is_null() { return }
    free((*val).data as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "231:1"]
pub unsafe extern "C" fn krb5_free_octet_data(mut context: krb5_context,
                                              mut val: *mut krb5_octet_data) {
    if val.is_null() { return }
    free((*val).data as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "240:1"]
pub unsafe extern "C" fn krb5_free_data_contents(mut context: krb5_context,
                                                 mut val: *mut krb5_data) {
    if val.is_null() { return }
    if !(*val).data.is_null() {
        free((*val).data as *mut libc::c_void);
        (*val).data = 0 as *mut libc::c_char
    };
}
#[no_mangle]
#[c2rust::src_loc = "251:1"]
pub unsafe extern "C" fn krb5_free_enc_data(mut context: krb5_context,
                                            mut val: *mut krb5_enc_data) {
    if val.is_null() { return }
    krb5_free_data_contents(context, &mut (*val).ciphertext);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "260:1"]
pub unsafe extern "C" fn krb5_free_etype_info(mut context: krb5_context,
                                              mut info: krb5_etype_info) {
    let mut i: libc::c_int = 0;
    if info.is_null() { return }
    i = 0 as libc::c_int;
    while !(*info.offset(i as isize)).is_null() {
        free((**info.offset(i as isize)).salt as *mut libc::c_void);
        krb5_free_data_contents(context,
                                &mut (**info.offset(i as isize)).s2kparams);
        free(*info.offset(i as isize) as *mut libc::c_void);
        i += 1
    }
    free(info as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "275:1"]
pub unsafe extern "C" fn krb5_free_enc_kdc_rep_part(mut context: krb5_context,
                                                    mut val:
                                                        *mut krb5_enc_kdc_rep_part) {
    if val.is_null() { return }
    krb5_free_keyblock(context, (*val).session);
    krb5_free_last_req(context, (*val).last_req);
    krb5_free_principal(context, (*val).server);
    krb5_free_addresses(context, (*val).caddrs);
    krb5_free_pa_data(context, (*val).enc_padata);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "288:1"]
pub unsafe extern "C" fn krb5_free_enc_tkt_part(mut context: krb5_context,
                                                mut val:
                                                    *mut krb5_enc_tkt_part) {
    if val.is_null() { return }
    krb5_free_keyblock(context, (*val).session);
    krb5_free_principal(context, (*val).client);
    free((*val).transited.tr_contents.data as *mut libc::c_void);
    krb5_free_addresses(context, (*val).caddrs);
    krb5_free_authdata(context, (*val).authorization_data);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "302:1"]
pub unsafe extern "C" fn krb5_free_error(mut context: krb5_context,
                                         mut val: *mut krb5_error) {
    if val.is_null() { return }
    krb5_free_principal(context, (*val).client);
    krb5_free_principal(context, (*val).server);
    free((*val).text.data as *mut libc::c_void);
    free((*val).e_data.data as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "314:1"]
pub unsafe extern "C" fn krb5_free_kdc_rep(mut context: krb5_context,
                                           mut val: *mut krb5_kdc_rep) {
    if val.is_null() { return }
    krb5_free_pa_data(context, (*val).padata);
    krb5_free_principal(context, (*val).client);
    krb5_free_ticket(context, (*val).ticket);
    free((*val).enc_part.ciphertext.data as *mut libc::c_void);
    krb5_free_enc_kdc_rep_part(context, (*val).enc_part2);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "328:1"]
pub unsafe extern "C" fn krb5_free_kdc_req(mut context: krb5_context,
                                           mut val: *mut krb5_kdc_req) {
    if val.is_null() { return }
    krb5_free_pa_data(context, (*val).padata);
    krb5_free_principal(context, (*val).client);
    krb5_free_principal(context, (*val).server);
    free((*val).ktype as *mut libc::c_void);
    krb5_free_addresses(context, (*val).addresses);
    free((*val).authorization_data.ciphertext.data as *mut libc::c_void);
    krb5_free_authdata(context, (*val).unenc_authdata);
    krb5_free_tickets(context, (*val).second_ticket);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "344:1"]
pub unsafe extern "C" fn krb5_free_keyblock_contents(mut context:
                                                         krb5_context,
                                                     mut key:
                                                         *mut krb5_keyblock) {
    krb5int_c_free_keyblock_contents(context, key);
}
#[no_mangle]
#[c2rust::src_loc = "350:1"]
pub unsafe extern "C" fn krb5_free_keyblock(mut context: krb5_context,
                                            mut val: *mut krb5_keyblock) {
    krb5int_c_free_keyblock(context, val);
}
#[no_mangle]
#[c2rust::src_loc = "358:1"]
pub unsafe extern "C" fn krb5_free_last_req(mut context: krb5_context,
                                            mut val:
                                                *mut *mut krb5_last_req_entry) {
    let mut temp: *mut *mut krb5_last_req_entry =
        0 as *mut *mut krb5_last_req_entry;
    if val.is_null() { return }
    temp = val;
    while !(*temp).is_null() {
        free(*temp as *mut libc::c_void);
        temp = temp.offset(1)
    }
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "370:1"]
pub unsafe extern "C" fn k5_zapfree_pa_data(mut val: *mut *mut krb5_pa_data) {
    let mut pa: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    if val.is_null() { return }
    pa = val;
    while !(*pa).is_null() {
        zapfree((**pa).contents as *mut libc::c_void,
                (**pa).length as size_t);
        zapfree(*pa as *mut libc::c_void,
                ::std::mem::size_of::<krb5_pa_data>() as libc::c_ulong);
        pa = pa.offset(1)
    }
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "384:1"]
pub unsafe extern "C" fn krb5_free_pa_data(mut context: krb5_context,
                                           mut val: *mut *mut krb5_pa_data) {
    let mut temp: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    if val.is_null() { return }
    temp = val;
    while !(*temp).is_null() {
        free((**temp).contents as *mut libc::c_void);
        free(*temp as *mut libc::c_void);
        temp = temp.offset(1)
    }
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "398:1"]
pub unsafe extern "C" fn krb5_free_principal(mut context: krb5_context,
                                             mut val: krb5_principal) {
    let mut i: krb5_int32 = 0;
    if val.is_null() { return }
    if !(*val).data.is_null() {
        i = (*val).length;
        loop  {
            i -= 1;
            if !(i >= 0 as libc::c_int) { break ; }
            free((*(*val).data.offset(i as isize)).data as *mut libc::c_void);
        }
        free((*val).data as *mut libc::c_void);
    }
    free((*val).realm.data as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "416:1"]
pub unsafe extern "C" fn krb5_free_priv(mut context: krb5_context,
                                        mut val: *mut krb5_priv) {
    if val.is_null() { return }
    free((*val).enc_part.ciphertext.data as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "425:1"]
pub unsafe extern "C" fn krb5_free_priv_enc_part(mut context: krb5_context,
                                                 mut val:
                                                     *mut krb5_priv_enc_part) {
    if val.is_null() { return }
    free((*val).user_data.data as *mut libc::c_void);
    krb5_free_address(context, (*val).r_address);
    krb5_free_address(context, (*val).s_address);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "436:1"]
pub unsafe extern "C" fn krb5_free_safe(mut context: krb5_context,
                                        mut val: *mut krb5_safe) {
    if val.is_null() { return }
    free((*val).user_data.data as *mut libc::c_void);
    krb5_free_address(context, (*val).r_address);
    krb5_free_address(context, (*val).s_address);
    krb5_free_checksum(context, (*val).checksum);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "449:1"]
pub unsafe extern "C" fn krb5_free_ticket(mut context: krb5_context,
                                          mut val: *mut krb5_ticket) {
    if val.is_null() { return }
    krb5_free_principal(context, (*val).server);
    free((*val).enc_part.ciphertext.data as *mut libc::c_void);
    krb5_free_enc_tkt_part(context, (*val).enc_part2);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "460:1"]
pub unsafe extern "C" fn krb5_free_tickets(mut context: krb5_context,
                                           mut val: *mut *mut krb5_ticket) {
    let mut temp: *mut *mut krb5_ticket = 0 as *mut *mut krb5_ticket;
    if val.is_null() { return }
    temp = val;
    while !(*temp).is_null() {
        krb5_free_ticket(context, *temp);
        temp = temp.offset(1)
    }
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "473:1"]
pub unsafe extern "C" fn krb5_free_tgt_creds(mut context: krb5_context,
                                             mut tgts: *mut *mut krb5_creds) {
    let mut tgtpp: *mut *mut krb5_creds = 0 as *mut *mut krb5_creds;
    if tgts.is_null() { return }
    tgtpp = tgts;
    while !(*tgtpp).is_null() {
        krb5_free_creds(context, *tgtpp);
        tgtpp = tgtpp.offset(1)
    }
    free(tgts as *mut libc::c_void);
}
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
#[c2rust::src_loc = "484:1"]
pub unsafe extern "C" fn krb5_free_tkt_authent(mut context: krb5_context,
                                               mut val:
                                                   *mut krb5_tkt_authent) {
    if val.is_null() { return }
    krb5_free_ticket(context, (*val).ticket);
    krb5_free_authenticator(context, (*val).authenticator);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "494:1"]
pub unsafe extern "C" fn krb5_free_unparsed_name(mut context: krb5_context,
                                                 mut val: *mut libc::c_char) {
    if !val.is_null() { free(val as *mut libc::c_void); };
}
#[no_mangle]
#[c2rust::src_loc = "501:1"]
pub unsafe extern "C" fn krb5_free_string(mut context: krb5_context,
                                          mut val: *mut libc::c_char) {
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "507:1"]
pub unsafe extern "C" fn krb5_free_sam_challenge_2(mut ctx: krb5_context,
                                                   mut sc2:
                                                       *mut krb5_sam_challenge_2) {
    if sc2.is_null() { return }
    krb5_free_sam_challenge_2_contents(ctx, sc2);
    free(sc2 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "516:1"]
pub unsafe extern "C" fn krb5_free_sam_challenge_2_contents(mut ctx:
                                                                krb5_context,
                                                            mut sc2:
                                                                *mut krb5_sam_challenge_2) {
    let mut cksump: *mut *mut krb5_checksum = 0 as *mut *mut krb5_checksum;
    if sc2.is_null() { return }
    if !(*sc2).sam_challenge_2_body.data.is_null() {
        krb5_free_data_contents(ctx, &mut (*sc2).sam_challenge_2_body);
    }
    if !(*sc2).sam_cksum.is_null() {
        cksump = (*sc2).sam_cksum;
        while !(*cksump).is_null() {
            krb5_free_checksum(ctx, *cksump);
            cksump = cksump.offset(1)
        }
        free((*sc2).sam_cksum as *mut libc::c_void);
        (*sc2).sam_cksum = 0 as *mut *mut krb5_checksum
    };
}
#[no_mangle]
#[c2rust::src_loc = "537:1"]
pub unsafe extern "C" fn krb5_free_sam_challenge_2_body(mut ctx: krb5_context,
                                                        mut sc2:
                                                            *mut krb5_sam_challenge_2_body) {
    if sc2.is_null() { return }
    krb5_free_sam_challenge_2_body_contents(ctx, sc2);
    free(sc2 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "547:1"]
pub unsafe extern "C" fn krb5_free_sam_challenge_2_body_contents(mut ctx:
                                                                     krb5_context,
                                                                 mut sc2:
                                                                     *mut krb5_sam_challenge_2_body) {
    if sc2.is_null() { return }
    if !(*sc2).sam_type_name.data.is_null() {
        krb5_free_data_contents(ctx, &mut (*sc2).sam_type_name);
    }
    if !(*sc2).sam_track_id.data.is_null() {
        krb5_free_data_contents(ctx, &mut (*sc2).sam_track_id);
    }
    if !(*sc2).sam_challenge_label.data.is_null() {
        krb5_free_data_contents(ctx, &mut (*sc2).sam_challenge_label);
    }
    if !(*sc2).sam_challenge.data.is_null() {
        krb5_free_data_contents(ctx, &mut (*sc2).sam_challenge);
    }
    if !(*sc2).sam_response_prompt.data.is_null() {
        krb5_free_data_contents(ctx, &mut (*sc2).sam_response_prompt);
    }
    if !(*sc2).sam_pk_for_sad.data.is_null() {
        krb5_free_data_contents(ctx, &mut (*sc2).sam_pk_for_sad);
    };
}
#[no_mangle]
#[c2rust::src_loc = "567:1"]
pub unsafe extern "C" fn krb5_free_sam_response_2(mut ctx: krb5_context,
                                                  mut sr2:
                                                      *mut krb5_sam_response_2) {
    if sr2.is_null() { return }
    krb5_free_sam_response_2_contents(ctx, sr2);
    free(sr2 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "576:1"]
pub unsafe extern "C" fn krb5_free_sam_response_2_contents(mut ctx:
                                                               krb5_context,
                                                           mut sr2:
                                                               *mut krb5_sam_response_2) {
    if sr2.is_null() { return }
    if !(*sr2).sam_track_id.data.is_null() {
        krb5_free_data_contents(ctx, &mut (*sr2).sam_track_id);
    }
    if !(*sr2).sam_enc_nonce_or_sad.ciphertext.data.is_null() {
        krb5_free_data_contents(ctx,
                                &mut (*sr2).sam_enc_nonce_or_sad.ciphertext);
    };
}
#[no_mangle]
#[c2rust::src_loc = "587:1"]
pub unsafe extern "C" fn krb5_free_enc_sam_response_enc_2(mut ctx:
                                                              krb5_context,
                                                          mut esre2:
                                                              *mut krb5_enc_sam_response_enc_2) {
    if esre2.is_null() { return }
    krb5_free_enc_sam_response_enc_2_contents(ctx, esre2);
    free(esre2 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "597:1"]
pub unsafe extern "C" fn krb5_free_enc_sam_response_enc_2_contents(mut ctx:
                                                                       krb5_context,
                                                                   mut esre2:
                                                                       *mut krb5_enc_sam_response_enc_2) {
    if esre2.is_null() { return }
    if !(*esre2).sam_sad.data.is_null() {
        krb5_free_data_contents(ctx, &mut (*esre2).sam_sad);
    };
}
#[no_mangle]
#[c2rust::src_loc = "607:1"]
pub unsafe extern "C" fn krb5_free_pa_enc_ts(mut ctx: krb5_context,
                                             mut pa_enc_ts:
                                                 *mut krb5_pa_enc_ts) {
    if pa_enc_ts.is_null() { return }
    free(pa_enc_ts as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "615:1"]
pub unsafe extern "C" fn krb5_free_pa_for_user(mut context: krb5_context,
                                               mut req:
                                                   *mut krb5_pa_for_user) {
    if req.is_null() { return }
    krb5_free_principal(context, (*req).user);
    (*req).user = 0 as krb5_principal;
    krb5_free_checksum_contents(context, &mut (*req).cksum);
    krb5_free_data_contents(context, &mut (*req).auth_package);
    free(req as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "627:1"]
pub unsafe extern "C" fn krb5_free_s4u_userid_contents(mut context:
                                                           krb5_context,
                                                       mut user_id:
                                                           *mut krb5_s4u_userid) {
    if user_id.is_null() { return }
    (*user_id).nonce = 0 as libc::c_int;
    krb5_free_principal(context, (*user_id).user);
    (*user_id).user = 0 as krb5_principal;
    krb5_free_data_contents(context, &mut (*user_id).subject_cert);
    (*user_id).subject_cert.length = 0 as libc::c_int as libc::c_uint;
    (*user_id).subject_cert.data = 0 as *mut libc::c_char;
    (*user_id).options = 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "641:1"]
pub unsafe extern "C" fn krb5_free_pa_s4u_x509_user(mut context: krb5_context,
                                                    mut req:
                                                        *mut krb5_pa_s4u_x509_user) {
    if req.is_null() { return }
    krb5_free_s4u_userid_contents(context, &mut (*req).user_id);
    krb5_free_checksum_contents(context, &mut (*req).cksum);
    free(req as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "651:1"]
pub unsafe extern "C" fn krb5_free_pa_pac_req(mut context: krb5_context,
                                              mut req: *mut krb5_pa_pac_req) {
    free(req as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "658:1"]
pub unsafe extern "C" fn krb5_free_fast_req(mut context: krb5_context,
                                            mut val: *mut krb5_fast_req) {
    if val.is_null() { return }
    krb5_free_kdc_req(context, (*val).req_body);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "667:1"]
pub unsafe extern "C" fn krb5_free_fast_armor(mut context: krb5_context,
                                              mut val: *mut krb5_fast_armor) {
    if val.is_null() { return }
    krb5_free_data_contents(context, &mut (*val).armor_value);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "676:1"]
pub unsafe extern "C" fn krb5_free_fast_response(mut context: krb5_context,
                                                 mut val:
                                                     *mut krb5_fast_response) {
    if val.is_null() { return }
    krb5_free_pa_data(context, (*val).padata);
    krb5_free_fast_finished(context, (*val).finished);
    krb5_free_keyblock(context, (*val).strengthen_key);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "687:1"]
pub unsafe extern "C" fn krb5_free_fast_finished(mut context: krb5_context,
                                                 mut val:
                                                     *mut krb5_fast_finished) {
    if val.is_null() { return }
    krb5_free_principal(context, (*val).client);
    krb5_free_checksum_contents(context, &mut (*val).ticket_checksum);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "697:1"]
pub unsafe extern "C" fn krb5_free_fast_armored_req(mut context: krb5_context,
                                                    mut val:
                                                        *mut krb5_fast_armored_req) {
    if val.is_null() { return }
    if !(*val).armor.is_null() {
        krb5_free_fast_armor(context, (*val).armor);
    }
    krb5_free_data_contents(context, &mut (*val).enc_part.ciphertext);
    if !(*val).req_checksum.contents.is_null() {
        krb5_free_checksum_contents(context, &mut (*val).req_checksum);
    }
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "710:1"]
pub unsafe extern "C" fn k5_free_data_ptr_list(mut list:
                                                   *mut *mut krb5_data) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !list.is_null() && !(*list.offset(i as isize)).is_null() {
        krb5_free_data(0 as krb5_context, *list.offset(i as isize));
        i += 1
    }
    free(list as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "720:1"]
pub unsafe extern "C" fn krb5int_free_data_list(mut context: krb5_context,
                                                mut data: *mut krb5_data) {
    let mut i: libc::c_int = 0;
    if data.is_null() { return }
    i = 0 as libc::c_int;
    while !(*data.offset(i as isize)).data.is_null() {
        free((*data.offset(i as isize)).data as *mut libc::c_void);
        i += 1
    }
    free(data as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "734:1"]
pub unsafe extern "C" fn krb5_free_ad_kdcissued(mut context: krb5_context,
                                                mut val:
                                                    *mut krb5_ad_kdcissued) {
    if val.is_null() { return }
    krb5_free_checksum_contents(context, &mut (*val).ad_checksum);
    krb5_free_principal(context, (*val).i_principal);
    krb5_free_authdata(context, (*val).elements);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "746:1"]
pub unsafe extern "C" fn krb5_free_ad_signedpath(mut context: krb5_context,
                                                 mut val:
                                                     *mut krb5_ad_signedpath) {
    let mut i: libc::c_int = 0;
    if val.is_null() { return }
    krb5_free_checksum_contents(context, &mut (*val).checksum);
    if !(*val).delegated.is_null() {
        i = 0 as libc::c_int;
        while !(*(*val).delegated.offset(i as isize)).is_null() {
            krb5_free_principal(context,
                                *(*val).delegated.offset(i as isize));
            i += 1
        }
        free((*val).delegated as *mut libc::c_void);
    }
    krb5_free_pa_data(context, (*val).method_data);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "764:1"]
pub unsafe extern "C" fn krb5_free_iakerb_header(mut context: krb5_context,
                                                 mut val:
                                                     *mut krb5_iakerb_header) {
    if val.is_null() { return }
    krb5_free_data_contents(context, &mut (*val).target_realm);
    krb5_free_data(context, (*val).cookie);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "775:1"]
pub unsafe extern "C" fn krb5_free_iakerb_finished(mut context: krb5_context,
                                                   mut val:
                                                       *mut krb5_iakerb_finished) {
    if val.is_null() { return }
    krb5_free_checksum_contents(context, &mut (*val).checksum);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "785:1"]
pub unsafe extern "C" fn k5_free_algorithm_identifier(mut context:
                                                          krb5_context,
                                                      mut val:
                                                          *mut krb5_algorithm_identifier) {
    if val.is_null() { return }
    free((*val).algorithm.data as *mut libc::c_void);
    free((*val).parameters.data as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "796:1"]
pub unsafe extern "C" fn k5_free_otp_tokeninfo(mut context: krb5_context,
                                               mut val:
                                                   *mut krb5_otp_tokeninfo) {
    let mut alg: *mut *mut krb5_algorithm_identifier =
        0 as *mut *mut krb5_algorithm_identifier;
    if val.is_null() { return }
    free((*val).vendor.data as *mut libc::c_void);
    free((*val).challenge.data as *mut libc::c_void);
    free((*val).token_id.data as *mut libc::c_void);
    free((*val).alg_id.data as *mut libc::c_void);
    alg = (*val).supported_hash_alg;
    while !alg.is_null() && !(*alg).is_null() {
        k5_free_algorithm_identifier(context, *alg);
        alg = alg.offset(1)
    }
    free((*val).supported_hash_alg as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "813:1"]
pub unsafe extern "C" fn k5_free_pa_otp_challenge(mut context: krb5_context,
                                                  mut val:
                                                      *mut krb5_pa_otp_challenge) {
    let mut ti: *mut *mut krb5_otp_tokeninfo =
        0 as *mut *mut krb5_otp_tokeninfo;
    if val.is_null() { return }
    free((*val).nonce.data as *mut libc::c_void);
    free((*val).service.data as *mut libc::c_void);
    ti = (*val).tokeninfo;
    while !(*ti).is_null() {
        k5_free_otp_tokeninfo(context, *ti);
        ti = ti.offset(1)
    }
    free((*val).tokeninfo as *mut libc::c_void);
    free((*val).salt.data as *mut libc::c_void);
    free((*val).s2kparams.data as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "830:1"]
pub unsafe extern "C" fn k5_free_pa_otp_req(mut context: krb5_context,
                                            mut val: *mut krb5_pa_otp_req) {
    if val.is_null() { return }
    (*val).flags = 0 as libc::c_int;
    free((*val).nonce.data as *mut libc::c_void);
    free((*val).enc_data.ciphertext.data as *mut libc::c_void);
    if !(*val).hash_alg.is_null() {
        k5_free_algorithm_identifier(context, (*val).hash_alg);
    }
    free((*val).otp_value.data as *mut libc::c_void);
    free((*val).pin.data as *mut libc::c_void);
    free((*val).challenge.data as *mut libc::c_void);
    free((*val).counter.data as *mut libc::c_void);
    free((*val).token_id.data as *mut libc::c_void);
    free((*val).alg_id.data as *mut libc::c_void);
    free((*val).vendor.data as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "850:1"]
pub unsafe extern "C" fn k5_free_kkdcp_message(mut context: krb5_context,
                                               mut val:
                                                   *mut krb5_kkdcp_message) {
    if val.is_null() { return }
    free((*val).target_domain.data as *mut libc::c_void);
    free((*val).kerb_message.data as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[c2rust::src_loc = "860:1"]
unsafe extern "C" fn free_vmac(mut context: krb5_context,
                               mut val: *mut krb5_verifier_mac) {
    if val.is_null() { return }
    krb5_free_principal(context, (*val).princ);
    krb5_free_checksum_contents(context, &mut (*val).checksum);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "870:1"]
pub unsafe extern "C" fn k5_free_cammac(mut context: krb5_context,
                                        mut val: *mut krb5_cammac) {
    let mut vp: *mut *mut krb5_verifier_mac =
        0 as *mut *mut krb5_verifier_mac;
    if val.is_null() { return }
    krb5_free_authdata(context, (*val).elements);
    free_vmac(context, (*val).kdc_verifier);
    free_vmac(context, (*val).svc_verifier);
    vp = (*val).other_verifiers;
    while !vp.is_null() && !(*vp).is_null() {
        free_vmac(context, *vp);
        vp = vp.offset(1)
    }
    free((*val).other_verifiers as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "886:1"]
pub unsafe extern "C" fn k5_free_secure_cookie(mut context: krb5_context,
                                               mut val:
                                                   *mut krb5_secure_cookie) {
    if val.is_null() { return }
    k5_zapfree_pa_data((*val).data);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "895:1"]
pub unsafe extern "C" fn k5_free_spake_factor(mut context: krb5_context,
                                              mut val:
                                                  *mut krb5_spake_factor) {
    if val.is_null() { return }
    if !(*val).data.is_null() {
        zapfree((*(*val).data).data as *mut libc::c_void,
                (*(*val).data).length as size_t);
    }
    free((*val).data as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "906:1"]
pub unsafe extern "C" fn k5_free_pa_spake(mut context: krb5_context,
                                          mut val: *mut krb5_pa_spake) {
    let mut f: *mut *mut krb5_spake_factor = 0 as *mut *mut krb5_spake_factor;
    if val.is_null() { return }
    match (*val).choice as libc::c_int {
        0 => { free((*val).u.support.groups as *mut libc::c_void); }
        1 => {
            krb5_free_data_contents(context, &mut (*val).u.challenge.pubkey);
            f = (*val).u.challenge.factors;
            while !f.is_null() && !(*f).is_null() {
                k5_free_spake_factor(context, *f);
                f = f.offset(1)
            }
            free((*val).u.challenge.factors as *mut libc::c_void);
        }
        2 => {
            krb5_free_data_contents(context, &mut (*val).u.response.pubkey);
            krb5_free_data_contents(context,
                                    &mut (*val).u.response.factor.ciphertext);
        }
        3 => {
            krb5_free_data_contents(context,
                                    &mut (*val).u.encdata.ciphertext);
        }
        _ => { }
    }
    free(val as *mut libc::c_void);
}
