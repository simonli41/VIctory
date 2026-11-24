@echo off
setlocal
pushd "%~dp0" || exit /b 1
set "CARGO_TARGET_DIR=%CD%\target"
set "BUILD_TARGET=x86_64-pc-windows-msvc"

where cargo >nul 2>nul
if errorlevel 1 (
    echo Install Rust with rustup, then reopen this file.
    echo Windows builds also need the Visual C++ x64 build tools and Windows SDK.
    goto failed
)

cargo test --locked --target %BUILD_TARGET% -p victory-gpu --lib descriptor::slots::tests
if errorlevel 1 goto failed
cargo build --locked --release --target %BUILD_TARGET% -p victory
if errorlevel 1 goto failed
set "VICTORY_EXE=%CARGO_TARGET_DIR%\%BUILD_TARGET%\release\victory.exe"
"%VICTORY_EXE%" --help >nul
if errorlevel 1 goto failed

echo Built and checked "%VICTORY_EXE%"
popd
pause
exit /b 0

:failed
echo Build failed. See the messages above.
popd
pause
exit /b 1
