@echo off
REM Build script for Developer Command Prompt
REM Sets environment variables and builds the project

echo Setting up build environment...
set CMAKE_GENERATOR=Visual Studio 17 2022
set AWS_LC_SYS_C_STD=c11
set CFLAGS=/std:c11
set CFLAGS_x86_64_pc_windows_msvc=/std:c11
REM Try to use prebuilt binaries to avoid compilation
set AWS_LC_SYS_USE_PKG_CONFIG=1

echo Building project...
cargo build

if %ERRORLEVEL% EQU 0 (
    echo.
    echo Build successful!
) else (
    echo.
    echo Build failed with error code %ERRORLEVEL%
)

