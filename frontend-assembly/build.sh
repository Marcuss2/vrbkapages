#/bin/sh

DIR="$( cd "$( dirname "$0" )" && pwd )"

(cd $DIR; trunk build $@)

rm -rf $DIR/../dist-assembly/*
mv $DIR/dist/* $DIR/../dist-assembly/
