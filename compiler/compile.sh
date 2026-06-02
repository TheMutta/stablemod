#!/bin/sh

set -ex

# Compiler Configuration
CROSS_TARGET="${1:-x86_64-elf}"
GCC_VERSION="13.2.0" # GCC version
BIN_VERSION="2.42"  # Binutils version
MPF_VERSION="4.2.1" # MPFR version
GMP_VERSION="6.3.0" # GMP version
MPC_VERSION="1.3.1" # MPC version

# Build Configuration
MAKE_JOBS=14
INSTALL_PATH="$PWD/target/$CROSS_TARGET"
DOWNLOAD_PATH="$PWD/download"

GCC_PKG_NAME="gcc-$GCC_VERSION"
BIN_PKG_NAME="binutils-$BIN_VERSION"
MPF_PKG_NAME="mpfr-$MPF_VERSION"
GMP_PKG_NAME="gmp-$GMP_VERSION"
MPC_PKG_NAME="mpc-$MPC_VERSION"

GCC_SRC="https://ftp.gnu.org/gnu/gcc/$GCC_PKG_NAME/$GCC_PKG_NAME.tar.gz" # GCC download link
BIN_SRC="https://ftp.gnu.org/gnu/binutils/$BIN_PKG_NAME.tar.gz"          # Binutils download link
MPF_SRC="https://ftp.gnu.org/gnu/mpfr/$MPF_PKG_NAME.tar.xz"          # MPFR download link
GMP_SRC="https://ftp.gnu.org/gnu/gmp/$GMP_PKG_NAME.tar.xz"          # GMP download link
MPC_SRC="https://ftp.gnu.org/gnu/mpc/$MPC_PKG_NAME.tar.gz"          # MPC download link

mkdir -p $INSTALL_PATH

mkdir -p $DOWNLOAD_PATH
cd $DOWNLOAD_PATH

# download sources
echo "--> [STATUS] downloading sources..."
if [ ! -f $GCC_PKG_NAME.tar.gz ]; then
	wget $GCC_SRC
fi
if [ ! -f $BIN_PKG_NAME.tar.gz ]; then
	wget $BIN_SRC
fi
if [ ! -f $MPF_PKG_NAME.tar.xz ]; then
	wget $MPF_SRC
fi
if [ ! -f $GMP_PKG_NAME.tar.xz ]; then
	wget $GMP_SRC
fi
if [ ! -f $MPC_PKG_NAME.tar.gz ]; then
	wget $MPC_SRC
fi

cd ..

# create directory
mkdir -pv build/$CROSS_TARGET
cd build/$CROSS_TARGET

# unpack source archives
echo "--> [STATUS] unpacking archives..."
if [ ! -d $GCC_PKG_NAME ]; then
	tar -xpvf $DOWNLOAD_PATH/$GCC_PKG_NAME.tar.gz
fi
if [ ! -d $BIN_PKG_NAME ]; then
	tar -xpvf $DOWNLOAD_PATH/$BIN_PKG_NAME.tar.gz
fi


# build binutils
cd $BIN_PKG_NAME
mkdir -pv build
cd build
../configure                      \
	--target=$CROSS_TARGET    \
	--prefix="$INSTALL_PATH"  \
	--with-sysroot            \
	--disable-nls             \
	--disable-werror
make -j$MAKE_JOBS
make install

cd ../..

# build gcc
cd $GCC_PKG_NAME
tar -xvpf $DOWNLOAD_PATH/$MPF_PKG_NAME.tar.xz
tar -xvpf $DOWNLOAD_PATH/$GMP_PKG_NAME.tar.xz
tar -xpvf $DOWNLOAD_PATH/$MPC_PKG_NAME.tar.gz
mv -v $MPF_PKG_NAME mpfr
mv -v $GMP_PKG_NAME gmp
mv -v $MPC_PKG_NAME mpc
mkdir -pv build
cd build
../configure                      \
	--target=$CROSS_TARGET    \
	--prefix="$INSTALL_PATH"  \
	--with-glibc-version=2.36 \
	--with-newlib             \
	--without-headers         \
	--enable-initfini-array   \
	--disable-nls             \
	--disable-shared          \
	--disable-multilib        \
	--disable-decimal-float   \
	--disable-threads         \
	--disable-libatomic       \
	--disable-libgomp         \
	--disable-libquadmath     \
	--disable-libssp          \
	--disable-libvtv          \
	--disable-libstdcxx       \
	--enable-languages=c,c++
make all-gcc -j$MAKE_JOBS
make all-target-libgcc -j$MAKE_JOBS
make install-gcc
make install-target-libgcc

cd ../../..

echo "--> [STATUS] DONE!"

