#!/bin/bash

build() {
    echo Building $1
    pushd $1 > /dev/null
    cargo clean
    cargo build --release
    popd > /dev/null
}

run() {
    BIN=`basename $1`
    if [ ! -f $1/target/release/$BIN ]; then
        build $1
    fi

    echo Running $1
    pushd $1 > /dev/null
    time /bin/bash -c "./target/release/$BIN > /dev/null"
    popd > /dev/null
}

BUILD=0
RUN=1

days=( $(find . -type d -name "day-??-part-?" | sort) )
if [ $BUILD == 1 ]; then
    for day in "${days[@]}"; do
        build $day
    done
fi

if [ $RUN == 1 ]; then
    for day in "${days[@]}"; do
        run $day
    done
fi
