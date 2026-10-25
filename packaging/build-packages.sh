#!/usr/bin/env bash
set -euo pipefail

version="${1:?usage: build-packages.sh VERSION ARCH (x86_64|aarch64)}"
arch="${2:?usage: build-packages.sh VERSION ARCH (x86_64|aarch64)}"

case "$arch" in
    x86_64) deb_arch=amd64 ;;
    aarch64) deb_arch=arm64 ;;
    *) echo "architecture inconnue : $arch" >&2; exit 1 ;;
esac

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
binary="$root/target/release/cera"
dist="$root/dist"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

[ -x "$binary" ] || { echo "binaire introuvable : $binary" >&2; exit 1; }
mkdir -p "$dist"

archive_name="cera-${version}-linux-${arch}"
archive_dir="$work/$archive_name"
mkdir -p "$archive_dir"
cp "$binary" "$archive_dir/cera"
cp -r "$root/plugins" "$archive_dir/plugins"
cp "$root/README.md" "$root/LICENSE" "$archive_dir/"
tar -C "$work" -czf "$dist/$archive_name.tar.gz" "$archive_name"

stage="$work/stage"
mkdir -p "$stage/usr/bin" "$stage/usr/share/cera" "$stage/usr/share/doc/cera"
install -m 755 "$binary" "$stage/usr/bin/cera"
cp -r "$root/plugins" "$stage/usr/share/cera/plugins"
find "$stage/usr/share/cera" -type d -exec chmod 755 {} +
find "$stage/usr/share/cera" -type f -exec chmod 644 {} +
chmod 755 "$stage/usr/share/cera/plugins/install.sh"
install -m 644 "$root/README.md" "$stage/usr/share/doc/cera/README.md"
install -m 644 "$root/LICENSE" "$stage/usr/share/doc/cera/copyright"

deb_root="$work/deb"
cp -a "$stage" "$deb_root"
mkdir -p "$deb_root/DEBIAN"
installed_size="$(du -sk "$stage" | cut -f1)"
cat > "$deb_root/DEBIAN/control" <<CONTROL
Package: cera
Version: $version
Section: editors
Priority: optional
Architecture: $deb_arch
Depends: libc6, python3
Installed-Size: $installed_size
Maintainer: kelian-4 <kelian-4@users.noreply.github.com>
Homepage: https://github.com/kelian-4/cera
Description: Terminal text editor with a plugin system
 Cera is a simple terminal text editor written in Rust, with a plugin
 system (C, C++, Rust, JS, Python) and syntax highlighting for 70 languages.
CONTROL
dpkg-deb --root-owner-group --build "$deb_root" "$dist/cera_${version}_${deb_arch}.deb"

rpm_top="$work/rpm"
mkdir -p "$rpm_top"
rpmbuild -bb "$root/packaging/cera.spec" \
    --target "${arch}-linux" \
    --define "_topdir $rpm_top" \
    --define "cera_version $version" \
    --define "cera_arch $arch" \
    --define "cera_stage $stage"
cp "$rpm_top"/RPMS/*/*.rpm "$dist/"

ls -la "$dist"
