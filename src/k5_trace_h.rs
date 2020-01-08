extern "C" {
    #[no_mangle]
    pub fn krb5int_trace(context: crate::krb5_h::krb5_context, fmt: *const i8, _: ...);
}
