# gssapi-rs

It's not good.


## Refactoring

- [ ] OM_uint32
   - [ ] return arg
   - [ ] `return 0 as i32 as OM_uint32`.
- [ ] Bad Casts
  - [ ] `1 as i32 as usize`
  - [ ] `return 0 as i32 as isize`
- [ ] Get rid of raw pointers. Unsafe to deref them.
