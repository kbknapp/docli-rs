# Maintainer: Kevin K. <kbknapp@gmail.com> 

_pkgname=docli
pkgname=${_pkgname}-git
pkgver=0.1.0-beta
pkgrel=1
pkgdesc='command line utility for managing digitalocean infrastructure'
arch=('x86_64')
url="https://kbknapp.github.io/${_pkgname}-rs"
license=('MIT')
makedepends=('git' 'cargo' 'rust')
provides=("${_pkgname}")
conflicts=("${_pkgname}-bin")
source=("git://github.com/kbknapp/${_pkgname}-rs.git")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/$_pkgname"
  git rev-list --count HEAD
}

build() {
  cd "$srcdir/$_pkgname"
  cargo build --release
}

package() {
  cd "$srcdir/$_pkgname"
  mkdir -p "${pkgdir}/usr/bin"
  cp -p "target/release/${_pkgname}" "${pkgdir}/usr/bin"
}
