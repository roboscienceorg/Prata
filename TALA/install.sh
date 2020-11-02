#!/bin/bash

maturin=maturin

if ! command -v $maturin &> /dev/null
then
	echo "Maturin not found, Checking for local Maturin"
	maturin=./maturin
	if ! command -v .$maturin &> /dev/null
	then
		echo "Maturin not installed"
		exit
	else
		echo "Using local Maturin"
	fi
fi

if ! python3 --version 2>&1 >/dev/null; then
    echo "Python3 needs to be installed"
fi

if ! cmake --version 2>&1 >/dev/null; then
    echo "Cmake needs to be installed"
fi

if ! cargo --version 2>&1 >/dev/null; then
    echo "Python3 needs to be installed"
fi

mkdir ./Build/ &>/dev/null
mkdir ./Build/Julia/ &>/dev/null
mkdir ./Build/Julia/TALA.jl/ &>/dev/null
mkdir ./Build/Julia/TALA.jl/src/ &>/dev/null
mkdir ./Build/Wheels/ &>/dev/null



$maturin build
pip uninstall -y TALA
pip install ./Build/Wheels/*.whl

cp ./target/debug/libTALA.so ./Build/TALA.so
cp ./Build/TALA.so ./Build/TALA.pyd
cp ./target/wheels/* ./Build/Wheels/
