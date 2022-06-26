# Maintainer: Kaporos <theo@daron.be>
pkgname=fancy_commit
pkgver=1.0.0
pkgrel=1
pkgdesc="A tool to standardize and follow git commit naming conventions"
arch=('x86_64')
url="https://github.com/Kaporos/fancy_commit"
license=('MIT' 'custom')
depends=('gcc-libs')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/$pkgver.tar.gz")
md5sums=("4758afd200cc2784005cd9a559d658b7")
build() {
  cd "$pkgname-$pkgver"
  cargo build --release 
}

package() {
  cd "$pkgname-$pkgver"
  local OUT_DIR=$(<out_dir)
  install -Dm755 "target/release/fancy_commit" "$pkgdir/usr/bin/fancy_commit"
}
