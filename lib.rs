#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]


#[macro_use]
extern crate c2rust_bitfields;#[macro_use]
extern crate c2rust_asm_casts;
extern crate libc;



pub mod src {
pub mod clients {
pub mod kinit {
pub mod kinit_kdb;
} // mod kinit
} // mod clients
pub mod kadmin {
pub mod cli {
pub mod kadmin;
pub mod keytab;
pub mod keytab_local;
} // mod cli
pub mod dbutil {
pub mod dump;
pub mod kadm5_create;
pub mod kdb5_create;
pub mod kdb5_destroy;
pub mod kdb5_mkey;
pub mod kdb5_stash;
pub mod kdb5_util;
pub mod ovload;
pub mod tabdump;
} // mod dbutil
pub mod server {
pub mod auth_acl;
pub mod auth_self;
pub mod ipropd_svc;
pub mod kadm_rpc_svc;
pub mod misc;
pub mod ovsec_kadmd;
pub mod schpw;
pub mod server_stubs;
} // mod server
pub mod testing {
pub mod util {
pub mod tcl_kadm5;
} // mod util
} // mod testing
} // mod kadmin
pub mod kdc {
pub mod do_as_req;
pub mod kdc_util;
pub mod main;
} // mod kdc
pub mod kprop {
pub mod kpropd;
pub mod kpropd_rpc;
pub mod kproplog;
} // mod kprop
pub mod lib {
pub mod apputils {
pub mod net_server;
} // mod apputils
pub mod gssapi {
pub mod krb5 {
pub mod disp_status;
} // mod krb5
} // mod gssapi
pub mod kadm5 {
pub mod alt_prof;
pub mod chpass_util;
pub mod clnt {
pub mod client_init;
pub mod client_principal;
pub mod client_rpc;
pub mod clnt_chpass_util;
pub mod clnt_policy;
pub mod clnt_privs;
} // mod clnt
pub mod kadm_rpc_xdr;
pub mod misc_free;
pub mod srv {
pub mod adb_xdr;
pub mod kadm5_hook;
pub mod pwqual;
pub mod pwqual_dict;
pub mod pwqual_empty;
pub mod pwqual_hesiod;
pub mod pwqual_princ;
pub mod server_init;
pub mod server_kdb;
pub mod server_misc;
pub mod svr_chpass_util;
pub mod svr_iters;
pub mod svr_policy;
pub mod svr_principal;
} // mod srv
pub mod str_conv;
} // mod kadm5
pub mod kdb {
pub mod iprop_xdr;
pub mod kdb5;
pub mod kdb_convert;
pub mod kdb_log;
} // mod kdb
pub mod rpc {
pub mod auth_gss;
pub mod auth_gssapi;
pub mod auth_gssapi_misc;
pub mod auth_none;
pub mod auth_unix;
pub mod authgss_prot;
pub mod authunix_prot;
pub mod bindresvport;
pub mod clnt_generic;
pub mod clnt_perror;
pub mod clnt_raw;
pub mod clnt_simple;
pub mod clnt_tcp;
pub mod clnt_udp;
pub mod get_myaddress;
pub mod getrpcport;
pub mod pmap_clnt;
pub mod pmap_getmaps;
pub mod pmap_getport;
pub mod pmap_prot;
pub mod pmap_prot2;
pub mod pmap_rmt;
pub mod rpc_callmsg;
pub mod rpc_commondata;
pub mod rpc_dtablesize;
pub mod rpc_prot;
pub mod svc;
pub mod svc_auth;
pub mod svc_auth_gss;
pub mod svc_auth_gssapi;
pub mod svc_auth_none;
pub mod svc_auth_unix;
pub mod svc_raw;
pub mod svc_run;
pub mod svc_simple;
pub mod svc_tcp;
pub mod svc_udp;
pub mod unit_test {
pub mod client;
pub mod rpc_test_clnt;
pub mod rpc_test_svc;
pub mod server;
} // mod unit_test
pub mod xdr;
pub mod xdr_alloc;
pub mod xdr_array;
pub mod xdr_float;
pub mod xdr_mem;
pub mod xdr_rec;
pub mod xdr_reference;
pub mod xdr_sizeof;
pub mod xdr_stdio;
} // mod rpc
} // mod lib
pub mod plugins {
pub mod kadm5_auth {
pub mod test {
pub mod main;
} // mod test
} // mod kadm5_auth
pub mod kadm5_hook {
pub mod test {
pub mod main;
} // mod test
} // mod kadm5_hook
pub mod kdb {
pub mod db2 {
pub mod adb_openclose;
pub mod adb_policy;
pub mod db2_exp;
pub mod kdb_db2;
pub mod lockout;
pub mod pol_xdr;
} // mod db2
pub mod lmdb {
pub mod kdb_lmdb;
pub mod lockout;
} // mod lmdb
} // mod kdb
pub mod pwqual {
pub mod test {
pub mod main;
} // mod test
} // mod pwqual
} // mod plugins
} // mod src

