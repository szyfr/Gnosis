@echo off

cls

for /f "tokens=2 delims==" %%a in ('wmic OS Get localdatetime /value') do set "dt=%%a"
set "date=%dt:~0,4%_%dt:~4,2%_%dt:~6,2%"

set "name=gnosis"

rem Figure out how to get it to compile directly into directory instead of target\debug\%date%\debug\...
rem cargo run --target-dir G:\%name%\target\debug\%date%\
cargo build --target-dir G:\%name%\target\debug\%date%\

xcopy "data\"    "target\debug\%date%\debug\data\" /v /q /s /e /y > nul
xcopy "src\"     "target\debug\%date%\debug\src\"  /v /q /s /e /y > nul