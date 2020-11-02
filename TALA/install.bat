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


mkdir  Build > NUL
mkdir .\Build\Wheels > NUL
mkdir .\Build\Julia\TALA.jl\src > NUL

maturin build

copy .\target\debug\TALA.dll .\Build\TALA.dll > NUL
copy .\Build\TALA.dll .\Build\TALA.pyd > NUL
copy .\target\wheels\* .\Build\wheels\ > NUL

pip uninstall -y TALA > NUL

echo [1;34m Please install TALA by installing the wheel file in Build/Wheels[0m
