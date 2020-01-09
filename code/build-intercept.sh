#!/bin/sh

prepare() {
  # cf https://bugs.gentoo.org/show_bug.cgi?id=448778
  patch -p1 < krb5-config_LDFLAGS.patch

  # FS#25384
  sed -i "/KRB5ROOT=/s/\/local//" src/util/ac_check_krb5.m4
}

build() {
   cd src
   export CFLAGS+=" -fPIC -fno-strict-aliasing -fstack-protector-all"
   export CPPFLAGS+=" -I/usr/include/et"

   # Original:
   # ./configure --prefix=/usr \
   #             --sbindir=/usr/bin \
   #             --sysconfdir=/etc \
   #             --localstatedir=/var/lib \
   #             --enable-shared \
   #             --with-system-et \
   #             --with-system-ss \
   #             --disable-rpath \
   #             --without-tcl \
   #             --enable-dns-for-realm \
   #             --with-ldap \
   #             --without-system-verto
   ./configure
	intercept-build make
}

convert_rust() {
	c2rust transpile -e -r --reduce-type-annotations compile_commands.json -o $HOME/dev/krb5-rs
}

prepare && build && convert_rust
