@echo off

rem Run cargo build
cargo build

rem Check if cargo build was successful
if %errorlevel% equ 0 (
    rem Run the binary file with src and dest parameters
    .\target\debug\cc-unzip-media.exe %1 %2
) else (
    echo Cargo build failed. Exiting...
    exit /b 1
)