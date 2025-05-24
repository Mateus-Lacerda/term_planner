pkgname=term_planner
pkgver=0.1.0
pkgrel=1
pkgdesc="A simple dunst-based planner"
arch=('x86_64')
depends=('glibc' 'rust')
optdepends=('dunst: notificador via notify-send para Dunst')
license=('MIT')
source=(".")
noextract=(".")
sha256sums=('SKIP')

build() {
  cd "$srcdir"
  cargo build --release
}

package() {
  cd "$srcdir"
  install -Dm755 "target/release/term_planner" \
                "$pkgdir/usr/bin/term_planner"
  install -Dm644 systemd/term_planner-notify.service \
                "$pkgdir/usr/lib/systemd/user/term_planner-notify.service"
  install -Dm644 systemd/term_planner-notify.timer \
                "$pkgdir/usr/lib/systemd/user/term_planner-notify.timer"
}

