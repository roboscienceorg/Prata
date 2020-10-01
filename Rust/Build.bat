@echo off
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
mkdir .\Build\Julia

cargo build --release
maturin build

copy .\target\release\TALA.dll .\Build\TALA.dll
copy .\Build\TALA.dll .\Build\TALA.pyd
copy .\target\wheels\* .\Build\wheels\
xcopy .\TALA\TALA.jl\* .\Build\Julia\TALA.jl\ /E


pause
