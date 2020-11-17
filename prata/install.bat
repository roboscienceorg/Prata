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
mkdir .\Build\Julia\prata.jl\src > NUL

maturin build --release

copy .\target\release\prata.dll .\Build\prata.dll > NUL
copy .\Build\prata.dll .\Build\prata.pyd > NUL
copy .\target\wheels\* .\Build\wheels\ > NUL
copy .\prata\src\* .\Build\Julia\prata.jl\src\* > NUL
copy .\target\release\prata.dll .\Build\Julia\prata.jl\src\prata.dll > NUL
copy .\prata\Project.toml .\Build\Julia\prata.jl > NUL

pip uninstall -y prata > NUL

echo [1;34m Please install prata by installing the wheel file in Build/Wheels[0m
