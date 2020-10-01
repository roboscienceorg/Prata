@echo off
julia --version >nul 2>&1 || (
    echo Install Julia Idiot
    pause
    exit
)


FOR /F "tokens=* USEBACKQ" %%F IN (`dir /b /s Build\Wheels`) DO (
SET t=%%F
)

julia .\installJulia.jl %t%

pause
