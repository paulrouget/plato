#! /bin/sh

set -e

CC=${CC:-gcc}
AR=${AR:-ar}

TARGET_OS=${TARGET_OS:-$(uname -s)}
mkdir -p ${TARGET_OS}
$CC $CPPFLAGS $CFLAGS -c mupdf_wrapper.c -o ${TARGET_OS}/mupdf_wrapper.o
$AR -rcs ${TARGET_OS}/libmupdf_wrapper.a ${TARGET_OS}/mupdf_wrapper.o
