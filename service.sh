#! /bin/sh

if [ $# -lt 1 ]; then
	printf 'Usage: %s CMD [OPTS].\n' "${0##*/}" 1>&2
	exit 1
fi

WRAPPER_PATH=crates/core/src/mupdf_wrapper
TARGET_OS=$(uname -s)

if ! [ -e "${WRAPPER_PATH}/${TARGET_OS}" ]; then
	cd "$WRAPPER_PATH" || exit 1
	./build.sh
	cd ../..
fi

CMD=$1
shift

case "$CMD" in
	run_emulator)
		cargo run --bin plato-emulator --features emulator "$@"
		;;
	install_importer)
		cargo install --path . --bin plato-import --features importer "$@"
		;;
	*)
		printf 'Unknown command: %s.\n' "$CMD" 1>&2
		exit 1
		;;
esac
