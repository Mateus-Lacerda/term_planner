# Maintainer: Mateus Lacerda <mlacerdam.ai@gmail.com>
pkgname=term_planner
pkgver=0.1.0
pkgrel=1
pkgdesc="A simple planner with dunst based notifications"
arch=('x86_64')
url="https://github.com/Mateus-Lacerda/term_planner"
license=('MIT')
depends=('glibc')
makedepends=('rust' 'cargo')
optdepends=('dunst: notificador via notify-send para Dunst')

source=("git+https://github.com/Mateus-Lacerda/term_planner.git#branch=main")
sha256sums=('SKIP')

build() {
  cd "$srcdir/term_planner"
  cargo build --release --locked
}

package() {
  cd "$srcdir/term_planner"

  install -Dm755 "target/release/term_planner" \
                "$pkgdir/usr/bin/term_planner"

  install -Dm644 systemd/term_planner-notify.service \
                "$pkgdir/usr/lib/systemd/user/term_planner-notify.service"
  install -Dm644 systemd/term_planner-notify.timer \
                "$pkgdir/usr/lib/systemd/user/term_planner-notify.timer"
}

