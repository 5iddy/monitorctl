# This is an example PKGBUILD file. Use this as a start to creating your own,
# and remove these comments. For more information, see 'man PKGBUILD'.
# NOTE: Please fill out the license field for your package! If it is unknown,
# then please put 'unknown'.

# Maintainer: 5iddy <siddhuyeturi@gmail.com>
pkgname=monitorctl
pkgver=v0.1.1
pkgrel=1
epoch=
pkgdesc="A way to control monitor brightness through the command line. You can change the brightness of all monitors simultaneously."
arch=('x86_64')
url="https://github.com/5iddy/monitorctl"
license=('NONE')
makedepends=('rustup')

prepare() {
	rustup default stable
}

build() {
	cargo build --profile release --package monitorctl
}

package() {
	install -Dm0755 "../target/release/monitorctl" "$pkgdir/usr/bin/monitorctl"
}
