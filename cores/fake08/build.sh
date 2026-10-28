#!/bin/sh
set -eu

here="$(cd "$(dirname "$0")" && pwd)"

device_cflags="-mcpu=cortex-a53"

usage() {
	echo "usage: $0 stamp COMMIT | build COMMIT WORKDIR OUT" >&2
	exit 2
}

sha256() {
	if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1"; else shasum -a 256 "$1"; fi |
		cut -d' ' -f1
}

stamp() {
	echo "commit=$1"
	echo "source=https://github.com/jtothebell/fake-08/tree/$1"
	echo "recipe=make -C platform/libretro platform=unix"
	echo "device_cflags=$device_cflags"
	echo "submodule=libs/z8lua https://github.com/jtothebell/z8lua"
	for p in "$here"/*.patch; do
		[ -e "$p" ] || continue
		echo "patch=$(basename "$p") sha256:$(sha256 "$p")"
	done
}

build() {
	commit="$1" work="$2" out="$3"
	src="$work/fake-08"

	rm -rf "$work"
	mkdir -p "$src"
	git -C "$src" init -q
	git -C "$src" remote add origin https://github.com/jtothebell/fake-08
	git -C "$src" fetch -q --depth 1 origin "$commit"
	git -C "$src" checkout -q --force --detach "$commit"
	git -C "$src" submodule update -q --init --depth 1 libs/z8lua
	for p in "$here"/*.patch; do
		[ -e "$p" ] || continue
		git -C "$src" apply -p1 "$p"
	done

	cflags=""
	if [ "$(uname -s)-$(uname -m)" = "Linux-aarch64" ]; then
		cflags="$device_cflags"
	fi

	CFLAGS="$cflags" CXXFLAGS="$cflags" make -C "$src/platform/libretro" platform=unix \
		-j"$(getconf _NPROCESSORS_ONLN)"

	mkdir -p "$(dirname "$out")"
	cp "$src/platform/libretro/fake08_libretro.so" "$out"
	stamp "$commit" >"$out.meta"
}

case "${1:-}" in
stamp)
	[ $# -eq 2 ] && [ -n "$2" ] || usage
	stamp "$2"
	;;
build)
	[ $# -eq 4 ] && [ -n "$2" ] && [ -n "$3" ] && [ -n "$4" ] || usage
	build "$2" "$3" "$4"
	;;
*)
	usage
	;;
esac
