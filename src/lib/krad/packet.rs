use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:30"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:30"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:30"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:30"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:30"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:30"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:30"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "561:5"]
    pub struct C2RustUnnamed {
        pub i: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "608:12"]
    pub struct C2RustUnnamed_0 {
        pub i: uint16_t,
    }
    /* Do it in macro form so we get the file/line of the invocation if
   the assertion fails.  */
    /* forward declaration for use in initializer */
    /* so ';' following macro use won't get error */
    /* This should be called in finalization only, so we shouldn't have
   multiple active threads mucking around in our library at this
   point.  So ignore the once_t object and just look at the flag.

   XXX Could we have problems with memory coherence between processors
   if we don't invoke mutex/once routines?  Probably not, the
   application code should already be coordinating things such that
   the library code is not in use by this point, and memory
   synchronization will be needed there.  */
    /* If we're using gcc, if the C++ support works, the compiler should
   build executables and shared libraries that support the use of
   static constructors and destructors.  The C compiler supports a
   function attribute that makes use of the same facility as C++.

   XXX How do we know if the C++ support actually works?  */
    /* Read and write integer values as (unaligned) octet strings in
   specific byte orders.  Add per-platform optimizations as
   needed.  */
    /* Check for BIG/LITTLE_ENDIAN macros.  If exactly one is defined, use
   it.  If both are defined, then BYTE_ORDER should be defined and
   match one of them.  Try those symbols, then try again with an
   underscore prefix.  */
    /* Optimize for GCC on platforms with known byte orders.

   GCC's packed structures can be written to with any alignment; the
   compiler will use byte operations, unaligned-word operations, or
   normal memory ops as appropriate for the architecture.

   This assumes the availability of uint##_t types, which should work
   on most of our platforms except Windows, where we're not using
   GCC.  */
    /* To do: Define SWAP16, SWAP32, SWAP64 macros to byte-swap values
   with the indicated numbers of bits.

   Linux: byteswap.h, bswap_16 etc.
   Solaris 10: none
   macOS: machine/endian.h or byte_order.h, NXSwap{Short,Int,LongLong}
   NetBSD: sys/bswap.h, bswap16 etc.  */
    /* Note that on Windows at least this file can be included from C++
   source, so casts *from* void* are required.  */
    #[inline]
    #[c2rust::src_loc = "554:1"]
    pub unsafe extern "C" fn store_16_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = __bswap_16(val as __uint16_t);
    }
    #[inline]
    #[c2rust::src_loc = "601:1"]
    pub unsafe extern "C" fn load_16_be(mut cvp: *const libc::c_void)
     -> libc::c_ushort {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return __bswap_16((*(p as *const C2RustUnnamed_0)).i);
    }
    use super::stdint_uintn_h::uint16_t;
    use super::byteswap_h::__bswap_16;
    use super::types_h::__uint16_t;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:30"]
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
    #[c2rust::src_loc = "137:1"]
    pub type krb5_int16 = int16_t;
    #[c2rust::src_loc = "138:1"]
    pub type krb5_ui_2 = uint16_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "180:1"]
    pub type krb5_cksumtype = krb5_int32;
    #[c2rust::src_loc = "182:1"]
    pub type krb5_keyusage = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
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
    /* *
 *@deprecated
 */
    /* *
 * Convert a Kerberos V5 credentials to a Kerberos V4 credentials
 *
 * @note Not implemented
 *
 * @retval KRB524_KRB4_DISABLED (always)
 */
    /* libkt.spec */
    /* *
 * Get a handle for a key table.
 *
 * @param [in]  context         Library context
 * @param [in]  name            Name of the key table
 * @param [out] ktid            Key table handle
 *
 * Resolve the key table name @a name and set @a ktid to a handle identifying
 * the key table.  Use krb5_kt_close() to free @a ktid when it is no longer
 * needed.
 *
 * @a name must be of the form @c type:residual, where @a type must be a type
 * known to the library and @a residual portion should be specific to the
 * particular keytab type.  If no @a type is given, the default is @c FILE.
 *
 * If @a name is of type @c FILE, the keytab file is not opened by this call.
 *
 * @code
 *  Example: krb5_kt_resolve(context, "FILE:/tmp/filename", &ktid);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Duplicate keytab handle.
 *
 * @param [in]  context         Library context
 * @param [in]  in              Key table handle to be duplicated
 * @param [out] out             Key table handle
 *
 * Create a new handle referring to the same key table as @a in.  The new
 * handle and @a in can be closed independently.
 *
 * @version New in 1.12
 */
    /* *
 * Get the default key table name.
 *
 * @param [in]     context      Library context
 * @param [out]    name         Default key table name
 * @param [in]     name_size    Space available in @a name
 *
 * Fill @a name with the name of the default key table for @a context.
 *
 * @sa MAX_KEYTAB_NAME_LEN
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_CONFIG_NOTENUFSPACE Buffer is too short
 * @return
 * Kerberos error codes
 */
    /* *
 * Resolve the default key table.
 *
 * @param [in]  context         Library context
 * @param [out] id              Key table handle
 *
 * Set @a id to a handle to the default key table.  The key table is not
 * opened.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Resolve the default client key table.
 *
 * @param [in]     context      Library context
 * @param [out]    keytab_out   Key table handle
 *
 * Fill @a keytab_out with a handle to the default client key table.
 *
 * @version New in 1.11
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Free the contents of a key table entry.
 *
 * @param [in] context          Library context
 * @param [in] entry            Key table entry whose contents are to be freed
 *
 * @note The pointer is not freed.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* * @deprecated Use krb5_free_keytab_entry_contents instead. */
    /* remove and add are functions, so that they can return NOWRITE
   if not a writable keytab */
    /* *
 * Remove an entry from a key table.
 *
 * @param [in] context          Library context
 * @param [in] id               Key table handle
 * @param [in] entry            Entry to remove from key table
 *
 * @retval
 * 0 Success
 * @retval
 *  KRB5_KT_NOWRITE     Key table is not writable
 * @return
 * Kerberos error codes
 */
    /* *
 * Add a new entry to a key table.
 *
 * @param [in] context          Library context
 * @param [in] id               Key table handle
 * @param [in] entry            Entry to be added
 *
 * @retval
 * 0  Success
 * @retval
 *  ENOMEM    Insufficient memory
 * @retval
 *  KRB5_KT_NOWRITE  Key table is not writeable
 * @return
 * Kerberos error codes
 */
    /* *
 * Convert a principal name into the default salt for that principal.
 *
 * @param [in]  context         Library context
 * @param [in]  pr              Principal name
 * @param [out] ret             Default salt for @a pr to be filled in
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* librc.spec--see rcache.h */
    /* libcc.spec */
    /* *
 * Resolve a credential cache name.
 *
 * @param [in]  context         Library context
 * @param [in]  name            Credential cache name to be resolved
 * @param [out] cache           Credential cache handle
 *
 * Fills in @a cache with a @a cache handle that corresponds to the name in @a
 * name.  @a name should be of the form @c type:residual, and @a type must be a
 * type known to the library.  If the @a name does not contain a colon,
 * interpret it as a file name.
 *
 * @code
 * Example: krb5_cc_resolve(context, "MEMORY:C_", &cache);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Duplicate ccache handle.
 *
 * @param [in]  context         Library context
 * @param [in]  in              Credential cache handle to be duplicated
 * @param [out] out             Credential cache handle
 *
 * Create a new handle referring to the same cache as @a in.
 * The new handle and @a in can be closed independently.
 */
    /* *
 * Return the name of the default credential cache.
 *
 * @param [in] context          Library context
 *
 * Return a pointer to the default credential cache name for @a context, as
 * determined by a prior call to krb5_cc_set_default_name(), by the KRB5CCNAME
 * environment variable, by the default_ccache_name profile variable, or by the
 * operating system or build-time default value.  The returned value must not
 * be modified or freed by the caller.  The returned value becomes invalid when
 * @a context is destroyed krb5_free_context() or if a subsequent call to
 * krb5_cc_set_default_name() is made on @a context.
 *
 * The default credential cache name is cached in @a context between calls to
 * this function, so if the value of KRB5CCNAME changes in the process
 * environment after the first call to this function on, that change will not
 * be reflected in later calls with the same context.  The caller can invoke
 * krb5_cc_set_default_name() with a NULL value of @a name to clear the cached
 * value and force the default name to be recomputed.
 *
 * @return
 * Name of default credential cache for the current user.
 */
    /* *
 * Set the default credential cache name.
 *
 * @param [in] context          Library context
 * @param [in] name             Default credential cache name or NULL
 *
 * Set the default credential cache name to @a name for future operations using
 * @a context.  If @a name is NULL, clear any previous application-set default
 * name and forget any cached value of the default name for @a context.
 *
 * Calls to this function invalidate the result of any previous calls to
 * krb5_cc_default_name() using @a context.
 *
 * @retval
 *  0  Success
 * @retval
 *  KV5M_CONTEXT          Bad magic number for @c _krb5_context structure
 * @return
 * Kerberos error codes
 */
    /* *
 * Resolve the default credential cache name.
 *
 * @param [in]  context         Library context
 * @param [out] ccache          Pointer to credential cache name
 *
 * Create a handle to the default credential cache as given by
 * krb5_cc_default_name().
 *
 * @retval
 * 0  Success
 * @retval
 * KV5M_CONTEXT            Bad magic number for @c _krb5_context structure
 * @retval
 * KRB5_FCC_INTERNAL       The name of the default credential cache cannot be
 *                         obtained
 * @return
 * Kerberos error codes
 */
    /* *
 * Copy a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  incc            Credential cache to be copied
 * @param [out] outcc           Copy of credential cache to be filled in
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Get a configuration value from a credential cache.
 *
 * @param [in]     context      Library context
 * @param [in]     id           Credential cache handle
 * @param [in]     principal    Configuration for this principal;
 *                              if NULL, global for the whole cache
 * @param [in]     key          Name of config variable
 * @param [out]    data         Data to be fetched
 *
 * Use krb5_free_data_contents() to free @a data when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Store a configuration value in a credential cache.
 *
 * @param [in]     context      Library context
 * @param [in]     id           Credential cache handle
 * @param [in]     principal    Configuration for a specific principal;
 *                              if NULL, global for the whole cache
 * @param [in]     key          Name of config variable
 * @param [in]     data         Data to store, or NULL to remove
 *
 * @note Existing configuration under the same key is over-written.
 *
 * @warning Before version 1.10 @a data was assumed to be always non-null.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Test whether a principal is a configuration principal.
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal to check
 *
 * @return
 * @c TRUE if the principal is a configuration principal (generated part of
 * krb5_cc_set_config()); @c FALSE otherwise.
 */
    /* *
 * Make a credential cache the primary cache for its collection.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * If the type of @a cache supports it, set @a cache to be the primary
 * credential cache for the collection it belongs to.
 *
 * @retval
 * 0  Success, or the type of @a cache doesn't support switching
 * @return
 * Kerberos error codes
 */
    /* *
 * Determine whether a credential cache type supports switching.
 *
 * @param [in] context          Library context
 * @param [in] type             Credential cache type
 *
 * @version New in 1.10
 *
 * @retval TRUE if @a type supports switching
 * @retval FALSE if it does not or is not a valid credential cache type.
 */
    /* *
 * Find a credential cache with a specified client principal.
 *
 * @param [in]  context         Library context
 * @param [in]  client          Client principal
 * @param [out] cache_out       Credential cache handle
 *
 * Find a cache within the collection whose default principal is @a client.
 * Use @a krb5_cc_close to close @a ccache when it is no longer needed.
 *
 * @retval 0 Success
 * @retval KRB5_CC_NOTFOUND
 *
 * @sa krb5_cccol_cursor_new
 *
 * @version New in 1.10
 */
    /* *
 * Select a credential cache to use with a server principal.
 *
 * @param [in]  context         Library context
 * @param [in]  server          Server principal
 * @param [out] cache_out       Credential cache handle
 * @param [out] princ_out       Client principal
 *
 * Select a cache within the collection containing credentials most appropriate
 * for use with @a server, according to configured rules and heuristics.
 *
 * Use krb5_cc_close() to release @a cache_out when it is no longer needed.
 * Use krb5_free_principal() to release @a princ_out when it is no longer
 * needed.  Note that @a princ_out is set in some error conditions.
 *
 * @return
 * If an appropriate cache is found, 0 is returned, @a cache_out is set to the
 * selected cache, and @a princ_out is set to the default principal of that
 * cache.
 *
 * If the appropriate client principal can be authoritatively determined but
 * the cache collection contains no credentials for that principal, then
 * KRB5_CC_NOTFOUND is returned, @a cache_out is set to NULL, and @a princ_out
 * is set to the appropriate client principal.
 *
 * If no configured mechanism can determine the appropriate cache or principal,
 * KRB5_CC_NOTFOUND is returned and @a cache_out and @a princ_out are set to
 * NULL.
 *
 * Any other error code indicates a fatal error in the processing of a cache
 * selection mechanism.
 *
 * @version New in 1.10
 */
    /* krb5_free.c */
/* *
 * Free the storage assigned to a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Principal to be freed
 */
    /* *
 * Free a krb5_authenticator structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Authenticator structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* *
 * Free the data stored in array of addresses.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of addresses to be freed
 *
 * This function frees the contents of @a val and the array itself.
 *
 * @note The last entry in the array must be a NULL pointer.
 */
    /* *
 * Free the storage assigned to array of authentication data.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of authentication data to be freed
 *
 * This function frees the contents of @a val and the array itself.
 *
 * @note The last entry in the array must be a NULL pointer.
 */
    /* *
 * Free a ticket.
 *
 * @param [in] context          Library context
 * @param [in] val              Ticket to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* *
 * Free an error allocated by krb5_read_error() or krb5_sendauth().
 *
 * @param [in] context          Library context
 * @param [in] val              Error data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* *
 * Free a krb5_creds structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Credential structure to be freed.
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* *
 * Free the contents of a krb5_creds structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Credential structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
    /* *
 * Free a krb5_checksum structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Checksum structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* *
 * Free the contents of a krb5_checksum structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Checksum structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
    /* *
 * Free a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Keyblock to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* *
 * Free the contents of a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] key              Keyblock to be freed
 *
 * This function frees the contents of @a key, but not the structure itself.
 */
    /* *
 * Free a krb5_ap_rep_enc_part structure.
 *
 * @param [in] context          Library context
 * @param [in] val              AP-REP enc part to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* *
 * Free a krb5_data structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* Free a krb5_octet_data structure (should be unused). */
    /* *
 * Free the contents of a krb5_data structure and zero the data field.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
    /* *
 * Free a string representation of a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Name string to be freed
 */
    /* *
 * Free a string allocated by a krb5 function.
 *
 * @param [in] context          Library context
 * @param [in] val              String to be freed
 *
 * @version New in 1.10
 */
    /* *
 * Free an array of encryption types.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of enctypes to be freed
 *
 * @version New in 1.12
 */
    /* *
 * Free an array of checksum types.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of checksum types to be freed
 */
    /* From krb5/os, but needed by the outside world */
/* *
 * Retrieve the system time of day, in sec and ms, since the epoch.
 *
 * @param [in]  context         Library context
 * @param [out] seconds         System timeofday, seconds portion
 * @param [out] microseconds    System timeofday, microseconds portion
 *
 * This function retrieves the system time of day with the context
 * specific time offset adjustment.
 *
 * @sa krb5_crypto_us_timeofday()
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Retrieve the current time with context specific time offset adjustment.
 *
 * @param [in]  context         Library context
 * @param [out] timeret         Timestamp to fill in
 *
 * This function retrieves the system time of day with the context specific
 * time offset adjustment.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Check if a timestamp is within the allowed clock skew of the current time.
 *
 * @param [in]     context      Library context
 * @param [in]     date         Timestamp to check
 *
 * This function checks if @a date is close enough to the current time
 * according to the configured allowable clock skew.
 *
 * @version New in 1.10
 *
 * @retval 0 Success
 * @retval KRB5KRB_AP_ERR_SKEW @a date is not within allowable clock skew
 */
    /* *
 * Return all interface addresses for this host.
 *
 * @param [in]  context         Library context
 * @param [out] addr            Array of krb5_address pointers, ending with
 *                              NULL
 *
 * Use krb5_free_addresses() to free @a addr when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the default realm.
 *
 * @param [in]  context         Library context
 * @param [out] lrealm          Default realm name
 *
 * Retrieves the default realm to be used if no user-specified realm is
 * available.
 *
 * Use krb5_free_default_realm() to free @a lrealm when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Override the default realm for the specified context.
 *
 * @param [in]     context      Library context
 * @param [in]     lrealm       Realm name for the default realm
 *
 * If @a lrealm is NULL, clear the default realm setting.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Free a default realm string returned by krb5_get_default_realm().
 *
 * @param [in] context          Library context
 * @param [in] lrealm           Realm to be freed
 */
    /* *
 * Canonicalize a hostname, possibly using name service.
 *
 * @param [in]  context         Library context
 * @param [in]  host            Input hostname
 * @param [out] canonhost_out   Canonicalized hostname
 *
 * This function canonicalizes orig_hostname, possibly using name service
 * lookups if configuration permits.  Use krb5_free_string() to free @a
 * canonhost_out when it is no longer needed.
 *
 * @version New in 1.15
 */
    /* *
 * Generate a full principal name from a service name.
 *
 * @param [in]  context         Library context
 * @param [in]  hostname        Host name, or NULL to use local host
 * @param [in]  sname           Service name, or NULL to use @c "host"
 * @param [in]  type            Principal type
 * @param [out] ret_princ       Generated principal
 *
 * This function converts a @a hostname and @a sname into @a krb5_principal
 * structure @a ret_princ.  The returned principal will be of the form @a
 * sname\/hostname\@REALM where REALM is determined by krb5_get_host_realm().
 * In some cases this may be the referral (empty) realm.
 *
 * The @a type can be one of the following:
 *
 * @li #KRB5_NT_SRV_HST canonicalizes the host name before looking up the
 * realm and generating the principal.
 *
 * @li #KRB5_NT_UNKNOWN accepts the hostname as given, and does not
 * canonicalize it.
 *
 * Use krb5_free_principal to free @a ret_princ when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Test whether a principal matches a matching principal.
 *
 * @param [in]  context         Library context
 * @param [in]  matching        Matching principal
 * @param [in]  princ           Principal to test
 *
 * @note A matching principal is a host-based principal with an empty realm
 * and/or second data component (hostname).  Profile configuration may cause
 * the hostname to be ignored even if it is present.  A principal matches a
 * matching principal if the former has the same non-empty (and non-ignored)
 * components of the latter.
 *
 * If @a matching is NULL, return TRUE.  If @a matching is not a matching
 * principal, return the value of krb5_principal_compare(context, matching,
 * princ).
 *
 * @return
 * TRUE if @a princ matches @a matching, FALSE otherwise.
 */
    /* *
 * Change a password for an existing Kerberos account.
 *
 * @param [in]  context             Library context
 * @param [in]  creds               Credentials for kadmin/changepw service
 * @param [in]  newpw               New password
 * @param [out] result_code         Numeric error code from server
 * @param [out] result_code_string  String equivalent to @a result_code
 * @param [out] result_string       Change password response from the KDC
 *
 * Change the password for the existing principal identified by @a creds.
 *
 * The possible values of the output @a result_code are:
 *
 * @li #KRB5_KPASSWD_SUCCESS   (0) - success
 * @li #KRB5_KPASSWD_MALFORMED (1) - Malformed request error
 * @li #KRB5_KPASSWD_HARDERROR (2) - Server error
 * @li #KRB5_KPASSWD_AUTHERROR (3) - Authentication error
 * @li #KRB5_KPASSWD_SOFTERROR (4) - Password change rejected
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set a password for a principal using specified credentials.
 *
 * @param [in]  context              Library context
 * @param [in]  creds                Credentials for kadmin/changepw service
 * @param [in]  newpw                New password
 * @param [in]  change_password_for  Change the password for this principal
 * @param [out] result_code          Numeric error code from server
 * @param [out] result_code_string   String equivalent to @a result_code
 * @param [out] result_string        Data returned from the remote system
 *
 * This function uses the credentials @a creds to set the password @a newpw for
 * the principal @a change_password_for.  It implements the set password
 * operation of RFC 3244, for interoperability with Microsoft Windows
 * implementations.
 *
 * @note If @a change_password_for is NULL, the change is performed on the
 * current principal. If @a change_password_for is non-null, the change is
 * performed on the principal name passed in @a change_password_for.
 *
 * The error code and strings are returned in @a result_code,
 * @a result_code_string and @a result_string.
 *
 * @sa krb5_set_password_using_ccache()
 *
 * @retval
 * 0  Success and result_code is set to #KRB5_KPASSWD_SUCCESS.
 * @return
 * Kerberos error codes.
 */
    /* *
 * Set a password for a principal using cached credentials.
 *
 * @param [in]  context              Library context
 * @param [in]  ccache               Credential cache
 * @param [in]  newpw                New password
 * @param [in]  change_password_for  Change the password for this principal
 * @param [out] result_code          Numeric error code from server
 * @param [out] result_code_string   String equivalent to @a result_code
 * @param [out] result_string        Data returned from the remote system
 *
 * This function uses the cached credentials from @a ccache to set the password
 * @a newpw for the principal @a change_password_for.  It implements RFC 3244
 * set password operation (interoperable with MS Windows implementations) using
 * the credential cache.
 *
 * The error code and strings are returned in @a result_code,
 * @a result_code_string and @a result_string.
 *
 * @note If @a change_password_for is set to NULL, the change is performed on
 * the default principal in @a ccache. If @a change_password_for is non null,
 * the change is performed on the specified principal.
 *
 * @sa krb5_set_password()
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Get a result message for changing or setting a password.
 *
 * @param [in]  context            Library context
 * @param [in]  server_string      Data returned from the remote system
 * @param [out] message_out        A message displayable to the user
 *
 * This function processes the @a server_string returned in the @a
 * result_string parameter of krb5_change_password(), krb5_set_password(), and
 * related functions, and returns a displayable string.  If @a server_string
 * contains Active Directory structured policy information, it will be
 * converted into human-readable text.
 *
 * Use krb5_free_string() to free @a message_out when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 *
 * @version New in 1.11
 */
    /* *
 * Retrieve configuration profile from the context.
 *
 * @param [in]  context         Library context
 * @param [out] profile         Pointer to data read from a configuration file
 *
 * This function creates a new @a profile object that reflects profile
 * in the supplied @a context.
 *
 * The @a profile object may be freed with profile_release() function.
 * See profile.h and profile API for more details.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
    /* * @deprecated Replaced by krb5_get_init_creds_password().*/
    /* * @deprecated Replaced by krb5_get_init_creds(). */
    /* * @deprecated Replaced by krb5_get_init_creds_keytab(). */
    /* KRB5_DEPRECATED */
    /* *
 * Parse and decrypt a @c KRB_AP_REQ message.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     inbuf          AP-REQ message to be parsed
 * @param [in]     server         Matching principal for server, or NULL to
 *                                allow any principal in keytab
 * @param [in]     keytab         Key table, or NULL to use the default
 * @param [out]    ap_req_options If non-null, the AP-REQ flags on output
 * @param [out]    ticket         If non-null, ticket from the AP-REQ message
 *
 * This function parses, decrypts and verifies a AP-REQ message from @a inbuf
 * and stores the authenticator in @a auth_context.
 *
 * If a keyblock was specified in @a auth_context using
 * krb5_auth_con_setuseruserkey(), that key is used to decrypt the ticket in
 * AP-REQ message and @a keytab is ignored.  In this case, @a server should be
 * specified as a complete principal name to allow for proper transited-path
 * checking and replay cache selection.
 *
 * Otherwise, the decryption key is obtained from @a keytab, or from the
 * default keytab if it is NULL.  In this case, @a server may be a complete
 * principal name, a matching principal (see krb5_sname_match()), or NULL to
 * match any principal name.  The keys tried against the encrypted part of the
 * ticket are determined as follows:
 *
 * - If @a server is a complete principal name, then its entry in @a keytab is
 *   tried.
 * - Otherwise, if @a keytab is iterable, then all entries in @a keytab which
 *   match @a server are tried.
 * - Otherwise, the server principal in the ticket must match @a server, and
 *   its entry in @a keytab is tried.
 *
 * The client specified in the decrypted authenticator must match the client
 * specified in the decrypted ticket.
 *
 * If the @a remote_addr field of @a auth_context is set, the request must come
 * from that address.
 *
 * If a replay cache handle is provided in the @a auth_context, the
 * authenticator and ticket are verified against it.  If no conflict is found,
 * the new authenticator is then stored in the replay cache of @a auth_context.
 *
 * Various other checks are performed on the decoded data, including
 * cross-realm policy, clockskew, and ticket validation times.
 *
 * On success the authenticator, subkey, and remote sequence number of the
 * request are stored in @a auth_context. If the #AP_OPTS_MUTUAL_REQUIRED
 * bit is set, the local sequence number is XORed with the remote sequence
 * number in the request.
 *
 * Use krb5_free_ticket() to free @a ticket when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve a service key from a key table.
 *
 * @param [in]  context     Library context
 * @param [in]  keyprocarg  Name of a key table (NULL to use default name)
 * @param [in]  principal   Service principal
 * @param [in]  vno         Key version number (0 for highest available)
 * @param [in]  enctype     Encryption type (0 for any type)
 * @param [out] key         Service key from key table
 *
 * Open and search the specified key table for the entry identified by @a
 * principal, @a enctype, and @a vno. If no key is found, return an error code.
 *
 * The default key table is used, unless @a keyprocarg is non-null.
 * @a keyprocarg designates a specific key table.
 *
 * Use krb5_free_keyblock() to free @a key when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return Kerberos error code if not found or @a keyprocarg is invalid.
 */
    /* *
 * Format a @c KRB-SAFE message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  userdata        User data in the message
 * @param [out] der_out         Formatted @c KRB-SAFE buffer
 * @param [out] rdata_out       Replay data. Specify NULL if not needed
 *
 * This function creates an integrity protected @c KRB-SAFE message
 * using data supplied by the application.
 *
 * Fields in @a auth_context specify the checksum type, the keyblock that
 * can be used to seed the checksum, full addresses (host and port) for
 * the sender and receiver, and @ref KRB5_AUTH_CONTEXT flags.
 *
 * The local address in @a auth_context must be set, and is used to form the
 * sender address used in the KRB-SAFE message.  The remote address is
 * optional; if specified, it will be used to form the receiver address used in
 * the message.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, a
 * timestamp is included in the KRB-SAFE message, and an entry for the message
 * is entered in an in-memory replay cache to detect if the message is
 * reflected by an attacker.  If #KRB5_AUTH_CONTEXT_DO_TIME is not set, no
 * replay cache is used.  If #KRB5_AUTH_CONTEXT_RET_TIME is set in @a
 * auth_context, a timestamp is included in the KRB-SAFE message and is stored
 * in @a rdata_out.
 *
 * If either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the @a auth_context local sequence number is included in the
 * KRB-SAFE message and then incremented.  If #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the sequence number used is stored in @a rdata_out.
 *
 * Use krb5_free_data_contents() to free @a der_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Format a @c KRB-PRIV message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  userdata        User data for @c KRB-PRIV message
 * @param [out] der_out         Formatted @c KRB-PRIV message
 * @param [out] rdata_out       Replay data (NULL if not needed)
 *
 * This function is similar to krb5_mk_safe(), but the message is encrypted and
 * integrity-protected, not just integrity-protected.
 *
 * The local address in @a auth_context must be set, and is used to form the
 * sender address used in the KRB-PRIV message.  The remote address is
 * optional; if specified, it will be used to form the receiver address used in
 * the message.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, a
 * timestamp is included in the KRB-PRIV message, and an entry for the message
 * is entered in an in-memory replay cache to detect if the message is
 * reflected by an attacker.  If #KRB5_AUTH_CONTEXT_DO_TIME is not set, no
 * replay cache is used.  If #KRB5_AUTH_CONTEXT_RET_TIME is set in @a
 * auth_context, a timestamp is included in the KRB-PRIV message and is stored
 * in @a rdata_out.
 *
 * If either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the @a auth_context local sequence number is included in the
 * KRB-PRIV message and then incremented.  If #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the sequence number used is stored in @a rdata_out.
 *
 * Use krb5_free_data_contents() to free @a der_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Client function for @c sendauth protocol.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     fd             File descriptor that describes network socket
 * @param [in]     appl_version   Application protocol version to be matched
 *                                with the receiver's application version
 * @param [in]     client         Client principal
 * @param [in]     server         Server principal
 * @param [in]     ap_req_options @ref AP_OPTS options
 * @param [in]     in_data        Data to be sent to the server
 * @param [in]     in_creds       Input credentials, or NULL to use @a ccache
 * @param [in]     ccache         Credential cache
 * @param [out]    error          If non-null, contains KRB_ERROR message
 *                                returned from server
 * @param [out]    rep_result     If non-null and @a ap_req_options is
 *                                #AP_OPTS_MUTUAL_REQUIRED, contains the result
 *                                of mutual authentication exchange
 * @param [out]    out_creds      If non-null, the retrieved credentials
 *
 * This function performs the client side of a sendauth/recvauth exchange by
 * sending and receiving messages over @a fd.
 *
 * Credentials may be specified in three ways:
 *
 * @li If @a in_creds is NULL, credentials are obtained with
 * krb5_get_credentials() using the principals @a client and @a server.  @a
 * server must be non-null; @a client may NULL to use the default principal of
 * @a ccache.
 *
 * @li If @a in_creds is non-null, but does not contain a ticket, credentials
 * for the exchange are obtained with krb5_get_credentials() using @a in_creds.
 * In this case, the values of @a client and @a server are unused.
 *
 * @li If @a in_creds is a complete credentials structure, it used directly.
 * In this case, the values of @a client, @a server, and @a ccache are unused.
 *
 * If the server is using a different application protocol than that specified
 * in @a appl_version, an error will be returned.
 *
 * Use krb5_free_creds() to free @a out_creds, krb5_free_ap_rep_enc_part() to
 * free @a rep_result, and krb5_free_error() to free @a error when they are no
 * longer needed.
 *
 * @sa krb5_recvauth()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Server function for @a sendauth protocol.
 *
 * @param [in]     context      Library context
 * @param [in,out] auth_context Pre-existing or newly created auth context
 * @param [in]     fd           File descriptor
 * @param [in]     appl_version Application protocol version to be matched
 *                              against the client's application version
 * @param [in]     server       Server principal (NULL for any in @a keytab)
 * @param [in]     flags        Additional specifications
 * @param [in]     keytab       Key table containing service keys
 * @param [out]    ticket       Ticket (NULL if not needed)
 *
 * This function performs the server side of a sendauth/recvauth exchange by
 * sending and receiving messages over @a fd.
 *
 * Use krb5_free_ticket() to free @a ticket when it is no longer needed.
 *
 * @sa krb5_sendauth()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Server function for @a sendauth protocol with version parameter.
 *
 * @param [in]     context      Library context
 * @param [in,out] auth_context Pre-existing or newly created auth context
 * @param [in]     fd           File descriptor
 * @param [in]     server       Server principal (NULL for any in @a keytab)
 * @param [in]     flags        Additional specifications
 * @param [in]     keytab       Decryption key
 * @param [out]    ticket       Ticket (NULL if not needed)
 * @param [out]    version      sendauth protocol version (NULL if not needed)
 *
 * This function is similar to krb5_recvauth() with the additional output
 * information place into @a version.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Format a @c KRB-CRED message for an array of credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  creds           Null-terminated array of credentials
 * @param [out] der_out         Encoded credentials
 * @param [out] rdata_out       Replay cache information (NULL if not needed)
 *
 * This function takes an array of credentials @a creds and formats
 * a @c KRB-CRED message @a der_out to pass to krb5_rd_cred().
 *
 * The local and remote addresses in @a auth_context are optional; if either is
 * specified, they are used to form the sender and receiver addresses in the
 * KRB-CRED message.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, an entry
 * for the message is entered in an in-memory replay cache to detect if the
 * message is reflected by an attacker.  If #KRB5_AUTH_CONTEXT_DO_TIME is not
 * set, no replay cache is used.  If #KRB5_AUTH_CONTEXT_RET_TIME is set in @a
 * auth_context, the timestamp used for the KRB-CRED message is stored in @a
 * rdata_out.
 *
 * If either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the @a auth_context local sequence number is included in the
 * KRB-CRED message and then incremented.  If #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the sequence number used is stored in @a rdata_out.
 *
 * Use krb5_free_data_contents() to free @a der_out when it is no longer
 * needed.
 *
 * The message will be encrypted using the send subkey of @a auth_context if it
 * is present, or the session key otherwise.  If neither key is present, the
 * credentials will not be encrypted, and the message should only be sent over
 * a secure channel.  No replay cache entry is used in this case.
 *
 * @retval
 *  0 Success
 * @retval
 *  ENOMEM Insufficient memory
 * @retval
 *   KRB5_RC_REQUIRED Message replay detection requires @a rcache parameter
 * @return
 * Kerberos error codes
 */
    /* *
 * Format a @c KRB-CRED message for a single set of credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  creds           Pointer to credentials
 * @param [out] der_out         Encoded credentials
 * @param [out] rdata_out       Replay cache data (NULL if not needed)
 *
 * This is a convenience function that calls krb5_mk_ncred() with a single set
 * of credentials.
 *
 * @retval
 * 0 Success
 * @retval
 *  ENOMEM Insufficient memory
 * @retval
 *  KRB5_RC_REQUIRED   Message replay detection requires @a rcache parameter
 * @return
 * Kerberos error codes
 */
    /* *
 * Read and validate a @c KRB-CRED message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  creddata        @c KRB-CRED message
 * @param [out] creds_out       Null-terminated array of forwarded credentials
 * @param [out] rdata_out       Replay data (NULL if not needed)
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.`
 *
 * @a creddata will be decrypted using the receiving subkey if it is present in
 * @a auth_context, or the session key if the receiving subkey is not present
 * or fails to decrypt the message.
 *
 * Use krb5_free_tgt_creds() to free @a creds_out when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Get a forwarded TGT and format a @c KRB-CRED message.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] rhost            Remote host
 * @param [in] client           Client principal of TGT
 * @param [in] server           Principal of server to receive TGT
 * @param [in] cc               Credential cache handle (NULL to use default)
 * @param [in] forwardable      Whether TGT should be forwardable
 * @param [out] outbuf          KRB-CRED message
 *
 * Get a TGT for use at the remote host @a rhost and format it into a KRB-CRED
 * message.  If @a rhost is NULL and @a server is of type #KRB5_NT_SRV_HST,
 * the second component of @a server will be used.
 *
 * @retval
 *  0 Success
 * @retval
 *   ENOMEM Insufficient memory
 * @retval
 *   KRB5_PRINC_NOMATCH Requested principal and ticket do not match
 * @retval
 *   KRB5_NO_TKT_SUPPLIED Request did not supply a ticket
 * @retval
 *   KRB5_CC_BADNAME Credential cache name or principal name malformed
 * @return
 * Kerberos error codes
 */
    /* *
 * Create and initialize an authentication context.
 *
 * @param [in]  context         Library context
 * @param [out] auth_context    Authentication context
 *
 * This function creates an authentication context to hold configuration and
 * state relevant to krb5 functions for authenticating principals and
 * protecting messages once authentication has occurred.
 *
 * By default, flags for the context are set to enable the use of the replay
 * cache (#KRB5_AUTH_CONTEXT_DO_TIME), but not sequence numbers.  Use
 * krb5_auth_con_setflags() to change the flags.
 *
 * The allocated @a auth_context must be freed with krb5_auth_con_free() when
 * it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Free a krb5_auth_context structure.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context to be freed
 *
 * This function frees an auth context allocated by krb5_auth_con_init().
 *
 * @retval 0  (always)
 */
    /* *
 * Set a flags field in a krb5_auth_context structure.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] flags            Flags bit mask
 *
 * Valid values for @a flags are:
 * @li #KRB5_AUTH_CONTEXT_DO_TIME Use timestamps
 * @li #KRB5_AUTH_CONTEXT_RET_TIME Save timestamps
 * @li #KRB5_AUTH_CONTEXT_DO_SEQUENCE Use sequence numbers
 * @li #KRB5_AUTH_CONTEXT_RET_SEQUENCE Save sequence numbers
 *
 * @retval 0 (always)
 */
    /* *
 * Retrieve flags from a krb5_auth_context structure.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] flags           Flags bit mask
 *
 * Valid values for @a flags are:
 * @li #KRB5_AUTH_CONTEXT_DO_TIME Use timestamps
 * @li #KRB5_AUTH_CONTEXT_RET_TIME Save timestamps
 * @li #KRB5_AUTH_CONTEXT_DO_SEQUENCE Use sequence numbers
 * @li #KRB5_AUTH_CONTEXT_RET_SEQUENCE Save sequence numbers
 *
 * @retval 0 (always)
 */
    /* *
 * Set a checksum callback in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] func             Checksum callback
 * @param [in] data             Callback argument
 *
 * Set a callback to obtain checksum data in krb5_mk_req().  The callback will
 * be invoked after the subkey and local sequence number are stored in @a
 * auth_context.
 *
 * @retval 0 (always)
 */
    /* *
 * Get the checksum callback from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] func            Checksum callback
 * @param [out] data            Callback argument
 *
 * @retval 0 (always)
 */
    /* *
 * Set the local and remote addresses in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] local_addr       Local address
 * @param [in] remote_addr      Remote address
 *
 * This function releases the storage assigned to the contents of the local and
 * remote addresses of @a auth_context and then sets them to @a local_addr and
 * @a remote_addr respectively.
 *
 * @sa krb5_auth_con_genaddrs()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve address fields from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] local_addr      Local address (NULL if not needed)
 * @param [out] remote_addr     Remote address (NULL if not needed)
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set local and remote port fields in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] local_port       Local port
 * @param [in] remote_port      Remote port
 *
 * This function releases the storage assigned to the contents of the local and
 * remote ports of @a auth_context and then sets them to @a local_port and @a
 * remote_port respectively.
 *
 * @sa krb5_auth_con_genaddrs()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set the session key in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] keyblock         User key
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the session key from an auth context as a keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] keyblock        Session key
 *
 * This function creates a keyblock containing the session key from @a
 * auth_context.  Use krb5_free_keyblock() to free @a keyblock when it is no
 * longer needed
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the session key from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] key             Session key
 *
 * This function sets @a key to the session key from @a auth_context.  Use
 * krb5_k_free_key() to release @a key when it is no longer needed.
 *
 * @retval 0 (always)
 */
    /* *
 * Retrieve the send subkey from an auth context as a keyblock.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] keyblock        Send subkey
 *
 * This function creates a keyblock containing the send subkey from @a
 * auth_context.  Use krb5_free_keyblock() to free @a keyblock when it is no
 * longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the send subkey from an auth context.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] key             Send subkey
 *
 * This function sets @a key to the send subkey from @a auth_context.  Use
 * krb5_k_free_key() to release @a key when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the receiving subkey from an auth context as a keyblock.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] keyblock        Receiving subkey
 *
 * This function creates a keyblock containing the receiving subkey from @a
 * auth_context.  Use krb5_free_keyblock() to free @a keyblock when it is no
 * longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the receiving subkey from an auth context as a keyblock.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] key             Receiving subkey
 *
 * This function sets @a key to the receiving subkey from @a auth_context.  Use
 * krb5_k_free_key() to release @a key when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set the send subkey in an auth context with a keyblock.
 *
 * @param [in] ctx              Library context
 * @param [in] ac               Authentication context
 * @param [in] keyblock         Send subkey
 *
 * This function sets the send subkey in @a ac to a copy of @a keyblock.
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
    /* *
 * Set the send subkey in an auth context.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] key             Send subkey
 *
 * This function sets the send subkey in @a ac to @a key, incrementing its
 * reference count.
 *
 * @version New in 1.9
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set the receiving subkey in an auth context with a keyblock.
 *
 * @param [in] ctx              Library context
 * @param [in] ac               Authentication context
 * @param [in] keyblock         Receiving subkey
 *
 * This function sets the receiving subkey in @a ac to a copy of @a keyblock.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set the receiving subkey in an auth context.
 *
 * @param [in] ctx              Library context
 * @param [in] ac               Authentication context
 * @param [in] key              Receiving subkey
 *
 * This function sets the receiving subkey in @a ac to @a key, incrementing its
 * reference count.
 *
 * @version New in 1.9
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* * @deprecated Replaced by krb5_auth_con_getsendsubkey(). */
    /* * @deprecated Replaced by krb5_auth_con_getrecvsubkey(). */
    /* *
 * Retrieve the local sequence number from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] seqnumber       Local sequence number
 *
 * Retrieve the local sequence number from @a auth_context and return it in @a
 * seqnumber.  The #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag must be set in @a
 * auth_context for this function to be useful.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the remote sequence number from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] seqnumber       Remote sequence number
 *
 * Retrieve the remote sequence number from @a auth_context and return it in @a
 * seqnumber.  The #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag must be set in @a
 * auth_context for this function to be useful.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Cause an auth context to use cipher state.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 *
 * Prepare @a auth_context to use cipher state when krb5_mk_priv() or
 * krb5_rd_priv() encrypt or decrypt data.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set the replay cache in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] rcache           Replay cache haddle
 *
 * This function sets the replay cache in @a auth_context to @a rcache.  @a
 * rcache will be closed when @a auth_context is freed, so the caller should
 * relinguish that responsibility.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the replay cache from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] rcache          Replay cache handle
 *
 * This function fetches the replay cache from @a auth_context.  The caller
 * should not close @a rcache.
 *
 * @retval 0 (always)
 */
    /* *
 * Retrieve the authenticator from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] authenticator   Authenticator
 *
 * Use krb5_free_authenticator() to free @a authenticator when it is no longer
 * needed.
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
    /* *
 * Set checksum type in an an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] cksumtype        Checksum type
 *
 * This function sets the checksum type in @a auth_context to be used by
 * krb5_mk_req() for the authenticator checksum.
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
    /*
 * end "func-proto.h"
 */
    /*
 * begin stuff from libos.h
 */
    /* *
 * @brief Read a password from keyboard input.
 *
 * @param [in]     context      Library context
 * @param [in]     prompt       First user prompt when reading password
 * @param [in]     prompt2      Second user prompt (NULL to prompt only once)
 * @param [out]    return_pwd   Returned password
 * @param [in,out] size_return  On input, maximum size of password; on output,
 *                              size of password read
 *
 * This function reads a password from keyboard input and stores it in @a
 * return_pwd.  @a size_return should be set by the caller to the amount of
 * storage space available in @a return_pwd; on successful return, it will be
 * set to the length of the password read.
 *
 * @a prompt is printed to the terminal, followed by ": ", and then a password
 * is read from the keyboard.
 *
 * If @a prompt2 is NULL, the password is read only once.  Otherwise, @a
 * prompt2 is printed to the terminal and a second password is read.  If the
 * two passwords entered are not identical, KRB5_LIBOS_BADPWDMATCH is returned.
 *
 * Echoing is turned off when the password is read.
 *
 * @retval
 *  0   Success
 * @return
 * Error in reading or verifying the password
 * @return
 * Kerberos error codes
 */
    /* *
 * Convert a principal name to a local name.
 *
 * @param [in]  context         Library context
 * @param [in]  aname           Principal name
 * @param [in]  lnsize_in       Space available in @a lname
 * @param [out] lname           Local name buffer to be filled in
 *
 * If @a aname does not correspond to any local account, KRB5_LNAME_NOTRANS is
 * returned.  If @a lnsize_in is too small for the local name,
 * KRB5_CONFIG_NOTENUFSPACE is returned.
 *
 * Local names, rather than principal names, can be used by programs that
 * translate to an environment-specific name (for example, a user account
 * name).
 *
 * @retval
 * 0  Success
 * @retval
 *  System errors
 * @return
 * Kerberos error codes
 */
    /* *
 * Get the Kerberos realm names for a host.
 *
 * @param [in]  context         Library context
 * @param [in]  host            Host name (or NULL)
 * @param [out] realmsp         Null-terminated list of realm names
 *
 * Fill in @a realmsp with a pointer to a null-terminated list of realm names.
 * If there are no known realms for the host, a list containing the referral
 * (empty) realm is returned.
 *
 * If @a host is NULL, the local host's realms are determined.
 *
 * Use krb5_free_host_realm() to release @a realmsp when it is no longer
 * needed.
 *
 * @retval
 *  0   Success
 * @retval
 *  ENOMEM  Insufficient memory
 * @return
 * Kerberos error codes
 */
    /* *
 *
 * @param [in] context           Library context
 * @param [in] hdata             Host name (or NULL)
 * @param [out] realmsp          Null-terminated list of realm names
 *
 * Fill in @a realmsp with a pointer to a null-terminated list of realm names
 * obtained through heuristics or insecure resolution methods which have lower
 * priority than KDC referrals.
 *
 * If @a host is NULL, the local host's realms are determined.
 *
 * Use krb5_free_host_realm() to release @a realmsp when it is no longer
 * needed.
 */
    /* *
 * Free the memory allocated by krb5_get_host_realm().
 *
 * @param [in] context          Library context
 * @param [in] realmlist        List of realm names to be released
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Determine if a principal is authorized to log in as a local user.
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal name
 * @param [in] luser            Local username
 *
 * Determine whether @a principal is authorized to log in as a local user @a
 * luser.
 *
 * @retval
 * TRUE Principal is authorized to log in as user; FALSE otherwise.
 */
    /* *
 * Generate auth context addresses from a connected socket.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] infd             Connected socket descriptor
 * @param [in] flags            Flags
 *
 * This function sets the local and/or remote addresses in @a auth_context
 * based on the local and remote endpoints of the socket @a infd.  The
 * following flags determine the operations performed:
 *
 * @li #KRB5_AUTH_CONTEXT_GENERATE_LOCAL_ADDR   Generate local address.
 * @li #KRB5_AUTH_CONTEXT_GENERATE_REMOTE_ADDR  Generate remote address.
 * @li #KRB5_AUTH_CONTEXT_GENERATE_LOCAL_FULL_ADDR  Generate local address and port.
 * @li #KRB5_AUTH_CONTEXT_GENERATE_REMOTE_FULL_ADDR Generate remote address and port.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set time offset field in a krb5_context structure.
 *
 * @param [in] context          Library context
 * @param [in] seconds          Real time, seconds portion
 * @param [in] microseconds     Real time, microseconds portion
 *
 * This function sets the time offset in @a context to the difference between
 * the system time and the real time as determined by @a seconds and @a
 * microseconds.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Return the time offsets from the os context.
 *
 * @param [in]  context         Library context
 * @param [out] seconds         Time offset, seconds portion
 * @param [out] microseconds    Time offset, microseconds portion
 *
 * This function returns the time offsets in @a context.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* str_conv.c */
/* *
 * Convert a string to an encryption type.
 *
 * @param [in]  string          String to convert to an encryption type
 * @param [out] enctypep        Encryption type
 *
 * @retval 0  Success; otherwise - EINVAL
 */
    /* *
 * Convert a string to a salt type.
 *
 * @param [in]  string          String to convert to an encryption type
 * @param [out] salttypep       Salt type to be filled in
 *
 * @retval 0  Success; otherwise - EINVAL
 */
    /* *
 * Convert a string to a checksum type.
 *
 * @param [in]  string          String to be converted
 * @param [out] cksumtypep      Checksum type to be filled in
 *
 * @retval 0  Success; otherwise - EINVAL
 */
    /* *
 * Convert a string to a timestamp.
 *
 * @param [in]  string          String to be converted
 * @param [out] timestampp      Pointer to timestamp
 *
 * @retval 0  Success; otherwise - EINVAL
 */
    /* *
 * Convert a string to a delta time value.
 *
 * @param [in]  string          String to be converted
 * @param [out] deltatp         Delta time to be filled in
 *
 * @retval 0  Success; otherwise - KRB5_DELTAT_BADFORMAT
 */
    /* *
 * Convert an encryption type to a string.
 *
 * @param [in]  enctype         Encryption type
 * @param [out] buffer          Buffer to hold encryption type string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert an encryption type to a name or alias.
 *
 * @param [in]  enctype         Encryption type
 * @param [in]  shortest        Flag
 * @param [out] buffer          Buffer to hold encryption type string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * If @a shortest is FALSE, this function returns the enctype's canonical name
 * (like "aes128-cts-hmac-sha1-96").  If @a shortest is TRUE, it return the
 * enctype's shortest alias (like "aes128-cts").
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a salt type to a string.
 *
 * @param [in]  salttype        Salttype to convert
 * @param [out] buffer          Buffer to receive the converted string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a checksum type to a string.
 *
 * @param [in]  cksumtype       Checksum type
 * @param [out] buffer          Buffer to hold converted checksum type
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a timestamp to a string.
 *
 * @param [in]  timestamp       Timestamp to convert
 * @param [out] buffer          Buffer to hold converted timestamp
 * @param [in]  buflen          Storage available in @a buffer
 *
 * The string is returned in the locale's appropriate date and time
 * representation.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a timestamp to a string, with optional output padding
 *
 * @param [in]  timestamp       Timestamp to convert
 * @param [out] buffer          Buffer to hold the converted timestamp
 * @param [in]  buflen          Length of buffer
 * @param [in]  pad             Optional value to pad @a buffer if converted
 *                              timestamp does not fill it
 *
 * If @a pad is not NULL, @a buffer is padded out to @a buflen - 1 characters
 * with the value of *@a pad.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a relative time value to a string.
 *
 * @param [in]  deltat          Relative time value to convert
 * @param [out] buffer          Buffer to hold time string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* The name of the Kerberos ticket granting service... and its size */
    /* flags for recvauth */
    /* initial ticket api functions */
    /* * Text for prompt used in prompter callback function.  */
    /* *< The prompt to show to the user */
    /* *< Boolean; informative prompt or hidden (e.g. PIN) */
    /* *< Must be allocated before call to  prompt routine */
    /* * Pointer to a prompter callback function. */
    /* *
 * Prompt user for password.
 *
 * @param [in] context          Library context
 * @param      data             Unused (callback argument)
 * @param [in] name             Name to output during prompt
 * @param [in] banner           Banner to output during prompt
 * @param [in] num_prompts      Number of prompts in @a prompts
 * @param [in] prompts          Array of prompts and replies
 *
 * This function is intended to be used as a prompter callback for
 * krb5_get_init_creds_password() or krb5_init_creds_init().
 *
 * Writes @a name and @a banner to stdout, each followed by a newline, then
 * writes each prompt field in the @a prompts array, followed by ": ", and sets
 * the reply field of the entry to a line of input read from stdin.  If the
 * hidden flag is set for a prompt, then terminal echoing is turned off when
 * input is read.
 *
 * @retval
 *  0   Success
 * @return
 * Kerberos error codes
 *
 */
    /* *
 * Long-term password responder question
 *
 * This question is asked when the long-term password is needed. It has no
 * challenge and the response is simply the password string.
 *
 * @version New in 1.11
 */
    /* *
 * OTP responder question
 *
 * The OTP responder question is asked when the KDC indicates that an OTP
 * value is required in order to complete the authentication.  The JSON format
 * of the challenge is:
 *
 *  @n {
 *  @n   "service": <string (optional)>,
 *  @n   "tokenInfo": [
 *  @n      {
 *  @n        "flags":     <number>,
 *  @n        "vendor":    <string (optional)>,
 *  @n        "challenge": <string (optional)>,
 *  @n        "length":    <number (optional)>,
 *  @n        "format":    <number (optional)>,
 *  @n        "tokenID":   <string (optional)>,
 *  @n        "algID":     <string (optional)>,
 *  @n      },
 *  @n      ...
 *  @n    ]
 *  @n  }
 *
 * The answer to the question MUST be JSON formatted:
 *
 * @n  {
 * @n    "tokeninfo": <number>,
 * @n    "value":     <string (optional)>,
 * @n    "pin":       <string (optional)>,
 * @n  }
 *
 * For more detail, please see RFC 6560.
 *
 * @version New in 1.11
 */
    /* *
 * These format constants identify the format of the token value.
 */
    /* *
 * This flag indicates that the token value MUST be collected.
 */
    /* *
 * This flag indicates that the PIN value MUST be collected.
 */
    /* *
 * This flag indicates that the token is now in re-synchronization mode with
 * the server.  The user is expected to reply with the next code displayed on
 * the token.
 */
    /* *
 * This flag indicates that the PIN MUST be returned as a separate item. This
 * flag only takes effect if KRB5_RESPONDER_OTP_FLAGS_COLLECT_PIN is set. If
 * this flag is not set, the responder may either concatenate PIN + token value
 * and store it as "value" in the answer or it may return them separately. If
 * they are returned separately, they will be concatenated internally.
 */
    /* *
 * PKINIT responder question
 *
 * The PKINIT responder question is asked when the client needs a password
 * that's being used to protect key information, and is formatted as a JSON
 * object.  A specific identity's flags value, if not zero, is the bitwise-OR
 * of one or more of the KRB5_RESPONDER_PKINIT_FLAGS_TOKEN_* flags defined
 * below, and possibly other flags to be added later.  Any resemblance to
 * similarly-named CKF_* values in the PKCS#11 API should not be depended on.
 *
 *  @n {
 *  @n     identity <string> : flags <number>,
 *  @n     ...
 *  @n }
 *
 * The answer to the question MUST be JSON formatted:
 *
 *  @n {
 *  @n     identity <string> : password <string>,
 *  @n     ...
 *  @n }
 *
 * @version New in 1.12
 */
    /* *
 * This flag indicates that an incorrect PIN was supplied at least once since
 * the last time the correct PIN was supplied.
 */
    /* *
 * This flag indicates that supplying an incorrect PIN will cause the token to
 * lock itself.
 */
    /* *
 * This flag indicates that the user PIN is locked, and you can't log in to the
 * token with it.
 */
    /* *
 * A container for a set of preauthentication questions and answers
 *
 * A responder context is supplied by the krb5 authentication system to a @ref
 * krb5_responder_fn callback.  It contains a list of questions and can receive
 * answers.  Questions contained in a responder context can be listed using
 * krb5_responder_list_questions(), retrieved using
 * krb5_responder_get_challenge(), or answered using
 * krb5_responder_set_answer().  The form of a question's challenge and
 * answer depend on the question name.
 *
 * @version New in 1.11
 */
    /* *
 * List the question names contained in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 *
 * Return a pointer to a null-terminated list of question names which are
 * present in @a rctx.  The pointer is an alias, valid only as long as the
 * lifetime of @a rctx, and should not be modified or freed by the caller.  A
 * question's challenge can be retrieved using krb5_responder_get_challenge()
 * and answered using krb5_responder_set_answer().
 *
 * @version New in 1.11
 */
    /* *
 * Retrieve the challenge data for a given question in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] question         Question name
 *
 * Return a pointer to a C string containing the challenge for @a question
 * within @a rctx, or NULL if the question is not present in @a rctx.  The
 * structure of the question depends on the question name, but will always be
 * printable UTF-8 text.  The returned pointer is an alias, valid only as long
 * as the lifetime of @a rctx, and should not be modified or freed by the
 * caller.
 *
 * @version New in 1.11
 */
    /* *
 * Answer a named question in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] question         Question name
 * @param [in] answer           The string to set (MUST be printable UTF-8)
 *
 * This function supplies an answer to @a question within @a rctx.  The
 * appropriate form of the answer depends on the question name.
 *
 * @retval EINVAL @a question is not present within @a rctx
 *
 * @version New in 1.11
 */
    /* *
 * Responder function for an initial credential exchange.
 *
 * @param [in] ctx              Library context
 * @param [in] data             Callback data
 * @param [in] rctx             Responder context
 *
 * A responder function is like a prompter function, but is used for handling
 * questions and answers as potentially complex data types.  Client
 * preauthentication modules will insert a set of named "questions" into
 * the responder context.  Each question may optionally contain a challenge.
 * This challenge is printable UTF-8, but may be an encoded value.  The
 * precise encoding and contents of the challenge are specific to the question
 * asked.  When the responder is called, it should answer all the questions it
 * understands.  Like the challenge, the answer MUST be printable UTF-8, but
 * may contain structured/encoded data formatted to the expected answer format
 * of the question.
 *
 * If a required question is unanswered, the prompter may be called.
 */
    /* -1 when not specified. */
    /* -1 when not specified. */
    /* *
 * Decode the KRB5_RESPONDER_QUESTION_OTP to a C struct.
 *
 * A convenience function which parses the KRB5_RESPONDER_QUESTION_OTP
 * question challenge data, making it available in native C.  The main feature
 * of this function is the ability to interact with OTP tokens without parsing
 * the JSON.
 *
 * The returned value must be passed to krb5_responder_otp_challenge_free() to
 * be freed.
 *
 * @param [in]  ctx             Library context
 * @param [in]  rctx            Responder context
 * @param [out] chl             Challenge structure
 *
 * @version New in 1.11
 */
    /* *
 * Answer the KRB5_RESPONDER_QUESTION_OTP question.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] ti               The index of the tokeninfo selected
 * @param [in] value            The value to set, or NULL for none
 * @param [in] pin              The pin to set, or NULL for none
 *
 * @version New in 1.11
 */
    /* *
 * Free the value returned by krb5_responder_otp_get_challenge().
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] chl              The challenge to free
 *
 * @version New in 1.11
 */
    /* 0 when not specified or not applicable. */
    /* *
 * Decode the KRB5_RESPONDER_QUESTION_PKINIT to a C struct.
 *
 * A convenience function which parses the KRB5_RESPONDER_QUESTION_PKINIT
 * question challenge data, making it available in native C.  The main feature
 * of this function is the ability to read the challenge without parsing
 * the JSON.
 *
 * The returned value must be passed to krb5_responder_pkinit_challenge_free()
 * to be freed.
 *
 * @param [in]  ctx             Library context
 * @param [in]  rctx            Responder context
 * @param [out] chl_out         Challenge structure
 *
 * @version New in 1.12
 */
    /* *
 * Answer the KRB5_RESPONDER_QUESTION_PKINIT question for one identity.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] identity         The identity for which a PIN is being supplied
 * @param [in] pin              The provided PIN, or NULL for none
 *
 * @version New in 1.12
 */
    /* *
 * Free the value returned by krb5_responder_pkinit_get_challenge().
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] chl              The challenge to free
 *
 * @version New in 1.12
 */
    /* * Store options for @c _krb5_get_init_creds */
    /* *
 * Allocate a new initial credential options structure.
 *
 * @param [in]  context         Library context
 * @param [out] opt             New options structure
 *
 * This function is the preferred way to create an options structure for
 * getting initial credentials, and is required to make use of certain options.
 * Use krb5_get_init_creds_opt_free() to free @a opt when it is no longer
 * needed.
 *
 * @retval 0 - Success; Kerberos errors otherwise.
 */
    /* *
 * Free initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure to free
 *
 * @sa krb5_get_init_creds_opt_alloc()
 */
    /* * @deprecated Use krb5_get_init_creds_opt_alloc() instead. */
    /* *
 * Set the ticket lifetime in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] tkt_life         Ticket lifetime
 */
    /* *
 * Set the ticket renewal lifetime in initial credential options.
 *
 * @param [in] opt              Pointer to @a options field
 * @param [in] renew_life       Ticket renewal lifetime
 */
    /* *
 * Set or unset the forwardable flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] forwardable      Whether credentials should be forwardable
 */
    /* *
 * Set or unset the proxiable flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] proxiable        Whether credentials should be proxiable
 */
    /* *
 * Set or unset the canonicalize flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] canonicalize     Whether to canonicalize client principal
 */
    /* *
 * Set or unset the anonymous flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] anonymous        Whether to make an anonymous request
 *
 * This function may be used to request anonymous credentials from the KDC by
 * setting @a anonymous to non-zero.  Note that anonymous credentials are only
 * a request; clients must verify that credentials are anonymous if that is a
 * requirement.
 */
    /* *
 * Set allowable encryption types in initial credential options.
 *
 * @param [in] opt               Options structure
 * @param [in] etype_list        Array of encryption types
 * @param [in] etype_list_length Length of @a etype_list
 */
    /* *
 * Set address restrictions in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] addresses        Null-terminated array of addresses
 */
    /* *
 * Set preauthentication types in initial credential options.
 *
 * @param [in] opt                 Options structure
 * @param [in] preauth_list        Array of preauthentication types
 * @param [in] preauth_list_length Length of @a preauth_list
 *
 * This function can be used to perform optimistic preauthentication when
 * getting initial credentials, in combination with
 * krb5_get_init_creds_opt_set_salt() and krb5_get_init_creds_opt_set_pa().
 */
    /* *
 * Set salt for optimistic preauthentication in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] salt             Salt data
 *
 * When getting initial credentials with a password, a salt string it used to
 * convert the password to a key.  Normally this salt is obtained from the
 * first KDC reply, but when performing optimistic preauthentication, the
 * client may need to supply the salt string with this function.
 */
    /* *
 * Set or unset change-password-prompt flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] prompt           Whether to prompt to change password
 *
 * This flag is on by default.  It controls whether
 * krb5_get_init_creds_password() will react to an expired-password error by
 * prompting for a new password and attempting to change the old one.
 */
    /* * Generic preauth option attribute/value pairs */
    /* *
 * Supply options for preauthentication in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] attr             Preauthentication option name
 * @param [in] value            Preauthentication option value
 *
 * This function allows the caller to supply options for preauthentication.
 * The values of @a attr and @a value are supplied to each preauthentication
 * module available within @a context.
 */
    /* *
 * Set location of FAST armor ccache in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] fast_ccache_name Credential cache name
 *
 * Sets the location of a credential cache containing an armor ticket to
 * protect an initial credential exchange using the FAST protocol extension.
 *
 * In version 1.7, setting an armor ccache requires that FAST be used for the
 * exchange.  In version 1.8 or later, setting the armor ccache causes FAST to
 * be used if the KDC supports it; krb5_get_init_creds_opt_set_fast_flags()
 * must be used to require that FAST be used.
 */
    /* *
 * Set FAST armor cache in initial credential options.
 *
 * @param [in] context           Library context
 * @param [in] opt               Options
 * @param [in] ccache            Credential cache handle
 *
 * This function is similar to krb5_get_init_creds_opt_set_fast_ccache_name(),
 * but uses a credential cache handle instead of a name.
 *
 * @version New in 1.9
 */
    /* *
 * Set an input credential cache in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] ccache           Credential cache handle
 *
 * If an input credential cache is set, then the krb5_get_init_creds family of
 * APIs will read settings from it.  Setting an input ccache is desirable when
 * the application wishes to perform authentication in the same way (using the
 * same preauthentication mechanisms, and making the same non-security-
 * sensitive choices) as the previous authentication attempt, which stored
 * information in the passed-in ccache.
 *
 * @version New in 1.11
 */
    /* *
 * Set an output credential cache in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] ccache           Credential cache handle
 *
 * If an output credential cache is set, then the krb5_get_init_creds family of
 * APIs will write credentials to it.  Setting an output ccache is desirable
 * both because it simplifies calling code and because it permits the
 * krb5_get_init_creds APIs to write out configuration information about the
 * realm to the ccache.
 */
    /* *
 * @brief Ask the KDC to include or not include a PAC in the ticket
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] req_pac          Whether to request a PAC or not
 *
 * If this option is set, the AS request will include a PAC-REQUEST pa-data
 * item explicitly asking the KDC to either include or not include a privilege
 * attribute certificate in the ticket authorization data.  By default, no
 * request is made; typically the KDC will default to including a PAC if it
 * supports them.
 *
 * @version New in 1.15
 */
    /* *
 * Set FAST flags in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] flags            FAST flags
 *
 * The following flag values are valid:
 * @li #KRB5_FAST_REQUIRED - Require FAST to be used
 *
 * @retval
 * 0 - Success; Kerberos errors otherwise.
 */
    /* *
 * Retrieve FAST flags from initial credential options.
 *
 * @param [in]  context         Library context
 * @param [in]  opt             Options
 * @param [out] out_flags       FAST flags
 *
 * @retval
 * 0 - Success; Kerberos errors otherwise.
 */
    /* Fast flags*/
    /* *< Require KDC to support FAST*/
    /* *
 * Set an expiration callback in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] cb               Callback function
 * @param [in] data             Callback argument
 *
 * Set a callback to receive password and account expiration times.
 *
 * This option only applies to krb5_get_init_creds_password().  @a cb will be
 * invoked if and only if credentials are successfully acquired.  The callback
 * will receive the @a context from the krb5_get_init_creds_password() call and
 * the @a data argument supplied with this API.  The remaining arguments should
 * be interpreted as follows:
 *
 * If @a is_last_req is true, then the KDC reply contained last-req entries
 * which unambiguously indicated the password expiration, account expiration,
 * or both.  (If either value was not present, the corresponding argument will
 * be 0.)  Furthermore, a non-zero @a password_expiration should be taken as a
 * suggestion from the KDC that a warning be displayed.
 *
 * If @a is_last_req is false, then @a account_expiration will be 0 and @a
 * password_expiration will contain the expiration time of either the password
 * or account, or 0 if no expiration time was indicated in the KDC reply.  The
 * callback should independently decide whether to display a password
 * expiration warning.
 *
 * Note that @a cb may be invoked even if credentials are being acquired for
 * the kadmin/changepw service in order to change the password.  It is the
 * caller's responsibility to avoid displaying a password expiry warning in
 * this case.
 *
 * @warning Setting an expire callback with this API will cause
 * krb5_get_init_creds_password() not to send password expiry warnings to the
 * prompter, as it ordinarily may.
 *
 * @version New in 1.9
 */
    /* *
 * Set the responder function in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] responder        Responder function
 * @param [in] data             Responder data argument
 *
 * @version New in 1.11
 */
    /* *
 * Get initial credentials using a password.
 *
 * @param [in]  context         Library context
 * @param [out] creds           New credentials
 * @param [in]  client          Client principal
 * @param [in]  password        Password (or NULL)
 * @param [in]  prompter        Prompter function
 * @param [in]  data            Prompter callback data
 * @param [in]  start_time      Time when ticket becomes valid (0 for now)
 * @param [in]  in_tkt_service  Service name of initial credentials (or NULL)
 * @param [in]  k5_gic_options  Initial credential options
 *
 * This function requests KDC for an initial credentials for @a client using @a
 * password.  If @a password is NULL, a password will be prompted for using @a
 * prompter if necessary.  If @a in_tkt_service is specified, it is parsed as a
 * principal name (with the realm ignored) and used as the service principal
 * for the request; otherwise the ticket-granting service is used.
 *
 * @sa krb5_verify_init_creds()
 *
 * @retval
 *  0    Success
 * @retval
 *  EINVAL Invalid argument
 * @retval
 *  KRB5_KDC_UNREACH Cannot contact any KDC for requested realm
 * @retval
 *  KRB5_PREAUTH_FAILED Generic Pre-athentication failure
 * @retval
 *  KRB5_LIBOS_PWDINTR Password read interrupted
 * @retval
 *  KRB5_REALM_CANT_RESOLVE Cannot resolve network address for KDC in requested realm
 * @retval
 *  KRB5KDC_ERR_KEY_EXP Password has expired
 * @retval
 *  KRB5_LIBOS_BADPWDMATCH Password mismatch
 * @retval
 *  KRB5_CHPW_PWDNULL New password cannot be zero length
 * @retval
 *  KRB5_CHPW_FAIL Password change failed
 * @return
 * Kerberos error codes
 */
    /* *
 * Retrieve enctype, salt and s2kparams from KDC
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal whose information is requested
 * @param [in]  opt             Initial credential options
 * @param [out] enctype_out     The enctype chosen by KDC
 * @param [out] salt_out        Salt returned from KDC
 * @param [out] s2kparams_out   String-to-key parameters returned from KDC
 *
 * Send an initial ticket request for @a principal and extract the encryption
 * type, salt type, and string-to-key parameters from the KDC response.  If the
 * KDC provides no etype-info, set @a enctype_out to @c ENCTYPE_NULL and set @a
 * salt_out and @a s2kparams_out to empty.  If the KDC etype-info provides no
 * salt, compute the default salt and place it in @a salt_out.  If the KDC
 * etype-info provides no string-to-key parameters, set @a s2kparams_out to
 * empty.
 *
 * @a opt may be used to specify options which affect the initial request, such
 * as request encryption types or a FAST armor cache (see
 * krb5_get_init_creds_opt_set_etype_list() and
 * krb5_get_init_creds_opt_set_fast_ccache_name()).
 *
 * Use krb5_free_data_contents() to free @a salt_out and @a s2kparams_out when
 * they are no longer needed.
 *
 * @version New in 1.17
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
    /* *< More responses needed */
    /* *
 * Free an initial credentials context.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 */
    /* *
 * Acquire credentials using an initial credentials context.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 *
 * This function synchronously obtains credentials using a context created by
 * krb5_init_creds_init().  On successful return, the credentials can be
 * retrieved with krb5_init_creds_get_creds().
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve acquired credentials from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] creds           Acquired credentials
 *
 * This function copies the acquired initial credentials from @a ctx into @a
 * creds, after the successful completion of krb5_init_creds_get() or
 * krb5_init_creds_step().  Use krb5_free_cred_contents() to free @a creds when
 * it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Get the last error from KDC from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] error           Error from KDC, or NULL if none was received
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Create a context for acquiring initial credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  client          Client principal to get initial creds for
 * @param [in]  prompter        Prompter callback
 * @param [in]  data            Prompter callback argument
 * @param [in]  start_time      Time when credentials become valid (0 for now)
 * @param [in]  options         Options structure (NULL for default)
 * @param [out] ctx             New initial credentials context
 *
 * This function creates a new context for acquiring initial credentials.  Use
 * krb5_init_creds_free() to free @a ctx when it is no longer needed.
 *
 * Any subsequent calls to krb5_init_creds_step(), krb5_init_creds_get(), or
 * krb5_init_creds_free() for this initial credentials context must use the
 * same @a context argument as the one passed to this function.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Specify a keytab to use for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] keytab           Key table handle
 *
 * This function supplies a keytab containing the client key for an initial
 * credentials request.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Get the next KDC request for acquiring initial credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [in]  in              KDC response (empty on the first call)
 * @param [out] out             Next KDC request
 * @param [out] realm           Realm for next KDC request
 * @param [out] flags           Output flags
 *
 * This function constructs the next KDC request in an initial credential
 * exchange, allowing the caller to control the transport of KDC requests and
 * replies.  On the first call, @a in should be set to an empty buffer; on
 * subsequent calls, it should be set to the KDC's reply to the previous
 * request.
 *
 * If more requests are needed, @a flags will be set to
 * #KRB5_INIT_CREDS_STEP_FLAG_CONTINUE and the next request will be placed in
 * @a out.  If no more requests are needed, @a flags will not contain
 * #KRB5_INIT_CREDS_STEP_FLAG_CONTINUE and @a out will be empty.
 *
 * If this function returns @c KRB5KRB_ERR_RESPONSE_TOO_BIG, the caller should
 * transmit the next request using TCP rather than UDP.  If this function
 * returns any other error, the initial credential exchange has failed.
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set a password for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] password         Password
 *
 * This function supplies a password to be used to construct the client key for
 * an initial credentials request.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Specify a service principal for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] service          Service principal string
 *
 * This function supplies a service principal string to acquire initial
 * credentials for instead of the default krbtgt service.  @a service is parsed
 * as a principal name; any realm part is ignored.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve ticket times from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] times           Ticket times for acquired credentials
 *
 * The initial credentials context must have completed obtaining credentials
 * via either krb5_init_creds_get() or krb5_init_creds_step().
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Create a context to get credentials from a KDC's Ticket Granting Service.
 *
 * @param[in]  context          Library context
 * @param[in]  ccache           Credential cache handle
 * @param[in]  creds            Input credentials
 * @param[in]  options          @ref KRB5_GC options for this request.
 * @param[out] ctx              New TGS request context
 *
 * This function prepares to obtain credentials matching @a creds, either by
 * retrieving them from @a ccache or by making requests to ticket-granting
 * services beginning with a ticket-granting ticket for the client principal's
 * realm.
 *
 * The resulting TGS acquisition context can be used asynchronously with
 * krb5_tkt_creds_step() or synchronously with krb5_tkt_creds_get().  See also
 * krb5_get_credentials() for synchronous use.
 *
 * Use krb5_tkt_creds_free() to free @a ctx when it is no longer needed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Synchronously obtain credentials using a TGS request context.
 *
 * @param[in] context           Library context
 * @param[in] ctx               TGS request context
 *
 * This function synchronously obtains credentials using a context created by
 * krb5_tkt_creds_init().  On successful return, the credentials can be
 * retrieved with krb5_tkt_creds_get_creds().
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve acquired credentials from a TGS request context.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[out] creds            Acquired credentials
 *
 * This function copies the acquired initial credentials from @a ctx into @a
 * creds, after the successful completion of krb5_tkt_creds_get() or
 * krb5_tkt_creds_step().  Use krb5_free_cred_contents() to free @a creds when
 * it is no longer needed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Free a TGS request context.
 *
 * @param[in]  context  Library context
 * @param[in]  ctx      TGS request context
 *
 * @version New in 1.9
 */
    /* *< More responses needed */
    /* *
 * Get the next KDC request in a TGS exchange.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[in]  in               KDC response (empty on the first call)
 * @param[out] out              Next KDC request
 * @param[out] realm            Realm for next KDC request
 * @param[out] flags            Output flags
 *
 * This function constructs the next KDC request for a TGS exchange, allowing
 * the caller to control the transport of KDC requests and replies.  On the
 * first call, @a in should be set to an empty buffer; on subsequent calls, it
 * should be set to the KDC's reply to the previous request.
 *
 * If more requests are needed, @a flags will be set to
 * #KRB5_TKT_CREDS_STEP_FLAG_CONTINUE and the next request will be placed in @a
 * out.  If no more requests are needed, @a flags will not contain
 * #KRB5_TKT_CREDS_STEP_FLAG_CONTINUE and @a out will be empty.
 *
 * If this function returns @c KRB5KRB_ERR_RESPONSE_TOO_BIG, the caller should
 * transmit the next request using TCP rather than UDP.  If this function
 * returns any other error, the TGS exchange has failed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve ticket times from a TGS request context.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[out] times            Ticket times for acquired credentials
 *
 * The TGS request context must have completed obtaining credentials via either
 * krb5_tkt_creds_get() or krb5_tkt_creds_step().
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Get initial credentials using a key table.
 *
 * @param [in]  context         Library context
 * @param [out] creds           New credentials
 * @param [in]  client          Client principal
 * @param [in]  arg_keytab      Key table handle
 * @param [in]  start_time      Time when ticket becomes valid (0 for now)
 * @param [in]  in_tkt_service  Service name of initial credentials (or NULL)
 * @param [in]  k5_gic_options  Initial credential options
 *
 * This function requests KDC for an initial credentials for @a client using a
 * client key stored in @a arg_keytab.  If @a in_tkt_service is specified, it
 * is parsed as a principal name (with the realm ignored) and used as the
 * service principal for the request; otherwise the ticket-granting service is
 * used.
 *
 * @sa krb5_verify_init_creds()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
    /* *< boolean */
    /* *
 * Initialize a credential verification options structure.
 *
 * @param [in] k5_vic_options   Verification options structure
 */
    /* *
 * Set whether credential verification is required.
 *
 * @param [in] k5_vic_options   Verification options structure
 * @param [in] ap_req_nofail    Whether to require successful verification
 *
 * This function determines how krb5_verify_init_creds() behaves if no keytab
 * information is available.  If @a ap_req_nofail is @c FALSE, verification
 * will be skipped in this case and krb5_verify_init_creds() will return
 * successfully.  If @a ap_req_nofail is @c TRUE, krb5_verify_init_creds() will
 * not return successfully unless verification can be performed.
 *
 * If this function is not used, the behavior of krb5_verify_init_creds() is
 * determined through configuration.
 */
    /* *
 * Verify initial credentials against a keytab.
 *
 * @param [in] context          Library context
 * @param [in] creds            Initial credentials to be verified
 * @param [in] server           Server principal (or NULL)
 * @param [in] keytab           Key table (NULL to use default keytab)
 * @param [in] ccache           Credential cache for fetched creds (or NULL)
 * @param [in] options          Verification options (NULL for default options)
 *
 * This function attempts to verify that @a creds were obtained from a KDC with
 * knowledge of a key in @a keytab, or the default keytab if @a keytab is NULL.
 * If @a server is provided, the highest-kvno key entry for that principal name
 * is used to verify the credentials; otherwise, all unique "host" service
 * principals in the keytab are tried.
 *
 * If the specified keytab does not exist, or is empty, or cannot be read, or
 * does not contain an entry for @a server, then credential verification may be
 * skipped unless configuration demands that it succeed.  The caller can
 * control this behavior by providing a verification options structure; see
 * krb5_verify_init_creds_opt_init() and
 * krb5_verify_init_creds_opt_set_ap_req_nofail().
 *
 * If @a ccache is NULL, any additional credentials fetched during the
 * verification process will be destroyed.  If @a ccache points to NULL, a
 * memory ccache will be created for the additional credentials and returned in
 * @a ccache.  If @a ccache points to a valid credential cache handle, the
 * additional credentials will be stored in that cache.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Get validated credentials from the KDC.
 *
 * @param [in]  context         Library context
 * @param [out] creds           Validated credentials
 * @param [in]  client          Client principal name
 * @param [in]  ccache          Credential cache
 * @param [in]  in_tkt_service  Server principal string (or NULL)
 *
 * This function gets a validated credential using a postdated credential from
 * @a ccache.  If @a in_tkt_service is specified, it is parsed (with the realm
 * part ignored) and used as the server principal of the credential;
 * otherwise, the ticket-granting service is used.
 *
 * If successful, the validated credential is placed in @a creds.
 *
 * @sa krb5_get_renewed_creds()
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_NO_2ND_TKT Request missing second ticket
 * @retval
 * KRB5_NO_TKT_SUPPLIED Request did not supply a ticket
 * @retval
 * KRB5_PRINC_NOMATCH Requested principal and ticket do not match
 * @retval
 * KRB5_KDCREP_MODIFIED KDC reply did not match expectations
 * @retval
 * KRB5_KDCREP_SKEW Clock skew too great in KDC reply
 * @return
 * Kerberos error codes
 */
    /* *
 * Get renewed credential from KDC using an existing credential.
 *
 * @param [in]  context         Library context
 * @param [out] creds           Renewed credentials
 * @param [in]  client          Client principal name
 * @param [in]  ccache          Credential cache
 * @param [in]  in_tkt_service  Server principal string (or NULL)
 *
 * This function gets a renewed credential using an existing one from @a
 * ccache.  If @a in_tkt_service is specified, it is parsed (with the realm
 * part ignored) and used as the server principal of the credential; otherwise,
 * the ticket-granting service is used.
 *
 * If successful, the renewed credential is placed in @a creds.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Decode an ASN.1-formatted ticket.
 *
 * @param [in]  code            ASN.1-formatted ticket
 * @param [out] rep             Decoded ticket information
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve a string value from the appdefaults section of krb5.conf.
 *
 * @param [in]  context         Library context
 * @param [in]  appname         Application name
 * @param [in]  realm           Realm name
 * @param [in]  option          Option to be checked
 * @param [in]  default_value   Default value to return if no match is found
 * @param [out] ret_value       String value of @a option
 *
 * This function gets the application defaults for @a option based on the given
 * @a appname and/or @a realm.
 *
 * @sa krb5_appdefault_boolean()
 */
    /* *
 * Retrieve a boolean value from the appdefaults section of krb5.conf.
 *
 * @param [in]  context         Library context
 * @param [in]  appname         Application name
 * @param [in]  realm           Realm name
 * @param [in]  option          Option to be checked
 * @param [in]  default_value   Default value to return if no match is found
 * @param [out] ret_value       Boolean value of @a option
 *
 * This function gets the application defaults for @a option based on the given
 * @a appname and/or @a realm.
 *
 * @sa krb5_appdefault_string()
 */
    /*
 * Prompter enhancements
 */
/* * Prompt for password */
    /* * Prompt for new password (during password change) */
    /* * Prompt for new password again */
    /* * Prompt for preauthentication data (such as an OTP value) */
    /* *
 * Get prompt types array from a context.
 *
 * @param [in] context          Library context
 *
 * @return
 * Pointer to an array of prompt types corresponding to the prompter's @a
 * prompts arguments.  Each type has one of the following values:
 *  @li #KRB5_PROMPT_TYPE_PASSWORD
 *  @li #KRB5_PROMPT_TYPE_NEW_PASSWORD
 *  @li #KRB5_PROMPT_TYPE_NEW_PASSWORD_AGAIN
 *  @li #KRB5_PROMPT_TYPE_PREAUTH
 */
    /* Error reporting */
/* *
 * Set an extended error message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] ...              printf(3) style parameters
 */
    /* *
 * Set an extended error message for an error code using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] args             List of vprintf(3) style arguments
 */
    /* *
 * Add a prefix to the message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] ...              printf(3) style parameters
 *
 * Format a message and prepend it to the current message for @a code.  The
 * prefix will be separated from the old message with a colon and space.
 */
    /* *
 * Add a prefix to the message for an error code using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] args             List of vprintf(3) style arguments
 *
 * This function is similar to krb5_prepend_error_message(), but uses a
 * va_list instead of variadic arguments.
 */
    /* *
 * Add a prefix to a different error code's message.
 *
 * @param [in] ctx              Library context
 * @param [in] old_code         Previous error code
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] ...              printf(3) style parameters
 *
 * Format a message and prepend it to the message for @a old_code.  The prefix
 * will be separated from the old message with a colon and space.  Set the
 * resulting message as the extended error message for @a code.
 */
    /* *
 * Add a prefix to a different error code's message using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] old_code         Previous error code
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] args             List of vprintf(3) style arguments
 *
 * This function is similar to krb5_wrap_error_message(), but uses a
 * va_list instead of variadic arguments.
 */
    /* *
 * Copy the most recent extended error message from one context to another.
 *
 * @param [in] dest_ctx         Library context to copy message to
 * @param [in] src_ctx          Library context with current message
 */
    /* *
 * Get the (possibly extended) error message for a code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 *
 * The behavior of krb5_get_error_message() is only defined the first time it
 * is called after a failed call to a krb5 function using the same context, and
 * only when the error code passed in is the same as that returned by the krb5
 * function.
 *
 * This function never returns NULL, so its result may be used unconditionally
 * as a C string.
 *
 * The string returned by this function must be freed using
 * krb5_free_error_message()
 *
 * @note Future versions may return the same string for the second
 * and following calls.
 */
    /* *
 * Free an error message generated by krb5_get_error_message().
 *
 * @param [in] ctx              Library context
 * @param [in] msg              Pointer to error message
 */
    /* *
 * Clear the extended error message in a context.
 *
 * @param [in] ctx              Library context
 *
 * This function unsets the extended error message in a context, to ensure that
 * it is not mistakenly applied to another occurrence of the same error code.
 */
    /* *
 * Unwrap authorization data.
 *
 * @param [in]  context         Library context
 * @param [in]  type            @ref KRB5_AUTHDATA type of @a container
 * @param [in]  container       Authorization data to be decoded
 * @param [out] authdata        List of decoded authorization data
 *
 * @sa krb5_encode_authdata_container()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Wrap authorization data in a container.
 *
 * @param [in]  context         Library context
 * @param [in]  type            @ref KRB5_AUTHDATA type of @a container
 * @param [in]  authdata        List of authorization data to be encoded
 * @param [out] container       List of encoded authorization data
 *
 * The result is returned in @a container as a single-element list.
 *
 * @sa krb5_decode_authdata_container()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /*
 * AD-KDCIssued
 */
/* *
 * Encode and sign AD-KDCIssued authorization data.
 *
 * @param [in]  context         Library context
 * @param [in]  key             Session key
 * @param [in]  issuer          The name of the issuing principal
 * @param [in]  authdata        List of authorization data to be signed
 * @param [out] ad_kdcissued    List containing AD-KDCIssued authdata
 *
 * This function wraps a list of authorization data entries @a authdata in an
 * AD-KDCIssued container (see RFC 4120 section 5.2.6.2) signed with @a key.
 * The result is returned in @a ad_kdcissued as a single-element list.
 */
    /* *
 * Unwrap and verify AD-KDCIssued authorization data.
 *
 * @param [in] context          Library context
 * @param [in] key              Session key
 * @param [in] ad_kdcissued     AD-KDCIssued authorization data to be unwrapped
 * @param [out] issuer          Name of issuing principal (or NULL)
 * @param [out] authdata        Unwrapped list of authorization data
 *
 * This function unwraps an AD-KDCIssued authdatum (see RFC 4120 section
 * 5.2.6.2) and verifies its signature against @a key.  The issuer field of the
 * authdatum element is returned in @a issuer, and the unwrapped list of
 * authdata is returned in @a authdata.
 */
    /*
 * Windows PAC
 */
    /* Microsoft defined types of data */
    /* *< Logon information */
    /* *< Credentials information */
    /* *< Server checksum */
    /* *< KDC checksum */
    /* *< Client name and ticket info */
    /* *< Constrained delegation info */
    /* *< User principal name and DNS info */
    /* * PAC data structure to convey authorization information */
    /* *
 * Add a buffer to a PAC handle.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] type             Buffer type
 * @param [in] data             contents
 *
 * This function adds a buffer of type @a type and contents @a data to @a pac
 * if there isn't already a buffer of this type present.
 *
 * The valid values of @a type is one of the following:
 * @li #KRB5_PAC_LOGON_INFO         -  Logon information
 * @li #KRB5_PAC_CREDENTIALS_INFO   -  Credentials information
 * @li #KRB5_PAC_SERVER_CHECKSUM    -  Server checksum
 * @li #KRB5_PAC_PRIVSVR_CHECKSUM   -  KDC checksum
 * @li #KRB5_PAC_CLIENT_INFO        -  Client name and ticket information
 * @li #KRB5_PAC_DELEGATION_INFO    -  Constrained delegation information
 * @li #KRB5_PAC_UPN_DNS_INFO       -  User principal name and DNS information
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Free a PAC handle.
 *
 * @param [in] context         Library context
 * @param [in] pac             PAC to be freed
 *
 * This function frees the contents of @a pac and the structure itself.
 */
    /* *
 * Retrieve a buffer value from a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  type            Type of buffer to retrieve
 * @param [out] data            Buffer value
 *
 * Use krb5_free_data_contents() to free @a data when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Return an array of buffer types in a PAC handle.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [out] len             Number of entries in @a types
 * @param [out] types           Array of buffer types
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Create an empty Privilege Attribute Certificate (PAC) handle.
 *
 * @param [in]  context         Library context
 * @param [out] pac             New PAC handle
 *
 * Use krb5_pac_free() to free @a pac when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Unparse an encoded PAC into a new handle.
 *
 * @param [in]  context         Library context
 * @param [in]  ptr             PAC buffer
 * @param [in]  len             Length of @a ptr
 * @param [out] pac             PAC handle
 *
 * Use krb5_pac_free() to free @a pac when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Verify a PAC.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] authtime         Expected timestamp
 * @param [in] principal        Expected principal name (or NULL)
 * @param [in] server           Key to validate server checksum (or NULL)
 * @param [in] privsvr          Key to validate KDC checksum (or NULL)
 *
 * This function validates @a pac against the supplied @a server, @a privsvr,
 * @a principal and @a authtime.  If @a principal is NULL, the principal and
 * authtime are not verified.  If @a server or @a privsvr is NULL, the
 * corresponding checksum is not verified.
 *
 * If successful, @a pac is marked as verified.
 *
 * @note A checksum mismatch can occur if the PAC was copied from a cross-realm
 * TGT by an ignorant KDC; also macOS Server Open Directory (as of 10.6)
 * generates PACs with no server checksum at all.  One should consider not
 * failing the whole authentication because of this reason, but, instead,
 * treating the ticket as if it did not contain a PAC or marking the PAC
 * information as non-verified.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Verify a PAC, possibly from a specified realm.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] authtime         Expected timestamp
 * @param [in] principal        Expected principal name (or NULL)
 * @param [in] server           Key to validate server checksum (or NULL)
 * @param [in] privsvr          Key to validate KDC checksum (or NULL)
 * @param [in] with_realm       If true, expect the realm of @a principal
 *
 * This function is similar to krb5_pac_verify(), but adds a parameter
 * @a with_realm.  If @a with_realm is true, the PAC_CLIENT_INFO field is
 * expected to include the realm of @a principal as well as the name.  This
 * flag is necessary to verify PACs in cross-realm S4U2Self referral TGTs.
 *
 * @version New in 1.17
 */
    /* *
 * Sign a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  authtime        Expected timestamp
 * @param [in]  principal       Expected principal name (or NULL)
 * @param [in]  server_key      Key for server checksum
 * @param [in]  privsvr_key     Key for KDC checksum
 * @param [out] data            Signed PAC encoding
 *
 * This function signs @a pac using the keys @a server_key and @a privsvr_key
 * and returns the signed encoding in @a data.  @a pac is modified to include
 * the server and KDC checksum buffers.  Use krb5_free_data_contents() to free
 * @a data when it is no longer needed.
 *
 * @version New in 1.10
 */
    /* *
 * Sign a PAC, possibly with a specified realm.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  authtime        Expected timestamp
 * @param [in]  principal       Principal name (or NULL)
 * @param [in]  server_key      Key for server checksum
 * @param [in]  privsvr_key     Key for KDC checksum
 * @param [in]  with_realm      If true, include the realm of @a principal
 * @param [out] data            Signed PAC encoding
 *
 * This function is similar to krb5_pac_sign(), but adds a parameter
 * @a with_realm.  If @a with_realm is true, the PAC_CLIENT_INFO field of the
 * signed PAC will include the realm of @a principal as well as the name.  This
 * flag is necessary to generate PACs for cross-realm S4U2Self referrals.
 *
 * @version New in 1.17
 */
    /*
 * Read client information from a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [out] authtime_out    Authentication timestamp (NULL if not needed)
 * @param [out] princname_out   Client account name
 *
 * Read the PAC_CLIENT_INFO buffer in @a pac.  Place the client account name as
 * a string in @a princname_out.  If @a authtime_out is not NULL, place the
 * initial authentication timestamp in @a authtime_out.
 *
 * @retval 0 on success, ENOENT if no PAC_CLIENT_INFO buffer is present in @a
 * pac, ERANGE if the buffer contains invalid lengths.
 *
 * @version New in 1.18
 */
    /* *
 * Allow the appplication to override the profile's allow_weak_crypto setting.
 *
 * @param [in] context          Library context
 * @param [in] enable           Boolean flag
 *
 * This function allows an application to override the allow_weak_crypto
 * setting.  It is primarily for use by aklog.
 *
 * @retval 0  (always)
 */
    /* *
 * A wrapper for passing information to a @c krb5_trace_callback.
 *
 * Currently, it only contains the formatted message as determined
 * the the format string and arguments of the tracing macro, but it
 * may be extended to contain more fields in the future.
 */
    /* *
 * Specify a callback function for trace events.
 *
 * @param [in] context          Library context
 * @param [in] fn               Callback function
 * @param [in] cb_data          Callback data
 *
 * Specify a callback for trace events occurring in krb5 operations performed
 * within @a context.  @a fn will be invoked with @a context as the first
 * argument, @a cb_data as the last argument, and a pointer to a
 * krb5_trace_info as the second argument.  If the trace callback is reset via
 * this function or @a context is destroyed, @a fn will be invoked with a NULL
 * second argument so it can clean up @a cb_data.  Supply a NULL value for @a
 * fn to disable trace callbacks within @a context.
 *
 * @note This function overrides the information passed through the
 * @a KRB5_TRACE environment variable.
 *
 * @version New in 1.9
 *
 * @return Returns KRB5_TRACE_NOSUPP if tracing is not supported in the library
 * (unless @a fn is NULL).
 */
    /* *
 * Specify a file name for directing trace events.
 *
 * @param [in] context          Library context
 * @param [in] filename         File name
 *
 * Open @a filename for appending (creating it, if necessary) and set up a
 * callback to write trace events to it.
 *
 * @note This function overrides the information passed through the
 * @a KRB5_TRACE environment variable.
 *
 * @version New in 1.9
 *
 * @retval KRB5_TRACE_NOSUPP Tracing is not supported in the library.
 */
    /* *
 * Hook function for inspecting or modifying messages sent to KDCs.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  realm           The realm the message will be sent to
 * @param [in]  message         The original message to be sent to the KDC
 * @param [out] new_message_out Optional replacement message to be sent
 * @param [out] reply_out       Optional synthetic reply
 *
 * If the hook function returns an error code, the KDC communication will be
 * aborted and the error code will be returned to the library operation which
 * initiated the communication.
 *
 * If the hook function sets @a reply_out, @a message will not be sent to the
 * KDC, and the given reply will used instead.
 *
 * If the hook function sets @a new_message_out, the given message will be sent
 * to the KDC in place of @a message.
 *
 * If the hook function returns successfully without setting either output,
 * @a message will be sent to the KDC normally.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_message_out or @a reply_out, to ensure that it can be freed correctly
 * by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
    /* *
 * Hook function for inspecting or overriding KDC replies.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  code            Status of KDC communication
 * @param [in]  realm           The realm the reply was received from
 * @param [in]  message         The message sent to the realm's KDC
 * @param [in]  reply           The reply received from the KDC
 * @param [out] new_reply_out   Optional replacement reply
 *
 * If @a code is zero, @a reply contains the reply received from the KDC.  The
 * hook function may return an error code to simulate an error, may synthesize
 * a different reply by setting @a new_reply_out, or may simply return
 * successfully to do nothing.
 *
 * If @a code is non-zero, KDC communication failed and @a reply should be
 * ignored.  The hook function may return @a code or a different error code, or
 * may synthesize a reply by setting @a new_reply_out and return successfully.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_reply_out, to ensure that it can be freed correctly by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
    #[c2rust::src_loc = "8510:1"]
    pub type krb5_post_recv_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_error_code, _: *const krb5_data,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    /* per Kerberos v5 protocol spec */
    /* not yet in the spec... */
    /* macros to determine if a type is a local type */
    /*
 * end "hostaddr.h"
 */
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
    /*
 * begin "encryption.h"
 */
    /* * Exposed contents of a key. */
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
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* checksum type */
        /* This file is generated, please don't edit it directly.  */
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* General definitions for Kerberos version 5. */
/*
 * Copyright 1989, 1990, 1995, 2001, 2003, 2007, 2011 by the Massachusetts
 * Institute of Technology.  All Rights Reserved.
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
        /* * @defgroup KRB5_H krb5 library API
 * @{
 */
        /* By default, do not expose deprecated interfaces. */
        /* !KRB5_CONFIG__ */
        /* for *_MAX */
        /* from profile.h */
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        /* *
 * Generate pseudo-random bytes.
 *
 * @param [in]  context         Library context
 * @param [out] data            Random data
 *
 * Fills in @a data with bytes from the PRNG used by krb5 crypto operations.
 * The caller must preinitialize @a data and allocate the desired amount of
 * space.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "800:1"]
        pub fn krb5_c_random_make_octets(context: krb5_context,
                                         data: *mut krb5_data)
         -> krb5_error_code;
        /* *
 * Compute a checksum (operates on keyblock).
 *
 * @param [in]  context         Library context
 * @param [in]  cksumtype       Checksum type (0 for mandatory type)
 * @param [in]  key             Encryption key for a keyed checksum
 * @param [in]  usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]  input           Input data
 * @param [out] cksum           Generated checksum
 *
 * This function computes a checksum of type @a cksumtype over @a input, using
 * @a key if the checksum type is a keyed checksum.  If @a cksumtype is 0 and
 * @a key is non-null, the checksum type will be the mandatory-to-implement
 * checksum type for the key's encryption type.  The actual checksum key will
 * be derived from @a key and @a usage if key derivation is specified for the
 * checksum type.  The newly created @a cksum must be released by calling
 * krb5_free_checksum_contents() when it is no longer needed.
 *
 * @note This function is similar to krb5_k_make_checksum(), but operates
 * on keyblock @a key.
 *
 * @sa krb5_c_verify_checksum()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "911:1"]
        pub fn krb5_c_make_checksum(context: krb5_context,
                                    cksumtype: krb5_cksumtype,
                                    key: *const krb5_keyblock,
                                    usage: krb5_keyusage,
                                    input: *const krb5_data,
                                    cksum: *mut krb5_checksum)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4699:1"]
        pub fn krb5_free_checksum_contents(context: krb5_context,
                                           val: *mut krb5_checksum);
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
    #[inline]
    #[c2rust::src_loc = "2274:1"]
    pub unsafe extern "C" fn alloc_data(mut data: *mut krb5_data,
                                        mut len: libc::c_uint)
     -> krb5_error_code {
        let mut ptr: *mut libc::c_char =
            calloc(if len > 0 as libc::c_int as libc::c_uint {
                       len
                   } else { 1 as libc::c_int as libc::c_uint } as
                       libc::c_ulong, 1 as libc::c_int as libc::c_ulong) as
                *mut libc::c_char;
        if ptr.is_null() { return 12 as libc::c_int }
        (*data).magic = -(1760647422 as libc::c_long) as krb5_magic;
        (*data).data = ptr;
        (*data).length = len;
        return 0 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "2251:1"]
    pub unsafe extern "C" fn make_data(mut data: *mut libc::c_void,
                                       mut len: libc::c_uint) -> krb5_data {
        let mut d: krb5_data =
            krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
        d.magic = -(1760647422 as libc::c_long) as krb5_magic;
        d.data = data as *mut libc::c_char;
        d.length = len;
        return d;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_data, krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::stdlib_h::calloc;
    extern "C" {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krad.h:30"]
pub mod krad_h {
    #[c2rust::src_loc = "60:1"]
    pub type krad_attrset = krad_attrset_st;
    #[c2rust::src_loc = "61:1"]
    pub type krad_packet = krad_packet_st;
    #[c2rust::src_loc = "63:1"]
    pub type krad_code = libc::c_uchar;
    #[c2rust::src_loc = "64:1"]
    pub type krad_attr = libc::c_uchar;
    #[c2rust::src_loc = "76:1"]
    pub type krad_packet_iter_cb
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: krb5_boolean)
                   -> *const krad_packet>;
    use super::krad_packet_st;
    use super::krb5_h::{krb5_boolean, krb5_data};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "60:16"]
        pub type krad_attrset_st;
        #[no_mangle]
        #[c2rust::src_loc = "136:1"]
        pub fn krad_attrset_get(set: *const krad_attrset, type_0: krad_attr,
                                indx: size_t) -> *const krb5_data;
        #[no_mangle]
        #[c2rust::src_loc = "120:1"]
        pub fn krad_attrset_free(set: *mut krad_attrset);
    }
    /* KRAD_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:30"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
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
#[c2rust::header_src = "/usr/include/time.h:30"]
pub mod time_h {
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn time(__timer: *mut time_t) -> time_t;
    }
}
#[c2rust::header_src = "/usr/include/string.h:30"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/bits/byteswap.h:30"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
        return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
                    |
                    (__bsx as libc::c_int & 0xff as libc::c_int) <<
                        8 as libc::c_int) as __uint16_t;
    }
    use super::types_h::__uint16_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krad/internal.h:30"]
pub mod internal_h {
    use super::krad_h::krad_attrset;
    use super::stddef_h::size_t;
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_data};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn kr_attrset_encode(set: *const krad_attrset,
                                 secret: *const libc::c_char,
                                 auth: *const libc::c_uchar,
                                 outbuf: *mut libc::c_uchar,
                                 outlen: *mut size_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn kr_attrset_decode(ctx: krb5_context, in_0: *const krb5_data,
                                 secret: *const libc::c_char,
                                 auth: *const libc::c_uchar,
                                 set: *mut *mut krad_attrset)
         -> krb5_error_code;
    }
    /* INTERNAL_H_ */
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __time_t, __ssize_t};
pub use self::sys_types_h::ssize_t;
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, store_16_be,
                              load_16_be};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_enctype, krb5_cksumtype,
                       krb5_keyusage, krb5_flags, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       _krb5_checksum, krb5_checksum, _profile_t,
                       krb5_c_random_make_octets, krb5_c_make_checksum,
                       krb5_free_checksum_contents};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, alloc_data, make_data,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::krad_h::{krad_attrset, krad_packet, krad_code, krad_attr,
                       krad_packet_iter_cb, krad_attrset_st, krad_attrset_get,
                       krad_attrset_free};
use self::stdlib_h::{calloc, free};
use self::errno_h::__errno_location;
use self::time_h::time;
use self::string_h::{strlen, memcmp, memset, memcpy};
pub use self::byteswap_h::__bswap_16;
use self::internal_h::{kr_attrset_encode, kr_attrset_decode};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "56:8"]
pub struct krad_packet_st {
    pub buffer: [libc::c_char; 4096],
    pub attrset: *mut krad_attrset,
    pub pkt: krb5_data,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krad/packet.c - Packet functions for libkrad */
/*
 * Copyright 2013 Red Hat, Inc.  All rights reserved.
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
#[c2rust::src_loc = "36:1"]
pub type uchar = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "62:9"]
pub struct idmap {
    pub x: [uchar; 32],
}
/* Ensure the map is empty. */
#[inline]
#[c2rust::src_loc = "67:1"]
unsafe extern "C" fn idmap_init(mut map: *mut idmap) {
    memset(map as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<idmap>() as libc::c_ulong);
}
/* Set an id as already allocated. */
#[inline]
#[c2rust::src_loc = "74:1"]
unsafe extern "C" fn idmap_set(mut map: *mut idmap, mut id: uchar) {
    (*map).x[(id as libc::c_int / 8 as libc::c_int) as usize] =
        ((*map).x[(id as libc::c_int / 8 as libc::c_int) as usize] as
             libc::c_int |
             (1 as libc::c_int) << id as libc::c_int % 8 as libc::c_int) as
            uchar;
}
/* Determine whether or not an id is used. */
#[inline]
#[c2rust::src_loc = "81:1"]
unsafe extern "C" fn idmap_isset(mut map: *const idmap, mut id: uchar)
 -> krb5_boolean {
    return ((*map).x[(id as libc::c_int / 8 as libc::c_int) as usize] as
                libc::c_int &
                (1 as libc::c_int) << id as libc::c_int % 8 as libc::c_int !=
                0 as libc::c_int) as libc::c_int as krb5_boolean;
}
/* Find an unused id starting the search at the value specified in id.
 * NOTE: For optimal security, the initial value of id should be random. */
#[inline]
#[c2rust::src_loc = "89:1"]
unsafe extern "C" fn idmap_find(mut map: *const idmap, mut id: *mut uchar)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut i: krb5_int16 = 0;
    i = *id as krb5_int16;
    loop  {
        if !(i as libc::c_int >= 0 as libc::c_int &&
                 i as libc::c_int <=
                     127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
           {
            current_block = 7502529970979898288;
            break ;
        }
        if idmap_isset(map, i as uchar) == 0 {
            current_block = 6349932073687642035;
            break ;
        }
        if *id as libc::c_int % 2 as libc::c_int == 0 as libc::c_int {
            i += 1
        } else { i -= 1 };
    }
    match current_block {
        7502529970979898288 => {
            i = *id as krb5_int16;
            loop  {
                if !(i as libc::c_int >= 0 as libc::c_int &&
                         i as libc::c_int <=
                             127 as libc::c_int * 2 as libc::c_int +
                                 1 as libc::c_int) {
                    current_block = 17216689946888361452;
                    break ;
                }
                if idmap_isset(map, i as uchar) == 0 {
                    current_block = 6349932073687642035;
                    break ;
                }
                if *id as libc::c_int % 2 as libc::c_int == 1 as libc::c_int {
                    i += 1
                } else { i -= 1 };
            }
            match current_block {
                6349932073687642035 => { }
                _ => { return 34 as libc::c_int }
            }
        }
        _ => { }
    }
    *id = i as uchar;
    return 0 as libc::c_int;
}
/* Generate size bytes of random data into the buffer. */
#[inline]
#[c2rust::src_loc = "112:1"]
unsafe extern "C" fn randomize(mut ctx: krb5_context,
                               mut buffer: *mut libc::c_void,
                               mut size: libc::c_uint) -> krb5_error_code {
    let mut rdata: krb5_data = make_data(buffer, size);
    return krb5_c_random_make_octets(ctx, &mut rdata);
}
/* Generate a radius packet id. */
#[c2rust::src_loc = "120:1"]
unsafe extern "C" fn id_generate(mut ctx: krb5_context,
                                 mut cb: krad_packet_iter_cb,
                                 mut data: *mut libc::c_void,
                                 mut id: *mut uchar) -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut tmp: *const krad_packet = 0 as *const krad_packet;
    let mut used: idmap = idmap{x: [0; 32],};
    let mut i: uchar = 0;
    retval =
        randomize(ctx, &mut i as *mut uchar as *mut libc::c_void,
                  ::std::mem::size_of::<uchar>() as libc::c_ulong as
                      libc::c_uint);
    if retval != 0 as libc::c_int {
        if cb.is_some() {
            Some(cb.expect("non-null function pointer")).expect("non-null function pointer")(data,
                                                                                             1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 krb5_boolean);
        }
        return retval
    }
    if cb.is_some() {
        idmap_init(&mut used);
        tmp =
            Some(cb.expect("non-null function pointer")).expect("non-null function pointer")(data,
                                                                                             0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 krb5_boolean);
        while !tmp.is_null() {
            idmap_set(&mut used,
                      *(*tmp).pkt.data.offset(1 as libc::c_int as isize) as
                          uchar);
            tmp =
                Some(cb.expect("non-null function pointer")).expect("non-null function pointer")(data,
                                                                                                 0
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     krb5_boolean)
        }
        retval = idmap_find(&mut used, &mut i);
        if retval != 0 as libc::c_int { return retval }
    }
    *id = i;
    return 0 as libc::c_int;
}
/* Generate a random authenticator field. */
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn auth_generate_random(mut ctx: krb5_context,
                                          mut rauth: *mut uchar)
 -> krb5_error_code {
    let mut trunctime: krb5_ui_4 = 0;
    let mut currtime: time_t = 0;
    /* Get the least-significant four bytes of the current time. */
    currtime = time(0 as *mut time_t);
    if currtime == -(1 as libc::c_int) as time_t {
        return *__errno_location()
    }
    trunctime = currtime as krb5_ui_4;
    memcpy(rauth as *mut libc::c_void,
           &mut trunctime as *mut krb5_ui_4 as *const libc::c_void,
           ::std::mem::size_of::<krb5_ui_4>() as libc::c_ulong);
    /* Randomize the rest of the buffer. */
    return randomize(ctx,
                     rauth.offset(::std::mem::size_of::<krb5_ui_4>() as
                                      libc::c_ulong as isize) as
                         *mut libc::c_void,
                     ((20 as libc::c_int - 4 as libc::c_int) as
                          libc::c_ulong).wrapping_sub(::std::mem::size_of::<krb5_ui_4>()
                                                          as libc::c_ulong) as
                         libc::c_uint);
}
/* Generate a response authenticator field. */
#[c2rust::src_loc = "169:1"]
unsafe extern "C" fn auth_generate_response(mut ctx: krb5_context,
                                            mut secret: *const libc::c_char,
                                            mut response: *const krad_packet,
                                            mut auth: *const uchar,
                                            mut rauth: *mut uchar)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut hash: krb5_checksum =
        krb5_checksum{magic: 0,
                      checksum_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    /* Allocate the temporary buffer. */
    retval =
        alloc_data(&mut data,
                   ((*response).pkt.length as
                        libc::c_ulong).wrapping_add(strlen(secret)) as
                       libc::c_uint);
    if retval != 0 as libc::c_int { return retval }
    /* Encoded RADIUS packet with the request's
     * authenticator and the secret at the end. */
    memcpy(data.data as *mut libc::c_void,
           (*response).pkt.data as *const libc::c_void,
           (*response).pkt.length as libc::c_ulong);
    memcpy(data.data.offset(4 as libc::c_int as isize) as *mut libc::c_void,
           auth as *const libc::c_void,
           (20 as libc::c_int - 4 as libc::c_int) as libc::c_ulong);
    memcpy(data.data.offset((*response).pkt.length as isize) as
               *mut libc::c_void, secret as *const libc::c_void,
           strlen(secret));
    /* Hash it. */
    retval =
        krb5_c_make_checksum(ctx, 0x7 as libc::c_int,
                             0 as *const krb5_keyblock, 0 as libc::c_int,
                             &mut data, &mut hash);
    free(data.data as *mut libc::c_void);
    if retval != 0 as libc::c_int { return retval }
    memcpy(rauth as *mut libc::c_void, hash.contents as *const libc::c_void,
           (20 as libc::c_int - 4 as libc::c_int) as libc::c_ulong);
    krb5_free_checksum_contents(ctx, &mut hash);
    return 0 as libc::c_int;
}
/* Create a new packet. */
#[c2rust::src_loc = "202:1"]
unsafe extern "C" fn packet_new() -> *mut krad_packet {
    let mut pkt: *mut krad_packet = 0 as *mut krad_packet;
    pkt =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krad_packet>() as libc::c_ulong) as
            *mut krad_packet;
    if pkt.is_null() { return 0 as *mut krad_packet }
    (*pkt).pkt =
        make_data((*pkt).buffer.as_mut_ptr() as *mut libc::c_void,
                  ::std::mem::size_of::<[libc::c_char; 4096]>() as
                      libc::c_ulong as libc::c_uint);
    return pkt;
}
/* Set the attrset object by decoding the packet. */
#[c2rust::src_loc = "216:1"]
unsafe extern "C" fn packet_set_attrset(mut ctx: krb5_context,
                                        mut secret: *const libc::c_char,
                                        mut pkt: *mut krad_packet)
 -> krb5_error_code {
    let mut tmp: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    tmp =
        make_data(&mut *(*pkt).pkt.data.offset(20 as libc::c_int as isize) as
                      *mut libc::c_char as *mut libc::c_uchar as
                      *mut libc::c_void,
                  (*pkt).pkt.length.wrapping_sub(20 as libc::c_int as
                                                     libc::c_uint));
    return kr_attrset_decode(ctx, &mut tmp, secret,
                             &mut *(*pkt).pkt.data.offset(4 as libc::c_int as
                                                              isize) as
                                 *mut libc::c_char as *mut uchar,
                             &mut (*pkt).attrset);
}
#[no_mangle]
#[c2rust::src_loc = "225:1"]
pub unsafe extern "C" fn krad_packet_bytes_needed(mut buffer:
                                                      *const krb5_data)
 -> ssize_t {
    let mut len: size_t = 0;
    if (*buffer).length < 4 as libc::c_int as libc::c_uint {
        return (4 as libc::c_int as
                    libc::c_uint).wrapping_sub((*buffer).length) as ssize_t
    }
    len =
        load_16_be(&mut *(*buffer).data.offset(2 as libc::c_int as isize) as
                       *mut libc::c_char as *const libc::c_void) as size_t;
    if len > 4096 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int) as ssize_t
    }
    return if (*buffer).length as libc::c_ulong > len {
               0 as libc::c_int as libc::c_ulong
           } else { len.wrapping_sub((*buffer).length as libc::c_ulong) } as
               ssize_t;
}
#[no_mangle]
#[c2rust::src_loc = "240:1"]
pub unsafe extern "C" fn krad_packet_free(mut pkt: *mut krad_packet) {
    if !pkt.is_null() { krad_attrset_free((*pkt).attrset); }
    free(pkt as *mut libc::c_void);
}
/* Create a new request packet. */
#[no_mangle]
#[c2rust::src_loc = "249:1"]
pub unsafe extern "C" fn krad_packet_new_request(mut ctx: krb5_context,
                                                 mut secret:
                                                     *const libc::c_char,
                                                 mut code: krad_code,
                                                 mut set: *const krad_attrset,
                                                 mut cb: krad_packet_iter_cb,
                                                 mut data: *mut libc::c_void,
                                                 mut request:
                                                     *mut *mut krad_packet)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut pkt: *mut krad_packet = 0 as *mut krad_packet;
    let mut id: uchar = 0;
    let mut attrset_len: size_t = 0;
    pkt = packet_new();
    if pkt.is_null() {
        if cb.is_some() {
            Some(cb.expect("non-null function pointer")).expect("non-null function pointer")(data,
                                                                                             1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 krb5_boolean);
        }
        return 12 as libc::c_int
    }
    /* Generate the ID. */
    retval = id_generate(ctx, cb, data, &mut id);
    if !(retval != 0 as libc::c_int) {
        *(&mut *(*pkt).pkt.data.offset(1 as libc::c_int as isize) as
              *mut libc::c_char as *mut uchar) = id;
        /* Generate the authenticator. */
        retval =
            auth_generate_random(ctx,
                                 &mut *(*pkt).pkt.data.offset(4 as libc::c_int
                                                                  as isize) as
                                     *mut libc::c_char as *mut uchar);
        if !(retval != 0 as libc::c_int) {
            /* Encode the attributes. */
            retval =
                kr_attrset_encode(set, secret,
                                  &mut *(*pkt).pkt.data.offset(4 as
                                                                   libc::c_int
                                                                   as isize)
                                      as *mut libc::c_char as *mut uchar,
                                  &mut *(*pkt).pkt.data.offset(20 as
                                                                   libc::c_int
                                                                   as isize)
                                      as *mut libc::c_char as
                                      *mut libc::c_uchar, &mut attrset_len);
            if !(retval != 0 as libc::c_int) {
                /* Set the code, ID and length. */
                (*pkt).pkt.length =
                    attrset_len.wrapping_add(20 as libc::c_int as
                                                 libc::c_ulong) as
                        libc::c_uint;
                *(&mut *(*pkt).pkt.data.offset(0 as libc::c_int as isize) as
                      *mut libc::c_char as *mut krad_code) = code;
                store_16_be((*pkt).pkt.length,
                            &mut *(*pkt).pkt.data.offset(2 as libc::c_int as
                                                             isize) as
                                *mut libc::c_char as *mut libc::c_void);
                /* Copy the attrset for future use. */
                retval = packet_set_attrset(ctx, secret, pkt);
                if !(retval != 0 as libc::c_int) {
                    *request = pkt;
                    return 0 as libc::c_int
                }
            }
        }
    }
    free(pkt as *mut libc::c_void);
    return retval;
}
/* Create a new request packet. */
#[no_mangle]
#[c2rust::src_loc = "302:1"]
pub unsafe extern "C" fn krad_packet_new_response(mut ctx: krb5_context,
                                                  mut secret:
                                                      *const libc::c_char,
                                                  mut code: krad_code,
                                                  mut set:
                                                      *const krad_attrset,
                                                  mut request:
                                                      *const krad_packet,
                                                  mut response:
                                                      *mut *mut krad_packet)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut pkt: *mut krad_packet = 0 as *mut krad_packet;
    let mut attrset_len: size_t = 0;
    pkt = packet_new();
    if pkt.is_null() { return 12 as libc::c_int }
    /* Encode the attributes. */
    retval =
        kr_attrset_encode(set, secret,
                          &mut *(*request).pkt.data.offset(4 as libc::c_int as
                                                               isize) as
                              *mut libc::c_char as *mut uchar,
                          &mut *(*pkt).pkt.data.offset(20 as libc::c_int as
                                                           isize) as
                              *mut libc::c_char as *mut libc::c_uchar,
                          &mut attrset_len);
    if !(retval != 0 as libc::c_int) {
        /* Set the code, ID and length. */
        (*pkt).pkt.length =
            attrset_len.wrapping_add(20 as libc::c_int as libc::c_ulong) as
                libc::c_uint;
        *(&mut *(*pkt).pkt.data.offset(0 as libc::c_int as isize) as
              *mut libc::c_char as *mut krad_code) = code;
        *(&mut *(*pkt).pkt.data.offset(1 as libc::c_int as isize) as
              *mut libc::c_char as *mut uchar) =
            *(&mut *(*request).pkt.data.offset(1 as libc::c_int as isize) as
                  *mut libc::c_char as *mut uchar);
        store_16_be((*pkt).pkt.length,
                    &mut *(*pkt).pkt.data.offset(2 as libc::c_int as isize) as
                        *mut libc::c_char as *mut libc::c_void);
        /* Generate the authenticator. */
        retval =
            auth_generate_response(ctx, secret, pkt,
                                   &mut *(*request).pkt.data.offset(4 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                       as *mut libc::c_char as *mut uchar,
                                   &mut *(*pkt).pkt.data.offset(4 as
                                                                    libc::c_int
                                                                    as isize)
                                       as *mut libc::c_char as *mut uchar);
        if !(retval != 0 as libc::c_int) {
            /* Copy the attrset for future use. */
            retval = packet_set_attrset(ctx, secret, pkt);
            if !(retval != 0 as libc::c_int) {
                *response = pkt;
                return 0 as libc::c_int
            }
        }
    }
    free(pkt as *mut libc::c_void);
    return retval;
}
/* Decode a packet. */
#[c2rust::src_loc = "347:1"]
unsafe extern "C" fn decode_packet(mut ctx: krb5_context,
                                   mut secret: *const libc::c_char,
                                   mut buffer: *const krb5_data,
                                   mut pkt: *mut *mut krad_packet)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut tmp: *mut krad_packet = 0 as *mut krad_packet;
    let mut len: krb5_ui_2 = 0;
    tmp = packet_new();
    if tmp.is_null() {
        retval = 12 as libc::c_int
    } else {
        /* Ensure a proper message length. */
        retval =
            if (*buffer).length < 20 as libc::c_int as libc::c_uint {
                90 as libc::c_int
            } else { 0 as libc::c_int };
        if !(retval != 0 as libc::c_int) {
            len =
                load_16_be(&mut *(*buffer).data.offset(2 as libc::c_int as
                                                           isize) as
                               *mut libc::c_char as *const libc::c_void);
            retval =
                if (len as libc::c_int) < 20 as libc::c_int {
                    74 as libc::c_int
                } else { 0 as libc::c_int };
            if !(retval != 0 as libc::c_int) {
                retval =
                    if len as libc::c_uint > (*buffer).length ||
                           len as libc::c_uint > (*tmp).pkt.length {
                        74 as libc::c_int
                    } else { 0 as libc::c_int };
                if !(retval != 0 as libc::c_int) {
                    /* Copy over the buffer. */
                    (*tmp).pkt.length = len as libc::c_uint;
                    memcpy((*tmp).pkt.data as *mut libc::c_void,
                           (*buffer).data as *const libc::c_void,
                           len as libc::c_ulong);
                    /* Parse the packet to ensure it is well-formed. */
                    retval = packet_set_attrset(ctx, secret, tmp);
                    if !(retval != 0 as libc::c_int) {
                        *pkt = tmp;
                        return 0 as libc::c_int
                    }
                }
            }
        }
    }
    krad_packet_free(tmp);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "390:1"]
pub unsafe extern "C" fn krad_packet_decode_request(mut ctx: krb5_context,
                                                    mut secret:
                                                        *const libc::c_char,
                                                    mut buffer:
                                                        *const krb5_data,
                                                    mut cb:
                                                        krad_packet_iter_cb,
                                                    mut data:
                                                        *mut libc::c_void,
                                                    mut duppkt:
                                                        *mut *const krad_packet,
                                                    mut reqpkt:
                                                        *mut *mut krad_packet)
 -> krb5_error_code {
    let mut tmp: *const krad_packet = 0 as *const krad_packet;
    let mut retval: krb5_error_code = 0;
    retval = decode_packet(ctx, secret, buffer, reqpkt);
    if cb.is_some() && retval == 0 as libc::c_int {
        tmp =
            Some(cb.expect("non-null function pointer")).expect("non-null function pointer")(data,
                                                                                             0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 krb5_boolean);
        while !tmp.is_null() {
            if *(&mut *(**reqpkt).pkt.data.offset(1 as libc::c_int as isize)
                     as *mut libc::c_char as *mut uchar) as libc::c_int ==
                   *(&mut *(*tmp).pkt.data.offset(1 as libc::c_int as isize)
                         as *mut libc::c_char as *mut uchar) as libc::c_int {
                break ;
            }
            tmp =
                Some(cb.expect("non-null function pointer")).expect("non-null function pointer")(data,
                                                                                                 0
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     krb5_boolean)
        }
    }
    if cb.is_some() && (retval != 0 as libc::c_int || !tmp.is_null()) {
        Some(cb.expect("non-null function pointer")).expect("non-null function pointer")(data,
                                                                                         1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             krb5_boolean);
    }
    *duppkt = tmp;
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "414:1"]
pub unsafe extern "C" fn krad_packet_decode_response(mut ctx: krb5_context,
                                                     mut secret:
                                                         *const libc::c_char,
                                                     mut buffer:
                                                         *const krb5_data,
                                                     mut cb:
                                                         krad_packet_iter_cb,
                                                     mut data:
                                                         *mut libc::c_void,
                                                     mut reqpkt:
                                                         *mut *const krad_packet,
                                                     mut rsppkt:
                                                         *mut *mut krad_packet)
 -> krb5_error_code {
    let mut auth: [uchar; 16] = [0; 16];
    let mut tmp: *const krad_packet = 0 as *const krad_packet;
    let mut retval: krb5_error_code = 0;
    retval = decode_packet(ctx, secret, buffer, rsppkt);
    if cb.is_some() && retval == 0 as libc::c_int {
        tmp =
            Some(cb.expect("non-null function pointer")).expect("non-null function pointer")(data,
                                                                                             0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 krb5_boolean);
        while !tmp.is_null() {
            if !(*(&mut *(**rsppkt).pkt.data.offset(1 as libc::c_int as isize)
                       as *mut libc::c_char as *mut uchar) as libc::c_int !=
                     *(&mut *(*tmp).pkt.data.offset(1 as libc::c_int as isize)
                           as *mut libc::c_char as *mut uchar) as libc::c_int)
               {
                /* Response */
                retval =
                    auth_generate_response(ctx, secret, *rsppkt,
                                           &mut *(*tmp).pkt.data.offset(4 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                               as *mut libc::c_char as
                                               *mut uchar, auth.as_mut_ptr());
                if retval != 0 as libc::c_int {
                    krad_packet_free(*rsppkt);
                    break ;
                } else if memcmp(&mut *(**rsppkt).pkt.data.offset(4 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                     as *mut libc::c_char as *mut uchar as
                                     *const libc::c_void,
                                 auth.as_mut_ptr() as *const libc::c_void,
                                 ::std::mem::size_of::<[uchar; 16]>() as
                                     libc::c_ulong) == 0 as libc::c_int
                 /* If the authenticator matches, then the response is valid. */
                 {
                    break ;
                }
            }
            tmp =
                Some(cb.expect("non-null function pointer")).expect("non-null function pointer")(data,
                                                                                                 0
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     krb5_boolean)
        }
    }
    if cb.is_some() && (retval != 0 as libc::c_int || !tmp.is_null()) {
        Some(cb.expect("non-null function pointer")).expect("non-null function pointer")(data,
                                                                                         1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             krb5_boolean);
    }
    *reqpkt = tmp;
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "451:1"]
pub unsafe extern "C" fn krad_packet_encode(mut pkt: *const krad_packet)
 -> *const krb5_data {
    return &(*pkt).pkt;
}
#[no_mangle]
#[c2rust::src_loc = "457:1"]
pub unsafe extern "C" fn krad_packet_get_code(mut pkt: *const krad_packet)
 -> krad_code {
    if pkt.is_null() { return 0 as libc::c_int as krad_code }
    return *(&mut *(*pkt).pkt.data.offset(0 as libc::c_int as isize) as
                 *mut libc::c_char as *mut krad_code);
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2013 Red Hat, Inc.  All rights reserved.
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
/*
 * This API is not considered as stable as the main krb5 API.
 *
 * - We may make arbitrary incompatible changes between feature releases
 *   (e.g. from 1.12 to 1.13).
 * - We will make some effort to avoid making incompatible changes for
 *   bugfix releases, but will make them if necessary.
 */
/* Called when a response is received or the request times out. */
/*
 * Called to iterate over a set of requests.  Either the callback will be
 * called until it returns NULL, or it will be called with cancel = TRUE to
 * terminate in the middle of an iteration.
 */
/*
 * Code
 */
/* Convert a code name to its number. Only works for codes defined
 * by RFC 2875 or 2882. Returns 0 if the name was not found. */
/* Convert a code number to its name. Only works for attributes defined
 * by RFC 2865 or 2882. Returns NULL if the name was not found. */
/*
 * Attribute
 */
/* Convert an attribute name to its number. Only works for attributes defined
 * by RFC 2865. Returns 0 if the name was not found. */
/* Convert an attribute number to its name. Only works for attributes defined
 * by RFC 2865. Returns NULL if the name was not found. */
/*
 * Attribute set
 */
/* Create a new attribute set. */
/* Create a deep copy of an attribute set. */
/* Free an attribute set. */
/* Add an attribute to a set. */
/* Add a four-octet unsigned number attribute to the given set. */
/* Delete the specified attribute. */
/* Get the specified attribute. */
/*
 * Packet
 */
/* Determine the bytes needed from the socket to get the whole packet.  Don't
 * cache the return value as it can change! Returns -1 on EBADMSG. */
/* Free a packet. */
/*
 * Create a new request packet.
 *
 * This function takes the attributes specified in set and converts them into a
 * radius packet. The packet will have a randomized id. If cb is not NULL, it
 * will be called passing data as the argument to iterate over a set of
 * outstanding requests. In this case, the id will be both random and unique
 * across the set of requests.
 */
/*
 * Create a new response packet.
 *
 * This function is similar to krad_packet_new_requst() except that it crafts a
 * packet in response to a request packet. This new packet will borrow values
 * from the request such as the id and the authenticator.
 */
/*
 * Decode a request radius packet from krb5_data.
 *
 * The resulting decoded packet will be a request packet stored in *reqpkt.
 *
 * If cb is NULL, *duppkt will always be NULL.
 *
 * If cb is not NULL, it will be called (with the data argument) to iterate
 * over a set of requests currently being processed. In this case, if the
 * packet is a duplicate of an already received request, the original request
 * will be set in *duppkt.
 */
/*
 * Decode a response radius packet from krb5_data.
 *
 * The resulting decoded packet will be a response packet stored in *rsppkt.
 *
 * If cb is NULL, *reqpkt will always be NULL.
 *
 * If cb is not NULL, it will be called (with the data argument) to iterate
 * over a set of requests awaiting responses. In this case, if the response
 * packet matches one of these requests, the original request will be set in
 * *reqpkt.
 */
/* Encode packet. */
/* Get the code for the given packet. */
/* Get the specified attribute. */
#[no_mangle]
#[c2rust::src_loc = "466:1"]
pub unsafe extern "C" fn krad_packet_get_attr(mut pkt: *const krad_packet,
                                              mut type_0: krad_attr,
                                              mut indx: size_t)
 -> *const krb5_data {
    return krad_attrset_get((*pkt).attrset, type_0, indx);
}
