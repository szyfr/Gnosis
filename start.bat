@echo off

cls

for /f "tokens=2 delims==" %%a in ('wmic OS Get localdatetime /value') do set "dt=%%a"
set "date=%dt:~0,4%_%dt:~4,2%_%dt:~6,2%"

set "name=gnosis"

call ./build.bat

Start remedybg target/debug/%date%/debug/%name%.exe

code ./