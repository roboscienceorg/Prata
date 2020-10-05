@echo off
python --version >nul 2>&1 || (
    echo Python must be installed, please install it
    pause
    exit
)

cmake --version >nul 2>&1 || (
    echo Cmake must be installed. please install it
    pause
    exit
)

cargo --version >nul 2>&1 || (
    echo cargo must be installed, install by going to "https://www.rust-lang.org/tools/install"
    pause
    exit
)

maturin --version >nul 2>&1 || (
    echo Maturin Must be installed, install by "pip install maturin"
    pause
    exit
)


mkdir  Build
mkdir .\Build\Wheels
mkdir .\Build\Julia\TALA.jl\src

cargo build
maturin build

copy .\target\debug\TALA.dll .\Build\TALA.dll
copy .\Build\TALA.dll .\Build\TALA.pyd
copy .\target\wheels\* .\Build\wheels\


pause
