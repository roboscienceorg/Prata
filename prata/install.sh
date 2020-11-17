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
    echo "cargo needs to be installed"
fi

mkdir ./Build/ &>/dev/null
mkdir ./Build/Julia/prata.jl/src/ &>/dev/null
mkdir ./Build/Wheels/ &>/dev/null

$maturin build --release

pip uninstall -y prata

cp ./target/release/libprata.so ./Build/prata.so
cp ./Build/prata.so ./Build/prata.pyd
cp ./target/wheels/* ./Build/Wheels/
cp .\prata\src\* .\Build\Julia\prata.jl\src\*
cp .\target\release\prata.so .\Build\Julia\prata.jl\src\prata.so
cp .\prata\Project.toml .\Build\Julia\prata.jl
