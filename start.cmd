@echo off
setlocal ENABLEEXTENSIONS ENABLEDELAYEDEXPANSION

:: Default all variables to empty
set NODE_NAME=
set NODE_DESCRIPTION=
set NODE_LAT=
set NODE_LNG=

:: Parse command-line arguments
:parse_args
if "%~1"=="" goto after_args
if "%~1"=="--node-name" (
    set NODE_NAME=%~2
    shift
) else if "%~1"=="--node-description" (
    set NODE_DESCRIPTION=%~2
    shift
) else if "%~1"=="--node-lat" (
    set NODE_LAT=%~2
    shift
) else if "%~1"=="--node-lng" (
    set NODE_LNG=%~2
    shift
) else (
    echo Unknown argument: %~1
    exit /b 1
)
shift
goto parse_args

:after_args

:: Prompt for required values
if "%NODE_NAME%"=="" (
    set /p NODE_NAME=Enter NODE_NAME:
)
if "%NODE_DESCRIPTION%"=="" (
    set /p NODE_DESCRIPTION=Enter NODE_DESCRIPTION:
)

:: Generate a random password using PowerShell
FOR /F "usebackq delims=" %%p IN (`powershell -Command "[Convert]::ToBase64String((1..32 | ForEach-Object {Get-Random -Minimum 0 -Maximum 256} | ForEach-Object {[byte]$_}))"` ) DO set POSTGRES_PASSWORD=%%p
FOR /F "usebackq delims=" %%p IN (`powershell -Command "[Convert]::ToBase64String((1..32 | ForEach-Object {Get-Random -Minimum 0 -Maximum 256} | ForEach-Object {[byte]$_}))"` ) DO set NODE_ID=%%p
FOR /F "usebackq delims=" %%p IN (`powershell -Command "[Convert]::ToBase64String((1..32 | ForEach-Object {Get-Random -Minimum 0 -Maximum 256} | ForEach-Object {[byte]$_}))"` ) DO set ROCKET_SECRET_KEY=%%p

:: Build DATABASE_URL
set DATABASE_URL=postgres://user:%POSTGRES_PASSWORD%@db:5432/mydb
set ROCKET_SECRET_KEY=%ROCKET_SECRET_KEY%

:: Write to .env
(
    echo POSTGRES_PASSWORD=%POSTGRES_PASSWORD%
    echo NODE_ID=%NODE_ID%
    echo NODE_NAME=%NODE_NAME%
    echo NODE_DESCRIPTION=%NODE_DESCRIPTION%
    echo NODE_LAT=%NODE_LAT%
    echo NODE_LNG=%NODE_LNG%
    echo DATABASE_URL=%DATABASE_URL%
    echo ROCKET_ADDRESS=0.0.0.0
    echo ROCKET_PORT=8000
    echo ROCKET_SECRET_KEY=%ROCKET_SECRET_KEY%
) > .env

:: Run docker-compose
docker-compose up -d
