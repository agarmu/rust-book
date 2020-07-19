#!/bin/bash

homedir=$(pwd)
for d in ./*/*
do
    echo $d
    cd $d
    cargo fmt
    cd $homedir
done
echo "Completed Code format."
