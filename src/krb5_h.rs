extern "C" {
    #[no_mangle]
    pub fn krb5_cc_copy_creds(
        context: crate::krb5_h::krb5_context,
        incc: crate::krb5_h::krb5_ccache,
        outcc: crate::krb5_h::krb5_ccache,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_free_authdata(
        context: crate::krb5_h::krb5_context,
        val: *mut *mut crate::krb5_h::krb5_authdata,
    );
    /* *
     * Build an anonymous principal.
     *
     * This function returns constant storage that must not be freed.
     *
     * @sa #KRB5_ANONYMOUS_PRINCSTR
     */
    #[no_mangle]
    pub fn krb5_anonymous_principal() -> crate::krb5_h::krb5_const_principal;
    /* *
     * Free a string representation of a principal.
     *
     * @param [in] context          Library context
     * @param [in] val              Name string to be freed
     */
    #[no_mangle]
    pub fn krb5_free_unparsed_name(context: crate::krb5_h::krb5_context, val: *mut i8);

    #[no_mangle]
    pub fn krb5_get_error_message(
        ctx: crate::krb5_h::krb5_context,
        code: crate::krb5_h::krb5_error_code,
    ) -> *const i8;

    #[no_mangle]
    pub fn krb5_free_error_message(ctx: crate::krb5_h::krb5_context, msg: *const i8);
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
    pub fn krb5_c_make_checksum(
        context: crate::krb5_h::krb5_context,
        cksumtype: crate::krb5_h::krb5_cksumtype,
        key: *const crate::krb5_h::krb5_keyblock,
        usage: crate::krb5_h::krb5_keyusage,
        input: *const crate::krb5_h::krb5_data,
        cksum: *mut crate::krb5_h::krb5_checksum,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Fill in a checksum element in IOV array (operates on opaque key)
     *
     * @param [in]     context         Library context
     * @param [in]     cksumtype       Checksum type (0 for mandatory type)
     * @param [in]     key             Encryption key for a keyed checksum
     * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
     * @param [in,out] data            IOV array
     * @param [in]     num_data        Size of @a data
     *
     * Create a checksum in the #KRB5_CRYPTO_TYPE_CHECKSUM element over
     * #KRB5_CRYPTO_TYPE_DATA and #KRB5_CRYPTO_TYPE_SIGN_ONLY chunks in @a data.
     * Only the #KRB5_CRYPTO_TYPE_CHECKSUM region is modified.
     *
     * @note This function is similar to krb5_c_make_checksum_iov(), but operates
     * on opaque @a key.
     *
     * @sa krb5_k_verify_checksum_iov()
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_k_make_checksum_iov(
        context: crate::krb5_h::krb5_context,
        cksumtype: crate::krb5_h::krb5_cksumtype,
        key: crate::krb5_h::krb5_key,
        usage: crate::krb5_h::krb5_keyusage,
        data: *mut crate::krb5_h::krb5_crypto_iov,
        num_data: crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Validate a checksum element in IOV array (operates on opaque key).
     *
     * @param [in]     context         Library context
     * @param [in]     cksumtype       Checksum type (0 for mandatory type)
     * @param [in]     key             Encryption key for a keyed checksum
     * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
     * @param [in]     data            IOV array
     * @param [in]     num_data        Size of @a data
     * @param [out]    valid           Non-zero for success, zero for failure
     *
     * Confirm that the checksum in the #KRB5_CRYPTO_TYPE_CHECKSUM element is a
     * valid checksum of the #KRB5_CRYPTO_TYPE_DATA and #KRB5_CRYPTO_TYPE_SIGN_ONLY
     * regions in the iov.
     *
     * @note This function is similar to krb5_c_verify_checksum_iov(), but operates
     * on opaque @a key.
     *
     * @sa krb5_k_make_checksum_iov()
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_k_verify_checksum_iov(
        context: crate::krb5_h::krb5_context,
        cksumtype: crate::krb5_h::krb5_cksumtype,
        key: crate::krb5_h::krb5_key,
        usage: crate::krb5_h::krb5_keyusage,
        data: *const crate::krb5_h::krb5_crypto_iov,
        num_data: crate::stddef_h::size_t,
        valid: *mut crate::krb5_h::krb5_boolean,
    ) -> crate::krb5_h::krb5_error_code;

    pub type _krb5_init_creds_context;

    pub type _krb5_tkt_creds_context;
    /* *
     * Compute a checksum (operates on opaque key).
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
     * @note This function is similar to krb5_c_make_checksum(), but operates
     * on opaque @a key.
     *
     * @sa krb5_c_verify_checksum()
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_k_make_checksum(
        context: crate::krb5_h::krb5_context,
        cksumtype: crate::krb5_h::krb5_cksumtype,
        key: crate::krb5_h::krb5_key,
        usage: crate::krb5_h::krb5_keyusage,
        input: *const crate::krb5_h::krb5_data,
        cksum: *mut crate::krb5_h::krb5_checksum,
    ) -> crate::krb5_h::krb5_error_code;
    /* *< Want cached ticket only */
    /* *< Set canonicalize KDC option */
    /* *< Do not store in credential cache */
    /* *< Acquire forwardable tickets */
    /* *< Disable transited check */
    /* *< Constrained delegation */
    /* * @} */
    /* end of KRB5_GC group */
    /* *
     * Get an additional ticket.
     *
     * @param [in]  context         Library context
     * @param [in]  options         Options
     * @param [in]  ccache          Credential cache handle
     * @param [in]  in_creds        Input credentials
     * @param [out] out_creds       Output updated credentials
     *
     * Use @a ccache or a TGS exchange to get a service ticket matching @a
     * in_creds.
     *
     * Valid values for @a options are:
     * @li #KRB5_GC_CACHED     Search only credential cache for the ticket
     * @li #KRB5_GC_USER_USER  Return a user to user authentication ticket
     *
     * @a in_creds must be non-null.  @a in_creds->client and @a in_creds->server
     * must be filled in to specify the client and the server respectively.  If any
     * authorization data needs to be requested for the service ticket (such as
     * restrictions on how the ticket can be used), specify it in @a
     * in_creds->authdata; otherwise set @a in_creds->authdata to NULL.  The
     * session key type is specified in @a in_creds->keyblock.enctype, if it is
     * nonzero.
     *
     * The expiration date is specified in @a in_creds->times.endtime.
     * The KDC may return tickets with an earlier expiration date.
     * If @a in_creds->times.endtime is set to 0, the latest possible
     * expiration date will be requested.
     *
     * Any returned ticket and intermediate ticket-granting tickets are stored
     * in @a ccache.
     *
     * Use krb5_free_creds() to free @a out_creds when it is no longer needed.
     *
     * @retval
     *  0  Success
     * @return
     * Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_get_credentials(
        context: crate::krb5_h::krb5_context,
        options: crate::krb5_h::krb5_flags,
        ccache: crate::krb5_h::krb5_ccache,
        in_creds: *mut crate::krb5_h::krb5_creds,
        out_creds: *mut *mut crate::krb5_h::krb5_creds,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Free an error allocated by krb5_read_error() or krb5_sendauth().
     *
     * @param [in] context          Library context
     * @param [in] val              Error data structure to be freed
     *
     * This function frees the contents of @a val and the structure itself.
     */
    #[no_mangle]
    pub fn krb5_free_error(
        context: crate::krb5_h::krb5_context,
        val: *mut crate::krb5_h::krb5_error,
    );
    /* *
     * Free a krb5_creds structure.
     *
     * @param [in] context          Library context
     * @param [in] val              Credential structure to be freed.
     *
     * This function frees the contents of @a val and the structure itself.
     */
    #[no_mangle]
    pub fn krb5_free_creds(
        context: crate::krb5_h::krb5_context,
        val: *mut crate::krb5_h::krb5_creds,
    );
    /* *
     * Free the contents of a krb5_checksum structure.
     *
     * @param [in] context          Library context
     * @param [in] val              Checksum structure to free contents of
     *
     * This function frees the contents of @a val, but not the structure itself.
     */
    #[no_mangle]
    pub fn krb5_free_checksum_contents(
        context: crate::krb5_h::krb5_context,
        val: *mut crate::krb5_h::krb5_checksum,
    );
    /* *
     * Set the ticket lifetime in initial credential options.
     *
     * @param [in] opt              Options structure
     * @param [in] tkt_life         Ticket lifetime
     */
    #[no_mangle]
    pub fn krb5_get_init_creds_opt_set_tkt_life(
        opt: *mut crate::krb5_h::krb5_get_init_creds_opt,
        tkt_life: crate::krb5_h::krb5_deltat,
    );
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
    #[no_mangle]
    pub fn krb5_init_creds_free(
        context: crate::krb5_h::krb5_context,
        ctx: crate::krb5_h::krb5_init_creds_context,
    );
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
    #[no_mangle]
    pub fn krb5_init_creds_init(
        context: crate::krb5_h::krb5_context,
        client: crate::krb5_h::krb5_principal,
        prompter: crate::krb5_h::krb5_prompter_fct,
        data: *mut libc::c_void,
        start_time: crate::krb5_h::krb5_deltat,
        options: *mut crate::krb5_h::krb5_get_init_creds_opt,
        ctx: *mut crate::krb5_h::krb5_init_creds_context,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_init_creds_set_keytab(
        context: crate::krb5_h::krb5_context,
        ctx: crate::krb5_h::krb5_init_creds_context,
        keytab: crate::krb5_h::krb5_keytab,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_init_creds_step(
        context: crate::krb5_h::krb5_context,
        ctx: crate::krb5_h::krb5_init_creds_context,
        in_0: *mut crate::krb5_h::krb5_data,
        out: *mut crate::krb5_h::krb5_data,
        realm: *mut crate::krb5_h::krb5_data,
        flags: *mut u32,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_init_creds_set_password(
        context: crate::krb5_h::krb5_context,
        ctx: crate::krb5_h::krb5_init_creds_context,
        password: *const i8,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_init_creds_get_times(
        context: crate::krb5_h::krb5_context,
        ctx: crate::krb5_h::krb5_init_creds_context,
        times: *mut crate::krb5_h::krb5_ticket_times,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_tkt_creds_init(
        context: crate::krb5_h::krb5_context,
        ccache: crate::krb5_h::krb5_ccache,
        creds: *mut crate::krb5_h::krb5_creds,
        options: crate::krb5_h::krb5_flags,
        ctx: *mut crate::krb5_h::krb5_tkt_creds_context,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Free a TGS request context.
     *
     * @param[in]  context  Library context
     * @param[in]  ctx      TGS request context
     *
     * @version New in 1.9
     */
    #[no_mangle]
    pub fn krb5_tkt_creds_free(
        context: crate::krb5_h::krb5_context,
        ctx: crate::krb5_h::krb5_tkt_creds_context,
    );
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
    #[no_mangle]
    pub fn krb5_tkt_creds_step(
        context: crate::krb5_h::krb5_context,
        ctx: crate::krb5_h::krb5_tkt_creds_context,
        in_0: *mut crate::krb5_h::krb5_data,
        out: *mut crate::krb5_h::krb5_data,
        realm: *mut crate::krb5_h::krb5_data,
        flags: *mut u32,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_tkt_creds_get_times(
        context: crate::krb5_h::krb5_context,
        ctx: crate::krb5_h::krb5_tkt_creds_context,
        times: *mut crate::krb5_h::krb5_ticket_times,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Return cipher block size.
     *
     * @param [in]  context         Library context
     * @param [in]  enctype         Encryption type
     * @param [out] blocksize       Block size for @a enctype
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_c_block_size(
        context: crate::krb5_h::krb5_context,
        enctype: crate::krb5_h::krb5_enctype,
        blocksize: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Return a length of a message field specific to the encryption type.
     *
     * @param [in]  context      Library context
     * @param [in]  enctype      Encryption type
     * @param [in]  type         Type field (See @ref KRB5_CRYPTO_TYPE types)
     * @param [out] size         Length of the @a type specific to @a enctype
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_c_crypto_length(
        context: crate::krb5_h::krb5_context,
        enctype: crate::krb5_h::krb5_enctype,
        type_0: crate::krb5_h::krb5_cryptotype,
        size: *mut u32,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Return a number of padding octets.
     *
     * @param [in]  context      Library context
     * @param [in]  enctype      Encryption type
     * @param [in]  data_length  Length of the plaintext to pad
     * @param [out] size         Number of padding octets
     *
     * This function returns the number of the padding octets required to pad
     * @a data_length octets of plaintext.
     *
     * @retval 0 Success; otherwise - KRB5_BAD_ENCTYPE
     */
    #[no_mangle]
    pub fn krb5_c_padding_length(
        context: crate::krb5_h::krb5_context,
        enctype: crate::krb5_h::krb5_enctype,
        data_length: crate::stddef_h::size_t,
        size: *mut u32,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Build a principal name using null-terminated strings.
     *
     * @param [in]  context         Library context
     * @param [out] princ           Principal name
     * @param [in]  rlen            Realm name length
     * @param [in]  realm           Realm name
     * @param [in]  ...             List of char * components, ending with NULL
     *
     * Call krb5_free_principal() to free @a princ when it is no longer needed.
     *
     * @note krb5_build_principal() and krb5_build_principal_alloc_va() perform the
     * same task.  krb5_build_principal() takes variadic arguments.
     * krb5_build_principal_alloc_va() takes a pre-computed @a varargs pointer.
     *
     * @code
     * Example of how to build principal H/S@R
     *     krb5_build_principal(context, &principal,
     *                          strlen("R"), "R", "H", "S", (char*)NULL);
     * @endcode
     *
     * @retval
     * 0  Success
     * @return
     * Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_build_principal(
        context: crate::krb5_h::krb5_context,
        princ: *mut crate::krb5_h::krb5_principal,
        rlen: u32,
        realm: *const i8,
        _: ...
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Get the output length of pseudo-random functions for an encryption type.
     *
     * @param [in]  context         Library context
     * @param [in]  enctype         Encryption type
     * @param [out] len             Length of PRF output
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_c_prf_length(
        context: crate::krb5_h::krb5_context,
        enctype: crate::krb5_h::krb5_enctype,
        len: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;
    /* * Retrieve the enctype of a krb5_key structure. */
    #[no_mangle]
    pub fn krb5_k_key_enctype(
        context: crate::krb5_h::krb5_context,
        key: crate::krb5_h::krb5_key,
    ) -> crate::krb5_h::krb5_enctype;
    /* *
     * Generate enctype-specific pseudo-random bytes (operates on opaque key).
     *
     * @param [in]  context      Library context
     * @param [in]  key          Key
     * @param [in]  input        Input data
     * @param [out] output       Output data
     *
     * This function selects a pseudo-random function based on @a key and
     * computes its value over @a input, placing the result into @a output.
     * The caller must preinitialize @a output and allocate space for the
     * result.
     *
     * @note This function is similar to krb5_c_prf(), but operates
     * on opaque @a key.
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_k_prf(
        context: crate::krb5_h::krb5_context,
        key: crate::krb5_h::krb5_key,
        input: *mut crate::krb5_h::krb5_data,
        output: *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;
    #[no_mangle]
    pub fn krb5_parse_name_flags(
        context: crate::krb5_h::krb5_context,
        name: *const i8,
        flags: i32,
        principal_out: *mut crate::krb5_h::krb5_principal,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Copy a principal.
     *
     * @param [in]  context         Library context
     * @param [in]  inprinc         Principal to be copied
     * @param [out] outprinc        Copy of @a inprinc
     *
     * This function creates a new principal structure with the contents of @a
     * inprinc.  Use krb5_free_principal() to free @a outprinc when it is no longer
     * needed.
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_copy_principal(
        context: crate::krb5_h::krb5_context,
        inprinc: crate::krb5_h::krb5_const_principal,
        outprinc: *mut crate::krb5_h::krb5_principal,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_sname_to_principal(
        context: crate::krb5_h::krb5_context,
        hostname: *const i8,
        sname: *const i8,
        type_0: crate::krb5_h::krb5_int32,
        ret_princ: *mut crate::krb5_h::krb5_principal,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Encrypt data using a key (operates on opaque key).
     *
     * @param [in]     context      Library context
     * @param [in]     key          Encryption key
     * @param [in]     usage        Key usage (see @ref KRB5_KEYUSAGE types)
     * @param [in,out] cipher_state Cipher state; specify NULL if not needed
     * @param [in]     input        Data to be encrypted
     * @param [out]    output       Encrypted data
     *
     * This function encrypts the data block @a input and stores the output into @a
     * output.  The actual encryption key will be derived from @a key and @a usage
     * if key derivation is specified for the encryption type.  If non-null, @a
     * cipher_state specifies the beginning state for the encryption operation, and
     * is updated with the state to be passed as input to the next operation.
     *
     * @note The caller must initialize @a output and allocate at least enough
     * space for the result (using krb5_c_encrypt_length() to determine the amount
     * of space needed).  @a output->length will be set to the actual length of the
     * ciphertext.
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_k_encrypt(
        context: crate::krb5_h::krb5_context,
        key: crate::krb5_h::krb5_key,
        usage: crate::krb5_h::krb5_keyusage,
        cipher_state: *const crate::krb5_h::krb5_data,
        input: *const crate::krb5_h::krb5_data,
        output: *mut crate::krb5_h::krb5_enc_data,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Decrypt data using a key (operates on opaque key).
     *
     * @param [in]     context      Library context
     * @param [in]     key          Encryption key
     * @param [in]     usage        Key usage (see @ref KRB5_KEYUSAGE types)
     * @param [in,out] cipher_state Cipher state; specify NULL if not needed
     * @param [in]     input        Encrypted data
     * @param [out]    output       Decrypted data
     *
     * This function decrypts the data block @a input and stores the output into @a
     * output. The actual decryption key will be derived from @a key and @a usage
     * if key derivation is specified for the encryption type.  If non-null, @a
     * cipher_state specifies the beginning state for the decryption operation, and
     * is updated with the state to be passed as input to the next operation.
     *
     * @note The caller must initialize @a output and allocate at least enough
     * space for the result.  The usual practice is to allocate an output buffer as
     * long as the ciphertext, and let krb5_c_decrypt() trim @a output->length.
     * For some enctypes, the resulting @a output->length may include padding
     * bytes.
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_k_decrypt(
        context: crate::krb5_h::krb5_context,
        key: crate::krb5_h::krb5_key,
        usage: crate::krb5_h::krb5_keyusage,
        cipher_state: *const crate::krb5_h::krb5_data,
        input: *const crate::krb5_h::krb5_enc_data,
        output: *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;
    /* * @deprecated Replaced by krb5_c_* API family. */
    #[no_mangle]
    pub fn krb5_encrypt_size(
        length: crate::stddef_h::size_t,
        crypto: crate::krb5_h::krb5_enctype,
    ) -> crate::stddef_h::size_t;
    /* *
     * Retrieve the full name of a credential cache.
     *
     * @param [in]  context         Library context
     * @param [in]  cache           Credential cache handle
     * @param [out] fullname_out    Full name of cache
     *
     * Use krb5_free_string() to free @a fullname_out when it is no longer needed.
     *
     * @version New in 1.10
     */
    #[no_mangle]
    pub fn krb5_cc_get_full_name(
        context: crate::krb5_h::krb5_context,
        cache: crate::krb5_h::krb5_ccache,
        fullname_out: *mut *mut i8,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Get a key table name.
     *
     * @param [in]  context         Library context
     * @param [in]  keytab          Key table handle
     * @param [out] name            Key table name
     * @param [in]  namelen         Maximum length to fill in name
     *
     * Fill @a name with the name of @a keytab including the type and delimiter.
     *
     * @sa MAX_KEYTAB_NAME_LEN
     *
     * @retval
     * 0 Success
     * @retval
     * KRB5_KT_NAME_TOOLONG  Key table name does not fit in @a namelen bytes
     *
     * @return
     * Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_kt_get_name(
        context: crate::krb5_h::krb5_context,
        keytab: crate::krb5_h::krb5_keytab,
        name: *mut i8,
        namelen: u32,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_free_addresses(
        context: crate::krb5_h::krb5_context,
        val: *mut *mut crate::krb5_h::krb5_address,
    );
    /* *
     * Test whether a checksum type is keyed.
     *
     * @param [in] ctype            Checksum type
     *
     * @return @c TRUE if @a ctype is a keyed checksum type, @c FALSE otherwise.
     */
    #[no_mangle]
    pub fn krb5_c_is_keyed_cksum(
        ctype: crate::krb5_h::krb5_cksumtype,
    ) -> crate::krb5_h::krb5_boolean;
    /* *
     * Validate a checksum element in IOV array (operates on keyblock).
     *
     * @param [in]     context         Library context
     * @param [in]     cksumtype       Checksum type (0 for mandatory type)
     * @param [in]     key             Encryption key for a keyed checksum
     * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
     * @param [in]     data            IOV array
     * @param [in]     num_data        Size of @a data
     * @param [out]    valid           Non-zero for success, zero for failure
     *
     * Confirm that the checksum in the #KRB5_CRYPTO_TYPE_CHECKSUM element is a
     * valid checksum of the #KRB5_CRYPTO_TYPE_DATA and #KRB5_CRYPTO_TYPE_SIGN_ONLY
     * regions in the iov.
     *
     * @note This function is similar to krb5_k_verify_checksum_iov(), but operates
     * on keyblock @a key.
     *
     * @sa krb5_c_make_checksum_iov()
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_c_verify_checksum_iov(
        context: crate::krb5_h::krb5_context,
        cksumtype: crate::krb5_h::krb5_cksumtype,
        key: *const crate::krb5_h::krb5_keyblock,
        usage: crate::krb5_h::krb5_keyusage,
        data: *const crate::krb5_h::krb5_crypto_iov,
        num_data: crate::stddef_h::size_t,
        valid: *mut crate::krb5_h::krb5_boolean,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_free_keyblock_contents(
        context: crate::krb5_h::krb5_context,
        key: *mut crate::krb5_h::krb5_keyblock,
    );
    /* * Retrieve a copy of the keyblock from a krb5_key structure. */
    #[no_mangle]
    pub fn krb5_k_key_keyblock(
        context: crate::krb5_h::krb5_context,
        key: crate::krb5_h::krb5_key,
        key_data: *mut *mut crate::krb5_h::krb5_keyblock,
    ) -> crate::krb5_h::krb5_error_code;
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
    pub fn krb5_c_random_make_octets(
        context: crate::krb5_h::krb5_context,
        data: *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Encrypt data in place supporting AEAD (operates on opaque key).
     *
     * @param [in]     context         Library context
     * @param [in]     key             Encryption key
     * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
     * @param [in]     cipher_state    Cipher state; specify NULL if not needed
     * @param [in,out] data            IOV array. Modified in-place.
     * @param [in]     num_data        Size of @a data
     *
     * This function encrypts the data block @a data and stores the output in-place.
     * The actual encryption key will be derived from @a key and @a usage
     * if key derivation is specified for the encryption type.  If non-null, @a
     * cipher_state specifies the beginning state for the encryption operation, and
     * is updated with the state to be passed as input to the next operation.
     * The caller must allocate the right number of krb5_crypto_iov
     * structures before calling into this API.
     *
     * @note On return from a krb5_c_encrypt_iov() call, the @a data->length in the
     * iov structure are adjusted to reflect actual lengths of the ciphertext used.
     * For example, if the padding length is too large, the length will be reduced.
     * Lengths are never increased.
     *
     * @note This function is similar to krb5_c_encrypt_iov(), but operates
     * on opaque key @a key.
     *
     * @sa krb5_k_decrypt_iov()
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_k_encrypt_iov(
        context: crate::krb5_h::krb5_context,
        key: crate::krb5_h::krb5_key,
        usage: crate::krb5_h::krb5_keyusage,
        cipher_state: *const crate::krb5_h::krb5_data,
        data: *mut crate::krb5_h::krb5_crypto_iov,
        num_data: crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Decrypt data in place supporting AEAD (operates on opaque key).
     *
     * @param [in]     context         Library context
     * @param [in]     key             Encryption key
     * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
     * @param [in]     cipher_state    Cipher state; specify NULL if not needed
     * @param [in,out] data            IOV array. Modified in-place.
     * @param [in]     num_data        Size of @a data
     *
     * This function decrypts the data block @a data and stores the output in-place.
     * The actual decryption key will be derived from @a key and @a usage
     * if key derivation is specified for the encryption type.  If non-null, @a
     * cipher_state specifies the beginning state for the decryption operation, and
     * is updated with the state to be passed as input to the next operation.
     * The caller must allocate the right number of krb5_crypto_iov
     * structures before calling into this API.
     *
     * @note On return from a krb5_c_decrypt_iov() call, the @a data->length in the
     * iov structure are adjusted to reflect actual lengths of the ciphertext used.
     * For example, if the padding length is too large, the length will be reduced.
     * Lengths are never increased.
     *
     * @note This function is similar to krb5_c_decrypt_iov(), but operates
     * on opaque key @a key.
     *
     * @sa krb5_k_encrypt_iov()
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_k_decrypt_iov(
        context: crate::krb5_h::krb5_context,
        key: crate::krb5_h::krb5_key,
        usage: crate::krb5_h::krb5_keyusage,
        cipher_state: *const crate::krb5_h::krb5_data,
        data: *mut crate::krb5_h::krb5_crypto_iov,
        num_data: crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;
    /* * @defgroup KRB5_KEYUSAGE KRB5_KEYUSAGE
     * @{
     */
    /* XXX need to register these */
    /* Defined in Integrating SAM Mechanisms with Kerberos draft */
    /* * Note conflict with @ref KRB5_KEYUSAGE_PA_S4U_X509_USER_REQUEST */
    /* * Note conflict with @ref KRB5_KEYUSAGE_PA_S4U_X509_USER_REPLY */
    /* Defined in [MS-SFU] */
    /* * Note conflict with @ref KRB5_KEYUSAGE_PA_SAM_CHALLENGE_TRACKID */
    /* * Note conflict with @ref KRB5_KEYUSAGE_PA_SAM_RESPONSE */
    /* unused */
    /* *< See RFC 6560 section 4.2 */
    /* define in draft-ietf-krb-wg-preauth-framework*/
    /* Key usage values 512-1023 are reserved for uses internal to a Kerberos
     * implementation. */
    /* *< Used for encrypted FAST cookies */
    /* *< Used for freshness tokens */
    /* * @} */
    /* end of KRB5_KEYUSAGE group */
    /* *
     * Verify that a specified encryption type is a valid Kerberos encryption type.
     *
     * @param [in] ktype            Encryption type
     *
     * @return @c TRUE if @a ktype is valid, @c FALSE if not
     */
    #[no_mangle]
    pub fn krb5_c_valid_enctype(ktype: crate::krb5_h::krb5_enctype) -> crate::krb5_h::krb5_boolean;
    /* *
     * Get the default principal of a credential cache.
     *
     * @param [in]  context         Library context
     * @param [in]  cache           Credential cache handle
     * @param [out] principal       Primary principal
     *
     * Returns the default client principal of a credential cache as set by
     * krb5_cc_initialize().
     *
     * Use krb5_free_principal() to free @a principal when it is no longer needed.
     *
     * @retval
     * 0  Success
     * @return
     * Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_cc_get_principal(
        context: crate::krb5_h::krb5_context,
        cache: crate::krb5_h::krb5_ccache,
        principal: *mut crate::krb5_h::krb5_principal,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Prepare to sequentially read every credential in a credential cache.
     *
     * @param [in]  context         Library context
     * @param [in]  cache           Credential cache handle
     * @param [out] cursor          Cursor
     *
     * krb5_cc_end_seq_get() must be called to complete the retrieve operation.
     *
     * @note If the cache represented by @a cache is modified between the time of
     * the call to this function and the time of the final krb5_cc_end_seq_get(),
     * these changes may not be reflected in the results of krb5_cc_next_cred()
     * calls.
     *
     * @retval 0  Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_cc_start_seq_get(
        context: crate::krb5_h::krb5_context,
        cache: crate::krb5_h::krb5_ccache,
        cursor: *mut crate::krb5_h::krb5_cc_cursor,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Retrieve the next entry from the credential cache.
     *
     * @param [in]  context         Library context
     * @param [in]  cache           Credential cache handle
     * @param [in]  cursor          Cursor
     * @param [out] creds           Next credential cache entry
     *
     * This function fills in @a creds with the next entry in @a cache and advances
     * @a cursor.
     *
     * Use krb5_free_cred_contents() to free @a creds when it is no longer needed.
     *
     * @sa krb5_cc_start_seq_get(), krb5_end_seq_get()
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_cc_next_cred(
        context: crate::krb5_h::krb5_context,
        cache: crate::krb5_h::krb5_ccache,
        cursor: *mut crate::krb5_h::krb5_cc_cursor,
        creds: *mut crate::krb5_h::krb5_creds,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Finish a series of sequential processing credential cache entries.
     *
     * @param [in] context          Library context
     * @param [in] cache            Credential cache handle
     * @param [in] cursor           Cursor
     *
     * This function finishes processing credential cache entries and invalidates
     * @a cursor.
     *
     * @sa krb5_cc_start_seq_get(), krb5_cc_next_cred()
     *
     * @retval 0 (always)
     */
    #[no_mangle]
    pub fn krb5_cc_end_seq_get(
        context: crate::krb5_h::krb5_context,
        cache: crate::krb5_h::krb5_ccache,
        cursor: *mut crate::krb5_h::krb5_cc_cursor,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Set options flags on a credential cache.
     *
     * @param [in] context          Library context
     * @param [in] cache            Credential cache handle
     * @param [in] flags            Flag bit mask
     *
     * This function resets @a cache flags to @a flags.
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_cc_set_flags(
        context: crate::krb5_h::krb5_context,
        cache: crate::krb5_h::krb5_ccache,
        flags: crate::krb5_h::krb5_flags,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Retrieve the type of a credential cache.
     *
     * @param [in] context          Library context
     * @param [in] cache            Credential cache handle
     *
     * @return The type of a credential cache as an alias that must not be modified
     * or freed by the caller.
     */
    #[no_mangle]
    pub fn krb5_cc_get_type(
        context: crate::krb5_h::krb5_context,
        cache: crate::krb5_h::krb5_ccache,
    ) -> *const i8;
    /* *
     * Check if the credential cache collection contains any credentials.
     *
     * @param [in]  context         Library context
     *
     * @version New in 1.11
     *
     * @retval 0 Credentials are available in the collection
     * @retval KRB5_CC_NOTFOUND The collection contains no credentials
     */
    #[no_mangle]
    pub fn krb5_cccol_have_content(
        context: crate::krb5_h::krb5_context,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Close a key table handle.
     *
     * @param [in] context          Library context
     * @param [in] keytab           Key table handle
     *
     * @retval 0
     */
    #[no_mangle]
    pub fn krb5_kt_close(
        context: crate::krb5_h::krb5_context,
        keytab: crate::krb5_h::krb5_keytab,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Get an entry from a key table.
     *
     * @param [in]  context         Library context
     * @param [in]  keytab          Key table handle
     * @param [in]  principal       Principal name
     * @param [in]  vno             Key version number (0 for highest available)
     * @param [in]  enctype         Encryption type (0 zero for any enctype)
     * @param [out] entry           Returned entry from key table
     *
     * Retrieve an entry from a key table which matches the @a keytab, @a
     * principal, @a vno, and @a enctype.  If @a vno is zero, retrieve the
     * highest-numbered kvno matching the other fields.  If @a enctype is 0, match
     * any enctype.
     *
     * Use krb5_free_keytab_entry_contents() to free @a entry when it is no longer
     * needed.
     *
     * @note If @a vno is zero, the function retrieves the highest-numbered-kvno
     * entry that matches the specified principal.
     *
     * @retval
     * 0 Success
     * @retval
     * Kerberos error codes on failure
     */
    #[no_mangle]
    pub fn krb5_kt_get_entry(
        context: crate::krb5_h::krb5_context,
        keytab: crate::krb5_h::krb5_keytab,
        principal: crate::krb5_h::krb5_const_principal,
        vno: crate::krb5_h::krb5_kvno,
        enctype: crate::krb5_h::krb5_enctype,
        entry: *mut crate::krb5_h::krb5_keytab_entry,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Start a sequential retrieval of key table entries.
     *
     * @param [in]  context         Library context
     * @param [in]  keytab          Key table handle
     * @param [out] cursor          Cursor
     *
     * Prepare to read sequentially every key in the specified key table.  Use
     * krb5_kt_end_seq_get() to release the cursor when it is no longer needed.
     *
     * @sa krb5_kt_next_entry(), krb5_kt_end_seq_get()
     *
     * @retval
     * 0 Success
     * @return
     * Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_kt_start_seq_get(
        context: crate::krb5_h::krb5_context,
        keytab: crate::krb5_h::krb5_keytab,
        cursor: *mut crate::krb5_h::krb5_kt_cursor,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Retrieve the next entry from the key table.
     *
     * @param [in]  context         Library context
     * @param [in]  keytab          Key table handle
     * @param [out] entry           Returned key table entry
     * @param [in]  cursor          Key table cursor
     *
     * Return the next sequential entry in @a keytab and advance @a cursor.
     * Callers must release the returned entry with krb5_kt_free_entry().
     *
     * @sa krb5_kt_start_seq_get(), krb5_kt_end_seq_get()
     *
     * @retval
     * 0 Success
     * @retval
     * KRB5_KT_END - if the last entry was reached
     * @return
     * Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_kt_next_entry(
        context: crate::krb5_h::krb5_context,
        keytab: crate::krb5_h::krb5_keytab,
        entry: *mut crate::krb5_h::krb5_keytab_entry,
        cursor: *mut crate::krb5_h::krb5_kt_cursor,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Release a keytab cursor.
     *
     * @param [in]  context         Library context
     * @param [in]  keytab          Key table handle
     * @param [out] cursor          Cursor
     *
     * This function should be called to release the cursor created by
     * krb5_kt_start_seq_get().
     *
     * @retval
     * 0 Success
     * @return
     * Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_kt_end_seq_get(
        context: crate::krb5_h::krb5_context,
        keytab: crate::krb5_h::krb5_keytab,
        cursor: *mut crate::krb5_h::krb5_kt_cursor,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Check if a keytab exists and contains entries.
     *
     * @param [in]  context         Library context
     * @param [in]  keytab          Key table handle
     *
     * @version New in 1.11
     *
     * @retval 0 Keytab exists and contains entries
     * @retval KRB5_KT_NOTFOUND Keytab does not contain entries
     */
    #[no_mangle]
    pub fn krb5_kt_have_content(
        context: crate::krb5_h::krb5_context,
        keytab: crate::krb5_h::krb5_keytab,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Convert a string principal name to a krb5_principal structure.
     *
     * @param [in]  context         Library context
     * @param [in]  name            String representation of a principal name
     * @param [out] principal_out   New principal
     *
     * Convert a string representation of a principal name to a krb5_principal
     * structure.
     *
     * A string representation of a Kerberos name consists of one or more principal
     * name components, separated by slashes, optionally followed by the \@
     * character and a realm name.  If the realm name is not specified, the local
     * realm is used.
     *
     * To use the slash and \@ symbols as part of a component (quoted) instead of
     * using them as a component separator or as a realm prefix), put a backslash
     * (\) character in front of the symbol.  Similarly, newline, tab, backspace,
     * and NULL characters can be included in a component by using @c n, @c t, @c b
     * or @c 0, respectively.
     *
     * @note The realm in a Kerberos @a name cannot contain slash, colon,
     * or NULL characters.
     *
     * Use krb5_free_principal() to free @a principal_out when it is no longer
     * needed.
     *
     * @retval
     * 0 Success
     * @return
     * Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_parse_name(
        context: crate::krb5_h::krb5_context,
        name: *const i8,
        principal_out: *mut crate::krb5_h::krb5_principal,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Convert a krb5_principal structure to a string representation.
     *
     * @param [in]  context         Library context
     * @param [in]  principal       Principal
     * @param [out] name            String representation of principal name
     *
     * The resulting string representation uses the format and quoting conventions
     * described for krb5_parse_name().
     *
     * Use krb5_free_unparsed_name() to free @a name when it is no longer needed.
     *
     * @retval
     * 0 Success
     * @return
     * Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_unparse_name(
        context: crate::krb5_h::krb5_context,
        principal: crate::krb5_h::krb5_const_principal,
        name: *mut *mut i8,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Compare two principals.
     *
     * @param [in] context          Library context
     * @param [in] princ1           First principal
     * @param [in] princ2           Second principal
     *
     * @retval
     * TRUE if the principals are the same; FALSE otherwise
     */
    #[no_mangle]
    pub fn krb5_principal_compare(
        context: crate::krb5_h::krb5_context,
        princ1: crate::krb5_h::krb5_const_principal,
        princ2: crate::krb5_h::krb5_const_principal,
    ) -> crate::krb5_h::krb5_boolean;
    /* *
     * Generate a replay cache object for server use and open it.
     *
     * @param [in]  context         Library context
     * @param [in]  piece           Unused (replay cache identifier)
     * @param [out] rcptr           Handle to an open rcache
     *
     * This function creates a handle to the default replay cache.  Use
     * krb5_rc_close() to close @a rcptr when it is no longer needed.
     *
     * @version Prior to release 1.18, this function creates a handle to a
     * different replay cache for each unique value of @a piece.
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_get_server_rcache(
        context: crate::krb5_h::krb5_context,
        piece: *const crate::krb5_h::krb5_data,
        rcptr: *mut crate::krb5_h::krb5_rcache,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Build a principal name using length-counted strings.
     *
     * @param [in]  context  Library context
     * @param [out] princ    Principal name
     * @param [in]  rlen     Realm name length
     * @param [in]  realm    Realm name
     * @param [in]  ...      List of unsigned int/char * components, followed by 0
     *
     * This function creates a principal from a length-counted string and a
     * variable-length list of length-counted components.  The list of components
     * ends with the first 0 length argument (so it is not possible to specify an
     * empty component with this function).  Call krb5_free_principal() to free
     * allocated memory for principal when it is no longer needed.
     *
     * @code
     * Example of how to build principal WELLKNOWN/ANONYMOUS@R
     *     krb5_build_principal_ext(context, &principal, strlen("R"), "R",
     *         (unsigned int)strlen(KRB5_WELLKNOWN_NAMESTR),
     *         KRB5_WELLKNOWN_NAMESTR,
     *         (unsigned int)strlen(KRB5_ANONYMOUS_PRINCSTR),
     *         KRB5_ANONYMOUS_PRINCSTR, 0);
     * @endcode
     *
     * @retval
     * 0  Success
     * @return
     * Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_build_principal_ext(
        context: crate::krb5_h::krb5_context,
        princ: *mut crate::krb5_h::krb5_principal,
        rlen: u32,
        realm: *const i8,
        _: ...
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_kt_resolve(
        context: crate::krb5_h::krb5_context,
        name: *const i8,
        ktid: *mut crate::krb5_h::krb5_keytab,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_kt_dup(
        context: crate::krb5_h::krb5_context,
        in_0: crate::krb5_h::krb5_keytab,
        out: *mut crate::krb5_h::krb5_keytab,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_kt_default(
        context: crate::krb5_h::krb5_context,
        id: *mut crate::krb5_h::krb5_keytab,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_kt_client_default(
        context: crate::krb5_h::krb5_context,
        keytab_out: *mut crate::krb5_h::krb5_keytab,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_free_keytab_entry_contents(
        context: crate::krb5_h::krb5_context,
        entry: *mut crate::krb5_h::krb5_keytab_entry,
    ) -> crate::krb5_h::krb5_error_code;
    /* * @deprecated Use krb5_free_keytab_entry_contents instead. */
    #[no_mangle]
    pub fn krb5_kt_free_entry(
        context: crate::krb5_h::krb5_context,
        entry: *mut crate::krb5_h::krb5_keytab_entry,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_cc_resolve(
        context: crate::krb5_h::krb5_context,
        name: *const i8,
        cache: *mut crate::krb5_h::krb5_ccache,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_cc_dup(
        context: crate::krb5_h::krb5_context,
        in_0: crate::krb5_h::krb5_ccache,
        out: *mut crate::krb5_h::krb5_ccache,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_cc_default(
        context: crate::krb5_h::krb5_context,
        ccache: *mut crate::krb5_h::krb5_ccache,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_cc_set_config(
        context: crate::krb5_h::krb5_context,
        id: crate::krb5_h::krb5_ccache,
        principal: crate::krb5_h::krb5_const_principal,
        key: *const i8,
        data: *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_is_config_principal(
        context: crate::krb5_h::krb5_context,
        principal: crate::krb5_h::krb5_const_principal,
    ) -> crate::krb5_h::krb5_boolean;
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
    #[no_mangle]
    pub fn krb5_cc_support_switch(
        context: crate::krb5_h::krb5_context,
        type_0: *const i8,
    ) -> crate::krb5_h::krb5_boolean;
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
    #[no_mangle]
    pub fn krb5_cc_cache_match(
        context: crate::krb5_h::krb5_context,
        client: crate::krb5_h::krb5_principal,
        cache_out: *mut crate::krb5_h::krb5_ccache,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_cc_select(
        context: crate::krb5_h::krb5_context,
        server: crate::krb5_h::krb5_principal,
        cache_out: *mut crate::krb5_h::krb5_ccache,
        princ_out: *mut crate::krb5_h::krb5_principal,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Free the contents of a krb5_creds structure.
     *
     * @param [in] context          Library context
     * @param [in] val              Credential structure to free contents of
     *
     * This function frees the contents of @a val, but not the structure itself.
     */
    #[no_mangle]
    pub fn krb5_free_cred_contents(
        context: crate::krb5_h::krb5_context,
        val: *mut crate::krb5_h::krb5_creds,
    );
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
    #[no_mangle]
    pub fn krb5_sname_match(
        context: crate::krb5_h::krb5_context,
        matching: crate::krb5_h::krb5_const_principal,
        princ: crate::krb5_h::krb5_const_principal,
    ) -> crate::krb5_h::krb5_boolean;
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
    #[no_mangle]
    pub fn krb5_get_init_creds_opt_alloc(
        context: crate::krb5_h::krb5_context,
        opt: *mut *mut crate::krb5_h::krb5_get_init_creds_opt,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Free initial credential options.
     *
     * @param [in] context          Library context
     * @param [in] opt              Options structure to free
     *
     * @sa krb5_get_init_creds_opt_alloc()
     */
    #[no_mangle]
    pub fn krb5_get_init_creds_opt_free(
        context: crate::krb5_h::krb5_context,
        opt: *mut crate::krb5_h::krb5_get_init_creds_opt,
    );
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
    #[no_mangle]
    pub fn krb5_get_init_creds_opt_set_out_ccache(
        context: crate::krb5_h::krb5_context,
        opt: *mut crate::krb5_h::krb5_get_init_creds_opt,
        ccache: crate::krb5_h::krb5_ccache,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_get_init_creds_password(
        context: crate::krb5_h::krb5_context,
        creds: *mut crate::krb5_h::krb5_creds,
        client: crate::krb5_h::krb5_principal,
        password: *const i8,
        prompter: crate::krb5_h::krb5_prompter_fct,
        data: *mut libc::c_void,
        start_time: crate::krb5_h::krb5_deltat,
        in_tkt_service: *const i8,
        k5_gic_options: *mut crate::krb5_h::krb5_get_init_creds_opt,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_get_init_creds_keytab(
        context: crate::krb5_h::krb5_context,
        creds: *mut crate::krb5_h::krb5_creds,
        client: crate::krb5_h::krb5_principal,
        arg_keytab: crate::krb5_h::krb5_keytab,
        start_time: crate::krb5_h::krb5_deltat,
        in_tkt_service: *const i8,
        k5_gic_options: *mut crate::krb5_h::krb5_get_init_creds_opt,
    ) -> crate::krb5_h::krb5_error_code;

    #[no_mangle]
    pub fn krb5_set_error_message(
        ctx: crate::krb5_h::krb5_context,
        code: crate::krb5_h::krb5_error_code,
        fmt: *const i8,
        _: ...
    );

    #[no_mangle]
    pub fn krb5_clear_error_message(ctx: crate::krb5_h::krb5_context);
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
    #[no_mangle]
    pub fn krb5_cc_default_name(context: crate::krb5_h::krb5_context) -> *const i8;
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
    #[no_mangle]
    pub fn krb5_cc_set_default_name(
        context: crate::krb5_h::krb5_context,
        name: *const i8,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_aname_to_localname(
        context: crate::krb5_h::krb5_context,
        aname: crate::krb5_h::krb5_const_principal,
        lnsize_in: i32,
        lname: *mut i8,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_kuserok(
        context: crate::krb5_h::krb5_context,
        principal: crate::krb5_h::krb5_principal,
        luser: *const i8,
    ) -> crate::krb5_h::krb5_boolean;
    /* *< Principal of this key */
    /* *< Time entry written to keytable */
    /* *< Key version number */
    /* *< The secret key */
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

    pub type _profile_t;

    pub type _krb5_auth_context;

    pub type _krb5_ccache;
    /*
     * end "ccache.h"
     */
    /*
     * begin "rcache.h"
     */

    pub type krb5_rc_st;
    /* *
     * Return the length of checksums for a checksum type.
     *
     * @param [in]  context         Library context
     * @param [in]  cksumtype       Checksum type
     * @param [out] length          Checksum length
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_c_checksum_length(
        context: crate::krb5_h::krb5_context,
        cksumtype: crate::krb5_h::krb5_cksumtype,
        length: *mut crate::stddef_h::size_t,
    ) -> crate::krb5_h::krb5_error_code;
    /* * Decrement the reference count on a key and free it if it hits zero. */
    #[no_mangle]
    pub fn krb5_k_free_key(context: crate::krb5_h::krb5_context, key: crate::krb5_h::krb5_key);
    /* *
     * Verify a checksum (operates on opaque key).
     *
     * @param [in]  context         Library context
     * @param [in]  key             Encryption key for a keyed checksum
     * @param [in]  usage           @a key usage
     * @param [in]  data            Data to be used to compute a new checksum
     *                              using @a key to compare @a cksum against
     * @param [in]  cksum           Checksum to be verified
     * @param [out] valid           Non-zero for success, zero for failure
     *
     * This function verifies that @a cksum is a valid checksum for @a data.  If
     * the checksum type of @a cksum is a keyed checksum, @a key is used to verify
     * the checksum.  If the checksum type in @a cksum is 0 and @a key is not NULL,
     * the mandatory checksum type for @a key will be used.  The actual checksum
     * key will be derived from @a key and @a usage if key derivation is specified
     * for the checksum type.
     *
     * @note This function is similar to krb5_c_verify_checksum(), but operates
     * on opaque @a key.
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_k_verify_checksum(
        context: crate::krb5_h::krb5_context,
        key: crate::krb5_h::krb5_key,
        usage: crate::krb5_h::krb5_keyusage,
        data: *const crate::krb5_h::krb5_data,
        cksum: *const crate::krb5_h::krb5_checksum,
        valid: *mut crate::krb5_h::krb5_boolean,
    ) -> crate::krb5_h::krb5_error_code;
    /* KRB5_DEPRECATED */
    /* *
     * Initialize a credential cache.
     *
     * @param [in] context          Library context
     * @param [in] cache            Credential cache handle
     * @param [in] principal        Default principal name
     *
     * Destroy any existing contents of @a cache and initialize it for the default
     * principal @a principal.
     *
     * @retval
     *  0  Success
     * @return
     *  System errors; Permission errors; Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_cc_initialize(
        context: crate::krb5_h::krb5_context,
        cache: crate::krb5_h::krb5_ccache,
        principal: crate::krb5_h::krb5_principal,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Destroy a credential cache.
     *
     * @param [in] context          Library context
     * @param [in] cache            Credential cache handle
     *
     * This function destroys any existing contents of @a cache and closes the
     * handle to it.
     *
     * @retval
     * 0  Success
     * @return
     * Permission errors
     */
    #[no_mangle]
    pub fn krb5_cc_destroy(
        context: crate::krb5_h::krb5_context,
        cache: crate::krb5_h::krb5_ccache,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Close a credential cache handle.
     *
     * @param [in] context          Library context
     * @param [in] cache            Credential cache handle
     *
     * This function closes a credential cache handle @a cache without affecting
     * the contents of the cache.
     *
     * @retval
     * 0  Success
     * @return
     * Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_cc_close(
        context: crate::krb5_h::krb5_context,
        cache: crate::krb5_h::krb5_ccache,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Store credentials in a credential cache.
     *
     * @param [in] context          Library context
     * @param [in] cache            Credential cache handle
     * @param [in] creds            Credentials to be stored in cache
     *
     * This function stores @a creds into @a cache.  If @a creds->server and the
     * server in the decoded ticket @a creds->ticket differ, the credentials will
     * be stored under both server principal names.
     *
     * @retval
     *  0  Success
     * @return Permission errors; storage failure errors; Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_cc_store_cred(
        context: crate::krb5_h::krb5_context,
        cache: crate::krb5_h::krb5_ccache,
        creds: *mut crate::krb5_h::krb5_creds,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Create a new credential cache of the specified type with a unique name.
     *
     * @param [in]  context         Library context
     * @param [in]  type            Credential cache type name
     * @param [in]  hint            Unused
     * @param [out] id              Credential cache handle
     *
     * @retval
     * 0 Success
     * @return
     * Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_cc_new_unique(
        context: crate::krb5_h::krb5_context,
        type_0: *const i8,
        hint: *const i8,
        id: *mut crate::krb5_h::krb5_ccache,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Free a krb5 library context.
     *
     * @param [in] context          Library context
     *
     * This function frees a @a context that was created by krb5_init_context()
     * or krb5_init_secure_context().
     */
    #[no_mangle]
    pub fn krb5_free_context(context: crate::krb5_h::krb5_context);
    /* *
     * Free an array of credential structures.
     *
     * @param [in] context          Library context
     * @param [in] tgts             Null-terminated array of credentials to free
     *
     * @note The last entry in the array @a tgts must be a NULL pointer.
     */
    #[no_mangle]
    pub fn krb5_free_tgt_creds(
        context: crate::krb5_h::krb5_context,
        tgts: *mut *mut crate::krb5_h::krb5_creds,
    );
    /* *
     * Format and encrypt a @c KRB_AP_REP message.
     *
     * @param [in]  context         Library context
     * @param [in]  auth_context    Authentication context
     * @param [out] outbuf          @c AP-REP message
     *
     * This function fills in @a outbuf with an AP-REP message using information
     * from @a auth_context.
     *
     * If the flags in @a auth_context indicate that a sequence number should be
     * used (either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or
     * #KRB5_AUTH_CONTEXT_RET_SEQUENCE) and the local sequence number in @a
     * auth_context is 0, a new number will be generated with
     * krb5_generate_seq_number().
     *
     * Use krb5_free_data_contents() to free @a outbuf when it is no longer needed.
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_mk_rep(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        outbuf: *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Parse and decrypt a @c KRB_AP_REP message for DCE RPC.
     *
     * @param [in]  context         Library context
     * @param [in]  auth_context    Authentication context
     * @param [in]  inbuf           AP-REP message
     * @param [out] nonce           Sequence number from the decrypted reply
     *
     * This function parses, decrypts and verifies a message from @a inbuf and
     * fills in @a nonce with a decrypted reply sequence number.
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_rd_rep_dce(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        inbuf: *const crate::krb5_h::krb5_data,
        nonce: *mut crate::krb5_h::krb5_ui_4,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Format and encode a @c KRB_ERROR message.
     *
     * @param [in]  context         Library context
     * @param [in]  dec_err         Error structure to be encoded
     * @param [out] enc_err         Encoded error structure
     *
     * This function creates a @c KRB_ERROR message in @a enc_err.  Use
     * krb5_free_data_contents() to free @a enc_err when it is no longer needed.
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_mk_error(
        context: crate::krb5_h::krb5_context,
        dec_err: *const crate::krb5_h::krb5_error,
        enc_err: *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Merge two authorization data lists into a new list.
     *
     * @param [in]  context         Library context
     * @param [in]  inauthdat1      First list of @a krb5_authdata structures
     * @param [in]  inauthdat2      Second list of @a krb5_authdata structures
     * @param [out] outauthdat      Merged list of @a krb5_authdata structures
     *
     * Merge two authdata arrays, such as the array from a ticket
     * and authenticator.
     * Use krb5_free_authdata() to free @a outauthdat when it is no longer needed.
     *
     * @note The last array entry in @a inauthdat1 and @a inauthdat2
     * must be a NULL pointer.
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_merge_authdata(
        context: crate::krb5_h::krb5_context,
        inauthdat1: *const *mut crate::krb5_h::krb5_authdata,
        inauthdat2: *const *mut crate::krb5_h::krb5_authdata,
        outauthdat: *mut *mut *mut crate::krb5_h::krb5_authdata,
    ) -> crate::krb5_h::krb5_error_code;
    /* krb5_free.c */
    /* *
     * Free the storage assigned to a principal.
     *
     * @param [in] context          Library context
     * @param [in] val              Principal to be freed
     */
    #[no_mangle]
    pub fn krb5_free_principal(
        context: crate::krb5_h::krb5_context,
        val: crate::krb5_h::krb5_principal,
    );
    /* *
     * Free a krb5_authenticator structure.
     *
     * @param [in] context          Library context
     * @param [in] val              Authenticator structure to be freed
     *
     * This function frees the contents of @a val and the structure itself.
     */
    #[no_mangle]
    pub fn krb5_free_authenticator(
        context: crate::krb5_h::krb5_context,
        val: *mut crate::krb5_h::krb5_authenticator,
    );
    /* *
     * Free a krb5_data structure.
     *
     * @param [in] context          Library context
     * @param [in] val              Data structure to be freed
     *
     * This function frees the contents of @a val and the structure itself.
     */
    #[no_mangle]
    pub fn krb5_free_data(context: crate::krb5_h::krb5_context, val: *mut crate::krb5_h::krb5_data);
    /* *
     * Free the contents of a krb5_data structure and zero the data field.
     *
     * @param [in] context          Library context
     * @param [in] val              Data structure to free contents of
     *
     * This function frees the contents of @a val, but not the structure itself.
     */
    #[no_mangle]
    pub fn krb5_free_data_contents(
        context: crate::krb5_h::krb5_context,
        val: *mut crate::krb5_h::krb5_data,
    );
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
    #[no_mangle]
    pub fn krb5_us_timeofday(
        context: crate::krb5_h::krb5_context,
        seconds: *mut crate::krb5_h::krb5_timestamp,
        microseconds: *mut crate::krb5_h::krb5_int32,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_timeofday(
        context: crate::krb5_h::krb5_context,
        timeret: *mut crate::krb5_h::krb5_timestamp,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_rd_cred(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        creddata: *mut crate::krb5_h::krb5_data,
        creds_out: *mut *mut *mut crate::krb5_h::krb5_creds,
        rdata_out: *mut crate::krb5_h::krb5_replay_data,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_init(
        context: crate::krb5_h::krb5_context,
        auth_context: *mut crate::krb5_h::krb5_auth_context,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_free(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_setflags(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        flags: crate::krb5_h::krb5_int32,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_getflags(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        flags: *mut crate::krb5_h::krb5_int32,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_setaddrs(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        local_addr: *mut crate::krb5_h::krb5_address,
        remote_addr: *mut crate::krb5_h::krb5_address,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_getkey_k(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        key: *mut crate::krb5_h::krb5_key,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_getsendsubkey_k(
        ctx: crate::krb5_h::krb5_context,
        ac: crate::krb5_h::krb5_auth_context,
        key: *mut crate::krb5_h::krb5_key,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_getrecvsubkey_k(
        ctx: crate::krb5_h::krb5_context,
        ac: crate::krb5_h::krb5_auth_context,
        key: *mut crate::krb5_h::krb5_key,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_getlocalseqnumber(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        seqnumber: *mut crate::krb5_h::krb5_int32,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_getremoteseqnumber(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        seqnumber: *mut crate::krb5_h::krb5_int32,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_setrcache(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        rcache: crate::krb5_h::krb5_rcache,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_getauthenticator(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        authenticator: *mut *mut crate::krb5_h::krb5_authenticator,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Create a krb5_key from the enctype and key data in a keyblock.
     *
     * @param [in]  context      Library context
     * @param [in]  key_data     Keyblock
     * @param [out] out          Opaque key
     *
     * The reference count on a key @a out is set to 1.
     * Use krb5_k_free_key() to free @a out when it is no longer needed.
     *
     * @retval 0 Success; otherwise - KRB5_BAD_ENCTYPE
     */
    #[no_mangle]
    pub fn krb5_k_create_key(
        context: crate::krb5_h::krb5_context,
        key_data: *const crate::krb5_h::krb5_keyblock,
        out: *mut crate::krb5_h::krb5_key,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Retrieve a specified credentials from a credential cache.
     *
     * @param [in]  context         Library context
     * @param [in]  cache           Credential cache handle
     * @param [in]  flags           Flags bit mask
     * @param [in]  mcreds          Credentials to match
     * @param [out] creds           Credentials matching the requested value
     *
     * This function searches a credential cache for credentials matching @a mcreds
     * and returns it if found.
     *
     * Valid values for @a flags are:
     *
     * @li #KRB5_TC_MATCH_TIMES        The requested lifetime must be at least as
     *                                 great as in @a mcreds .
     * @li #KRB5_TC_MATCH_IS_SKEY      The @a is_skey field much match exactly.
     * @li #KRB5_TC_MATCH_FLAGS        Flags set in @a mcreds must be set.
     * @li #KRB5_TC_MATCH_TIMES_EXACT  The requested lifetime must match exactly.
     * @li #KRB5_TC_MATCH_FLAGS_EXACT  Flags must match exactly.
     * @li #KRB5_TC_MATCH_AUTHDATA     The authorization data must match.
     * @li #KRB5_TC_MATCH_SRV_NAMEONLY Only the name portion of the principal
     *                                 name must match, not the realm.
     * @li #KRB5_TC_MATCH_2ND_TKT      The second tickets must match.
     * @li #KRB5_TC_MATCH_KTYPE        The encryption key types must match.
     * @li #KRB5_TC_SUPPORTED_KTYPES   Check all matching entries that have any
     *                                 supported encryption type and return the
     *                                 one with the encryption type listed earliest.
     *
     * Use krb5_free_cred_contents() to free @a creds when it is no longer needed.
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_cc_retrieve_cred(
        context: crate::krb5_h::krb5_context,
        cache: crate::krb5_h::krb5_ccache,
        flags: crate::krb5_h::krb5_flags,
        mcreds: *mut crate::krb5_h::krb5_creds,
        creds: *mut crate::krb5_h::krb5_creds,
    ) -> crate::krb5_h::krb5_error_code;
    /*
     * end "keytab.h"
     */
    /*
     * begin "func-proto.h"
     */
    /* *< Use secure context configuration */
    /* *< Use KDC configuration if available */
    /* *
     * Create a krb5 library context.
     *
     * @param [out] context         Library context
     *
     * The @a context must be released by calling krb5_free_context() when
     * it is no longer needed.
     *
     * @warning Any program or module that needs the Kerberos code to not trust the
     * environment must use krb5_init_secure_context(), or clean out the
     * environment.
     *
     * @retval
     * 0 Success
     * @return
     * Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_init_context(
        context: *mut crate::krb5_h::krb5_context,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Set default TGS encryption types in a krb5_context structure.
     *
     * @param [in] context          Library context
     * @param [in] etypes           Encryption type(s) to set
     *
     * This function sets the default enctype list for TGS requests
     * made using @a context to @a etypes.
     *
     * @note This overrides the default list (from config file or built-in).
     *
     * @retval
     *  0    Success
     * @retval
     *  KRB5_PROG_ETYPE_NOSUPP Program lacks support for encryption type
     * @return
     * Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_set_default_tgs_enctypes(
        context: crate::krb5_h::krb5_context,
        etypes: *const crate::krb5_h::krb5_enctype,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Create a @c KRB_AP_REQ message using supplied credentials.
     *
     * @param [in]     context        Library context
     * @param [in,out] auth_context   Pre-existing or newly created auth context
     * @param [in]     ap_req_options @ref AP_OPTS options
     * @param [in]     in_data        Application data to be checksummed in the
     *                                authenticator, or NULL
     * @param [in]     in_creds       Credentials for the service with valid ticket
     *                                and key
     * @param [out]    outbuf         @c AP-REQ message
     *
     * Valid @a ap_req_options are:
     * @li #AP_OPTS_USE_SESSION_KEY - Use the session key when creating the
     *                                request used for user to user
     *                                authentication.
     * @li #AP_OPTS_MUTUAL_REQUIRED - Request a mutual authentication packet from
     *                                the reciever.
     * @li #AP_OPTS_USE_SUBKEY      - Generate a subsession key from the current
     *                                session key obtained from the credentials.
     *
     * This function creates a KRB_AP_REQ message using supplied credentials @a
     * in_creds.  @a auth_context may point to an existing auth context or to NULL,
     * in which case a new one will be created.  If @a in_data is non-null, a
     * checksum of it will be included in the authenticator contained in the
     * KRB_AP_REQ message.  Use krb5_free_data_contents() to free @a outbuf when it
     * is no longer needed.
     *
     * On successful return, the authenticator is stored in @a auth_context with
     * the @a client and @a checksum fields nulled out.  (This is to prevent
     * pointer-sharing problems; the caller should not need these fields anyway,
     * since the caller supplied them.)
     *
     * @sa krb5_mk_req()
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_mk_req_extended(
        context: crate::krb5_h::krb5_context,
        auth_context: *mut crate::krb5_h::krb5_auth_context,
        ap_req_options: crate::krb5_h::krb5_flags,
        in_data: *mut crate::krb5_h::krb5_data,
        in_creds: *mut crate::krb5_h::krb5_creds,
        outbuf: *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Format and encrypt a @c KRB_AP_REP message for DCE RPC.
     *
     * @param [in]  context         Library context
     * @param [in]  auth_context    Authentication context
     * @param [out] outbuf          @c AP-REP message
     *
     * Use krb5_free_data_contents() to free @a outbuf when it is no longer needed.
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_mk_rep_dce(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        outbuf: *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Parse and decrypt a @c KRB_AP_REP message.
     *
     * @param [in]  context         Library context
     * @param [in]  auth_context    Authentication context
     * @param [in]  inbuf           AP-REP message
     * @param [out] repl            Decrypted reply message
     *
     * This function parses, decrypts and verifies a message from @a inbuf and
     * fills in @a repl with a pointer to allocated memory containing the fields
     * from the encrypted response.
     *
     * Use krb5_free_ap_rep_enc_part() to free @a repl when it is no longer needed.
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_rd_rep(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        inbuf: *const crate::krb5_h::krb5_data,
        repl: *mut *mut crate::krb5_h::krb5_ap_rep_enc_part,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Decode a @c KRB-ERROR message.
     *
     * @param [in]  context         Library context
     * @param [in]  enc_errbuf      Encoded error message
     * @param [out] dec_error       Decoded error message
     *
     * This function processes @c KRB-ERROR message @a enc_errbuf and returns
     * an allocated structure @a dec_error containing the error message.
     * Use krb5_free_error() to free @a dec_error when it is no longer needed.
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_rd_error(
        context: crate::krb5_h::krb5_context,
        enc_errbuf: *const crate::krb5_h::krb5_data,
        dec_error: *mut *mut crate::krb5_h::krb5_error,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Free a krb5_keyblock structure.
     *
     * @param [in] context          Library context
     * @param [in] val              Keyblock to be freed
     *
     * This function frees the contents of @a val and the structure itself.
     */
    #[no_mangle]
    pub fn krb5_free_keyblock(
        context: crate::krb5_h::krb5_context,
        val: *mut crate::krb5_h::krb5_keyblock,
    );
    /* *
     * Free a krb5_ap_rep_enc_part structure.
     *
     * @param [in] context          Library context
     * @param [in] val              AP-REP enc part to be freed
     *
     * This function frees the contents of @a val and the structure itself.
     */
    #[no_mangle]
    pub fn krb5_free_ap_rep_enc_part(
        context: crate::krb5_h::krb5_context,
        val: *mut crate::krb5_h::krb5_ap_rep_enc_part,
    );
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
    #[no_mangle]
    pub fn krb5_fwd_tgt_creds(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        rhost: *const i8,
        client: crate::krb5_h::krb5_principal,
        server: crate::krb5_h::krb5_principal,
        cc: crate::krb5_h::krb5_ccache,
        forwardable: i32,
        outbuf: *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_set_checksum_func(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        func: crate::krb5_h::krb5_mk_req_checksum_func,
        data: *mut libc::c_void,
    ) -> crate::krb5_h::krb5_error_code;
    /* *
     * Set the session key in an auth context.
     *
     * @param [in] context          Library context
     * @param [in] auth_context     Authentication context
     * @param [in] keyblock         User key
     *
     * @retval 0 Success; otherwise - Kerberos error codes
     */
    #[no_mangle]
    pub fn krb5_auth_con_setuseruserkey(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        keyblock: *mut crate::krb5_h::krb5_keyblock,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_getsendsubkey(
        ctx: crate::krb5_h::krb5_context,
        ac: crate::krb5_h::krb5_auth_context,
        keyblock: *mut *mut crate::krb5_h::krb5_keyblock,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_setsendsubkey_k(
        ctx: crate::krb5_h::krb5_context,
        ac: crate::krb5_h::krb5_auth_context,
        key: crate::krb5_h::krb5_key,
    ) -> crate::krb5_h::krb5_error_code;
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
    #[no_mangle]
    pub fn krb5_auth_con_set_req_cksumtype(
        context: crate::krb5_h::krb5_context,
        auth_context: crate::krb5_h::krb5_auth_context,
        cksumtype: crate::krb5_h::krb5_cksumtype,
    ) -> crate::krb5_h::krb5_error_code;
}
// =============== BEGIN krb5_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_crypto_iov {
    pub flags: crate::krb5_h::krb5_cryptotype,
    pub data: crate::krb5_h::krb5_data,
}
/* checksum type */

/* *
 * Structure to describe a region of text to be encrypted or decrypted.
 *
 * The @a flags member describes the type of the iov.
 * The @a data member points to the memory that will be manipulated.
 * All iov APIs take a pointer to the first element of an array of krb5_crypto_iov's
 * along with the size of that array. Buffer contents are manipulated in-place;
 * data is overwritten. Callers must allocate the right number of krb5_crypto_iov
 * structures before calling into an iov API.
 */
pub type krb5_crypto_iov = crate::krb5_h::_krb5_crypto_iov;
pub type krb5_init_creds_context = *mut crate::krb5_h::_krb5_init_creds_context;
pub type krb5_tkt_creds_context = *mut crate::krb5_h::_krb5_tkt_creds_context;
pub type krb5_cryptotype = crate::krb5_h::krb5_int32;
pub type krb5_const_pointer = *const libc::c_void;
/* *< Requested options */

/* *< Ticket */

/* *< Encrypted authenticator */

/*
 * end "safepriv.h"
 */

/*
 * begin "ccache.h"
 */

/* * Cursor for sequential lookup */
pub type krb5_cc_cursor = crate::krb5_h::krb5_pointer;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_prompt {
    pub prompt: *mut i8,
    pub hidden: i32,
    pub reply: *mut crate::krb5_h::krb5_data,
}
/* *< Principal of this key */

/* *< Time entry written to keytable */

/* *< Key version number */

/* *< The secret key */

/* flags for recvauth */

/* initial ticket api functions */

/* * Text for prompt used in prompter callback function.  */
pub type krb5_prompt = crate::krb5_h::_krb5_prompt;
/* *< The prompt to show to the user */

/* *< Boolean; informative prompt or hidden (e.g. PIN) */

/* *< Must be allocated before call to  prompt routine */

/* * Pointer to a prompter callback function. */
pub type krb5_prompter_fct = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut libc::c_void,
        _: *const i8,
        _: *const i8,
        _: i32,
        _: *mut crate::krb5_h::krb5_prompt,
    ) -> crate::krb5_h::krb5_error_code,
>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_get_init_creds_opt {
    pub flags: crate::krb5_h::krb5_flags,
    pub tkt_life: crate::krb5_h::krb5_deltat,
    pub renew_life: crate::krb5_h::krb5_deltat,
    pub forwardable: i32,
    pub proxiable: i32,
    pub etype_list: *mut crate::krb5_h::krb5_enctype,
    pub etype_list_length: i32,
    pub address_list: *mut *mut crate::krb5_h::krb5_address,
    pub preauth_list: *mut crate::krb5_h::krb5_preauthtype,
    pub preauth_list_length: i32,
    pub salt: *mut crate::krb5_h::krb5_data,
}
/* * Store options for @c _krb5_get_init_creds */
pub type krb5_get_init_creds_opt = crate::krb5_h::_krb5_get_init_creds_opt;
/* typedef struct _profile_t *profile_t; */

/*
 * begin wordsize.h
 */

/*
 * Word-size related definition.
 */
pub type krb5_octet = crate::stdlib::uint8_t;
pub type krb5_int16 = crate::stdlib::int16_t;
pub type krb5_ui_2 = crate::stdlib::uint16_t;
pub type krb5_int32 = crate::stdlib::int32_t;
pub type krb5_ui_4 = crate::stdlib::uint32_t;
/* this strange form is necessary since - is a unary operator, not a sign
indicator */

/* this strange form is necessary since - is a unary operator, not a sign
indicator */

/*
 * end wordsize.h
 */

/*
 * begin "base-defs.h"
 */

/*
 * Basic definitions for Kerberos V5 library
 */
pub type krb5_boolean = u32;
pub type krb5_msgtype = u32;
pub type krb5_kvno = u32;
pub type krb5_addrtype = crate::krb5_h::krb5_int32;
pub type krb5_enctype = crate::krb5_h::krb5_int32;
pub type krb5_cksumtype = crate::krb5_h::krb5_int32;
pub type krb5_authdatatype = crate::krb5_h::krb5_int32;
pub type krb5_keyusage = crate::krb5_h::krb5_int32;
pub type krb5_preauthtype = crate::krb5_h::krb5_int32;
/* This may change, later on */
pub type krb5_flags = crate::krb5_h::krb5_int32;
/* *
 * Represents a timestamp in seconds since the POSIX epoch.  This legacy type
 * is used frequently in the ABI, but cannot represent timestamps after 2038 as
 * a positive number.  Code which uses this type should cast values of it to
 * uint32_t so that negative values are treated as timestamps between 2038 and
 * 2106 on platforms with 64-bit time_t.
 */
pub type krb5_timestamp = crate::krb5_h::krb5_int32;
pub type krb5_deltat = crate::krb5_h::krb5_int32;
/* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
pub type krb5_error_code = crate::krb5_h::krb5_int32;
pub type krb5_magic = crate::krb5_h::krb5_error_code;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_data {
    pub magic: crate::krb5_h::krb5_magic,
    pub length: u32,
    pub data: *mut i8,
}
pub type krb5_data = crate::krb5_h::_krb5_data;
/* Originally used to recognize AFS and default salts.  No longer used. */
pub type krb5_pointer = *mut libc::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_principal_data {
    pub magic: crate::krb5_h::krb5_magic,
    pub realm: crate::krb5_h::krb5_data,
    pub data: *mut crate::krb5_h::krb5_data,
    pub length: crate::krb5_h::krb5_int32,
    pub type_0: crate::krb5_h::krb5_int32,
}
pub type krb5_principal = *mut crate::krb5_h::krb5_principal_data;
/*
 * Per V5 spec on definition of principal types
 */

/* *<  Name type not known */

/* *< Just the name of the principal
as in DCE, or for users */

/* *< Service and other unique instance (krbtgt) */

/* *< Service with host name as instance
(telnet, rcommands) */

/* *< Service with host as remaining components */

/* *< Unique ID */

/* *< PKINIT */

/* *< Name in form of SMTP email name */

/* *< Windows 2000 UPN */

/* *< Well-known (special) principal */

/* *< First component of
NT_WELLKNOWN principals */

/* *< Windows 2000 UPN and SID */

/* *< NT 4 style name */

/* *< NT 4 style name and SID */

/* * Constant version of krb5_principal_data */
pub type krb5_const_principal = *const crate::krb5_h::krb5_principal_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_address {
    pub magic: crate::krb5_h::krb5_magic,
    pub addrtype: crate::krb5_h::krb5_addrtype,
    pub length: u32,
    pub contents: *mut crate::krb5_h::krb5_octet,
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
pub type krb5_address = crate::krb5_h::_krb5_address;
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
pub type krb5_post_recv_fn = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut libc::c_void,
        _: crate::krb5_h::krb5_error_code,
        _: *const crate::krb5_h::krb5_data,
        _: *const crate::krb5_h::krb5_data,
        _: *const crate::krb5_h::krb5_data,
        _: *mut *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code,
>;
/* not yet in the spec... */

/* macros to determine if a type is a local type */

/*
 * end "hostaddr.h"
 */
pub type krb5_context = *mut crate::k5_int_h::_krb5_context;
pub type krb5_pre_send_fn = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *mut libc::c_void,
        _: *const crate::krb5_h::krb5_data,
        _: *const crate::krb5_h::krb5_data,
        _: *mut *mut crate::krb5_h::krb5_data,
        _: *mut *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code,
>;
pub type krb5_trace_callback = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: *const crate::krb5_h::krb5_trace_info,
        _: *mut libc::c_void,
    ) -> (),
>;
pub type krb5_trace_info = crate::krb5_h::_krb5_trace_info;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_trace_info {
    pub message: *const i8,
}
/*
 * Prompter enhancements
 */

/* * Prompt for password */

/* * Prompt for new password (during password change) */

/* * Prompt for new password again */

/* * Prompt for preauthentication data (such as an OTP value) */
pub type krb5_prompt_type = crate::krb5_h::krb5_int32;
pub type krb5_auth_context = *mut crate::krb5_h::_krb5_auth_context;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_keyblock {
    pub magic: crate::krb5_h::krb5_magic,
    pub enctype: crate::krb5_h::krb5_enctype,
    pub length: u32,
    pub contents: *mut crate::krb5_h::krb5_octet,
}
/*
 * begin "encryption.h"
 */

/* * Exposed contents of a key. */
pub type krb5_keyblock = crate::krb5_h::_krb5_keyblock;
/* *
 * Opaque identifier for a key.
 *
 * Use with the krb5_k APIs for better performance for repeated operations with
 * the same key and usage.  Key identifiers must not be used simultaneously
 * within multiple threads, as they may contain mutable internal state and are
 * not mutex-protected.
 */
pub type krb5_key = *mut crate::k5_int_h::krb5_key_st;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_checksum {
    pub magic: crate::krb5_h::krb5_magic,
    pub checksum_type: crate::krb5_h::krb5_cksumtype,
    pub length: u32,
    pub contents: *mut crate::krb5_h::krb5_octet,
}
pub type krb5_checksum = crate::krb5_h::_krb5_checksum;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_enc_data {
    pub magic: crate::krb5_h::krb5_magic,
    pub enctype: crate::krb5_h::krb5_enctype,
    pub kvno: crate::krb5_h::krb5_kvno,
    pub ciphertext: crate::krb5_h::krb5_data,
}
pub type krb5_enc_data = crate::krb5_h::_krb5_enc_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_ticket_times {
    pub authtime: crate::krb5_h::krb5_timestamp,
    pub starttime: crate::krb5_h::krb5_timestamp,
    pub endtime: crate::krb5_h::krb5_timestamp,
    pub renew_till: crate::krb5_h::krb5_timestamp,
}
/* checksum type */

/* *< Generate a subsession key
from the current session key
obtained from the
credentials */

/* #define      AP_OPTS_RESERVED        0x10000000 */

/* #define      AP_OPTS_RESERVED        0x08000000 */

/* #define      AP_OPTS_RESERVED        0x04000000 */

/* #define      AP_OPTS_RESERVED        0x02000000 */

/* #define      AP_OPTS_RESERVED        0x01000000 */

/* #define      AP_OPTS_RESERVED        0x00800000 */

/* #define      AP_OPTS_RESERVED        0x00400000 */

/* #define      AP_OPTS_RESERVED        0x00200000 */

/* #define      AP_OPTS_RESERVED        0x00100000 */

/* #define      AP_OPTS_RESERVED        0x00080000 */

/* #define      AP_OPTS_RESERVED        0x00040000 */

/* #define      AP_OPTS_RESERVED        0x00020000 */

/* #define      AP_OPTS_RESERVED        0x00010000 */

/* #define      AP_OPTS_RESERVED        0x00008000 */

/* #define      AP_OPTS_RESERVED        0x00004000 */

/* #define      AP_OPTS_RESERVED        0x00002000 */

/* #define      AP_OPTS_RESERVED        0x00001000 */

/* #define      AP_OPTS_RESERVED        0x00000800 */

/* #define      AP_OPTS_RESERVED        0x00000400 */

/* #define      AP_OPTS_RESERVED        0x00000200 */

/* #define      AP_OPTS_RESERVED        0x00000100 */

/* #define      AP_OPTS_RESERVED        0x00000080 */

/* #define      AP_OPTS_RESERVED        0x00000040 */

/* #define      AP_OPTS_RESERVED        0x00000020 */

/* #define      AP_OPTS_RESERVED        0x00000010 */

/* #define      AP_OPTS_RESERVED        0x00000008 */

/* #define      AP_OPTS_RESERVED        0x00000004 */

/* * @} */

/* end of AP_OPTS group */

/* definitions for ad_type fields. */

/* Ticket flags */

/* flags are 32 bits; each host is responsible to put the 4 bytes
representing these bits into net order before transmission */

/* #define      TKT_FLG_RESERVED        0x80000000 */

/* #define      TKT_FLG_RESERVED        0x00004000 */

/* #define      TKT_FLG_RESERVED        0x00002000 */

/* #define      TKT_FLG_RESERVED        0x00001000 */

/* #define      TKT_FLG_RESERVED        0x00000800 */

/* #define      TKT_FLG_RESERVED        0x00000400 */

/* #define      TKT_FLG_RESERVED        0x00000200 */

/* #define      TKT_FLG_RESERVED        0x00000100 */

/* #define      TKT_FLG_RESERVED        0x00000080 */

/* #define      TKT_FLG_RESERVED        0x00000040 */

/* #define      TKT_FLG_RESERVED        0x00000020 */

/* #define      TKT_FLG_RESERVED        0x00000010 */

/* #define      TKT_FLG_RESERVED        0x00000008 */

/* #define      TKT_FLG_RESERVED        0x00000004 */

/* #define      TKT_FLG_RESERVED        0x00000002 */

/* #define      TKT_FLG_RESERVED        0x00000001 */

/* definitions for lr_type fields. */

/* definitions for msec direction bit for KRB_SAFE, KRB_PRIV */

/*
 * end "fieldbits.h"
 */

/*
 * begin "proto.h"
 */

/* * Protocol version number */

/* Message types */

/* *< Initial authentication request */

/* *< Response to AS request */

/* *< Ticket granting server request */

/* *< Response to TGS request */

/* *< Auth req to application server */

/* *< Response to mutual AP request */

/* *< Safe application message */

/* *< Private application message */

/* *< Cred forwarding message */

/* *< Error response */

/* LastReq types */

/* PADATA types */

/* *< RFC 4120 */

/* *< RFC 4120 */

/* Not used */

/* *< timestamp encrypted in key. RFC 4120 */

/* *< SecurId passcode. RFC 4120 */

/* *< Sesame project. RFC 4120 */

/* *< OSF DCE. RFC 4120 */

/* *< Cybersafe. RFC 4120 */

/* *< Cygnus. RFC 4120, 3961 */

/* *< Etype info for preauth. RFC 4120 */

/* *< SAM/OTP */

/* *< SAM/OTP */

/* *< PKINIT */

/* *< PKINIT */

/* *< PKINIT. RFC 4556 */

/* *< PKINIT. RFC 4556 */

/* *< RFC 4120 */

/* *< RFC 4120 */

/* *< Windows 2000 referrals. RFC 6820 */

/* *< SAM/OTP. RFC 4120 */

/* *< Embedded in typed data. RFC 4120 */

/* *< draft referral system */

/* *< draft challenge system, updated */

/* *< draft challenge system, updated */

/* MS-KILE */

/* *< include Windows PAC */

/* *< username protocol transition request */

/* *< certificate protocol transition request */

/* *< AS checksum */

/* *< RFC 6113 */

/* *< RFC 6113 */

/* *< RFC 6113 */

/* *< RFC 6113 */

/* *< RFC 6560 section 4.1 */

/* *< RFC 6560 section 4.2 */

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
pub type krb5_ticket_times = crate::krb5_h::_krb5_ticket_times;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_authdata {
    pub magic: crate::krb5_h::krb5_magic,
    pub ad_type: crate::krb5_h::krb5_authdatatype,
    pub length: u32,
    pub contents: *mut crate::krb5_h::krb5_octet,
}
/* *< Time at which KDC issued the initial ticket that corresponds to this ticket */

/* XXX ? should ktime in KDC_REP == authtime
in ticket? otherwise client can't get this */

/* *< optional in ticket, if not present, use @a authtime */

/* *< Ticket expiration time */

/* *< Latest time at which renewal of ticket can be valid */

/* * Structure for auth data */
pub type krb5_authdata = crate::krb5_h::_krb5_authdata;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_transited {
    pub magic: crate::krb5_h::krb5_magic,
    pub tr_type: crate::krb5_h::krb5_octet,
    pub tr_contents: crate::krb5_h::krb5_data,
}
/* *< ADTYPE */

/* *< Length of data  */

/* *< Data */

/* * Structure for transited encoding */
pub type krb5_transited = crate::krb5_h::_krb5_transited;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_enc_tkt_part {
    pub magic: crate::krb5_h::krb5_magic,
    pub flags: crate::krb5_h::krb5_flags,
    pub session: *mut crate::krb5_h::krb5_keyblock,
    pub client: crate::krb5_h::krb5_principal,
    pub transited: crate::krb5_h::krb5_transited,
    pub times: crate::krb5_h::krb5_ticket_times,
    pub caddrs: *mut *mut crate::krb5_h::krb5_address,
    pub authorization_data: *mut *mut crate::krb5_h::krb5_authdata,
}
/* *< Transited encoding type */

/* *< Contents */

/* * Encrypted part of ticket. */
pub type krb5_enc_tkt_part = crate::krb5_h::_krb5_enc_tkt_part;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_ticket {
    pub magic: crate::krb5_h::krb5_magic,
    pub server: crate::krb5_h::krb5_principal,
    pub enc_part: crate::krb5_h::krb5_enc_data,
    pub enc_part2: *mut crate::krb5_h::krb5_enc_tkt_part,
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
pub type krb5_ticket = crate::krb5_h::_krb5_ticket;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_authenticator {
    pub magic: crate::krb5_h::krb5_magic,
    pub client: crate::krb5_h::krb5_principal,
    pub checksum: *mut crate::krb5_h::krb5_checksum,
    pub cusec: crate::krb5_h::krb5_int32,
    pub ctime: crate::krb5_h::krb5_timestamp,
    pub subkey: *mut crate::krb5_h::krb5_keyblock,
    pub seq_number: crate::krb5_h::krb5_ui_4,
    pub authorization_data: *mut *mut crate::krb5_h::krb5_authdata,
}
/* cleartext portion */

/* *< server name/realm */

/* *< encryption type, kvno, encrypted encoding */

/* *< ptr to decrypted version, if available */

/* the unencrypted version */

/* *
 * Ticket authenticator.
 *
 * The C representation of an unencrypted authenticator.
 */
pub type krb5_authenticator = crate::krb5_h::_krb5_authenticator;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_creds {
    pub magic: crate::krb5_h::krb5_magic,
    pub client: crate::krb5_h::krb5_principal,
    pub server: crate::krb5_h::krb5_principal,
    pub keyblock: crate::krb5_h::krb5_keyblock,
    pub times: crate::krb5_h::krb5_ticket_times,
    pub is_skey: crate::krb5_h::krb5_boolean,
    pub ticket_flags: crate::krb5_h::krb5_flags,
    pub addresses: *mut *mut crate::krb5_h::krb5_address,
    pub ticket: crate::krb5_h::krb5_data,
    pub second_ticket: crate::krb5_h::krb5_data,
    pub authdata: *mut *mut crate::krb5_h::krb5_authdata,
}
/* *< client name/realm */

/* *< checksum, includes type, optional */

/* *< client usec portion */

/* *< client sec portion */

/* *< true session key, optional */

/* *< sequence #, optional */

/* *< authoriazation data */

/* * Credentials structure including ticket, session key, and lifetime info. */
pub type krb5_creds = crate::krb5_h::_krb5_creds;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_pa_data {
    pub magic: crate::krb5_h::krb5_magic,
    pub pa_type: crate::krb5_h::krb5_preauthtype,
    pub length: u32,
    pub contents: *mut crate::krb5_h::krb5_octet,
}
/* *< client's principal identifier */

/* *< server's principal identifier */

/* *< session encryption key info */

/* *< lifetime info */

/* *< true if ticket is encrypted in
another ticket's skey */

/* *< flags in ticket */

/* *< addrs in ticket */

/* *< ticket string itself */

/* *< second ticket, if related to
ticket (via DUPLICATE-SKEY or
ENC-TKT-IN-SKEY) */

/* *< authorization data */

/* * Pre-authentication data */
pub type krb5_pa_data = crate::krb5_h::_krb5_pa_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_kdc_req {
    pub magic: crate::krb5_h::krb5_magic,
    pub msg_type: crate::krb5_h::krb5_msgtype,
    pub padata: *mut *mut crate::krb5_h::krb5_pa_data,
    pub kdc_options: crate::krb5_h::krb5_flags,
    pub client: crate::krb5_h::krb5_principal,
    pub server: crate::krb5_h::krb5_principal,
    pub from: crate::krb5_h::krb5_timestamp,
    pub till: crate::krb5_h::krb5_timestamp,
    pub rtime: crate::krb5_h::krb5_timestamp,
    pub nonce: crate::krb5_h::krb5_int32,
    pub nktypes: i32,
    pub ktype: *mut crate::krb5_h::krb5_enctype,
    pub addresses: *mut *mut crate::krb5_h::krb5_address,
    pub authorization_data: crate::krb5_h::krb5_enc_data,
    pub unenc_authdata: *mut *mut crate::krb5_h::krb5_authdata,
    pub second_ticket: *mut *mut crate::krb5_h::krb5_ticket,
}
/* *< Preauthentication data type */

/* *< Length of data */

/* *< Data */

/* * C representation of KDC-REQ protocol message, including KDC-REQ-BODY */
pub type krb5_kdc_req = crate::krb5_h::_krb5_kdc_req;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_error {
    pub magic: crate::krb5_h::krb5_magic,
    pub ctime: crate::krb5_h::krb5_timestamp,
    pub cusec: crate::krb5_h::krb5_int32,
    pub susec: crate::krb5_h::krb5_int32,
    pub stime: crate::krb5_h::krb5_timestamp,
    pub error: crate::krb5_h::krb5_ui_4,
    pub client: crate::krb5_h::krb5_principal,
    pub server: crate::krb5_h::krb5_principal,
    pub text: crate::krb5_h::krb5_data,
    pub e_data: crate::krb5_h::krb5_data,
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

/* * Error message structure */
pub type krb5_error = crate::krb5_h::_krb5_error;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_ap_req {
    pub magic: crate::krb5_h::krb5_magic,
    pub ap_options: crate::krb5_h::krb5_flags,
    pub ticket: *mut crate::krb5_h::krb5_ticket,
    pub authenticator: crate::krb5_h::krb5_enc_data,
}
/* some of these may be meaningless in certain contexts */

/* *< Client sec portion; optional */

/* *< Client usec portion; optional */

/* *< Server usec portion */

/* *< Server sec portion */

/* *< Error code (protocol error #'s) */

/* *< Client principal and realm */

/* *< Server principal and realm */

/* *< Descriptive text */

/* *< Additional error-describing data */

/* * Authentication header. */
pub type krb5_ap_req = crate::krb5_h::_krb5_ap_req;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_replay_data {
    pub timestamp: crate::krb5_h::krb5_timestamp,
    pub usec: crate::krb5_h::krb5_int32,
    pub seq: crate::krb5_h::krb5_ui_4,
}
pub type krb5_ccache = *mut crate::krb5_h::_krb5_ccache;
pub type krb5_rcache = *mut crate::krb5_h::krb5_rc_st;
/* *< Requested options */

/* *< Ticket */

/* *< Encrypted authenticator */

/*
 * end "rcache.h"
 */

/*
 * begin "keytab.h"
 */

/* XXX */

/* *< Long enough for MAXPATHLEN + some extra */
pub type krb5_kt_cursor = crate::krb5_h::krb5_pointer;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_keytab_entry_st {
    pub magic: crate::krb5_h::krb5_magic,
    pub principal: crate::krb5_h::krb5_principal,
    pub timestamp: crate::krb5_h::krb5_timestamp,
    pub vno: crate::krb5_h::krb5_kvno,
    pub key: crate::krb5_h::krb5_keyblock,
}
/* * A key table entry. */
pub type krb5_keytab_entry = crate::krb5_h::krb5_keytab_entry_st;
pub type krb5_keytab = *mut crate::k5_int_h::_krb5_kt;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_ap_rep_enc_part {
    pub magic: crate::krb5_h::krb5_magic,
    pub ctime: crate::krb5_h::krb5_timestamp,
    pub cusec: crate::krb5_h::krb5_int32,
    pub subkey: *mut crate::krb5_h::krb5_keyblock,
    pub seq_number: crate::krb5_h::krb5_ui_4,
}
/* *< Requested options */

/* *< Ticket */

/* *< Encrypted authenticator */

/* * Cleartext that is encrypted and put into @c _krb5_ap_rep.  */
pub type krb5_ap_rep_enc_part = crate::krb5_h::_krb5_ap_rep_enc_part;
/* *< Client time, seconds portion */

/* *< Client time, microseconds portion */

/* *< Subkey (optional) */

/* *< Sequence number */

/* Flags for krb5_auth_con_genaddrs(). */

/* * Generate the local network address. */

/* * Generate the remote network address.  */

/* * Generate the local network address and the local port. */

/* * Generate the remote network address and the remote port. */

/* * Type of function used as a callback to generate checksum data for mk_req */
pub type krb5_mk_req_checksum_func = Option<
    unsafe extern "C" fn(
        _: crate::krb5_h::krb5_context,
        _: crate::krb5_h::krb5_auth_context,
        _: *mut libc::c_void,
        _: *mut *mut crate::krb5_h::krb5_data,
    ) -> crate::krb5_h::krb5_error_code,
>;
