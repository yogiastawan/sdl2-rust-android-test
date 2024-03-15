#!/bin/sh
NAME_LIB='sdl_rust_test'
COLOR='\033[1;32m'

ANDROID_PROJECT=/home/gihexdev00/AndroidStudioProjects/SDLTest
SDL_LIB=/home/gihexdev00/Game-Dev/lib/SDL-android

JNI_LIBS=${ANDROID_PROJECT}/app/src/main/jniLibs


echo -e "${COLOR}Cleaning output directory..."
rm -rf $JNI_LIBS
echo -e "${COLOR}Creating output directory..."
mkdir $JNI_LIBS
mkdir $JNI_LIBS/arm64-v8a
mkdir $JNI_LIBS/armeabi-v7a
mkdir $JNI_LIBS/x86
mkdir $JNI_LIBS/x86_64

if [ $# -lt 1 ]; then
    build='debug'
else
    build=release

fi

echo -e "${COLOR}Copy SDl Lib to target ${build}..."
cp ${SDL_LIB}/${build}/arm64-v8a/libSDL2.so ./target/aarch64-linux-android/${build}/deps/libSDL2.so
cp ${SDL_LIB}/${build}/armeabi-v7a/libSDL2.so ./target/armv7-linux-androideabi/${build}/deps/libSDL2.so
cp ${SDL_LIB}/${build}/x86/libSDL2.so ./target/i686-linux-android/${build}/deps/libSDL2.so
cp ${SDL_LIB}/${build}/x86_64/libSDL2.so ./target/x86_64-linux-android/${build}/deps/libSDL2.so


echo -e "${COLOR}Build ${build}..."
cargo build --target aarch64-linux-android --$1
cargo build --target armv7-linux-androideabi --$1
cargo build --target i686-linux-android --$1
cargo build --target x86_64-linux-android --$1

echo -e "${COLOR}Add to ${JNI_LIBS}"
cp target/aarch64-linux-android/${build}/lib${NAME_LIB}.so $JNI_LIBS/arm64-v8a/lib${NAME_LIB}.so
cp target/armv7-linux-androideabi/${build}/lib${NAME_LIB}.so $JNI_LIBS/armeabi-v7a/lib${NAME_LIB}.so
cp target/i686-linux-android/${build}/lib${NAME_LIB}.so $JNI_LIBS/x86/lib${NAME_LIB}.so
cp target/x86_64-linux-android/${build}/lib${NAME_LIB}.so $JNI_LIBS/x86_64/lib${NAME_LIB}.so
