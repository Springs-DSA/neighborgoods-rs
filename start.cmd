@echo off
setlocal ENABLEEXTENSIONS ENABLEDELAYEDEXPANSION

:: Default all variables to empty
set NODE_NAME=
set NODE_DESCRIPTION=
set NODE_LAT=
set NODE_LNG=
set NODE_ID=

:: Parse command-line arguments
:parse_args
if "%~1"=="" goto after_args
if "%~1"=="--node-name" (
    set NODE_NAME=%~2
    shift
) else if "%~1"=="--node-description" (
    set NODE_DESCRIPTION=%~2
    shift
) else if "%~1"=="--node-id" (
    set NODE_ID=%~2
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

:: Setup unique value generation
set charset=ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789
set length=32
set POSTGRES_PASSWORD=
set ROCKET_SECRET_KEY=

:: To reuse existing node ID, or generate one
if "%NODE_ID%"=="" (
    set NODE_ID=

    :: Generate and set NODE_ID
    for /L %%i in (1,1,%length%) do (
        set /A "index=!random! %% 62"
        call set "char=%%charset:~!index!,1%%"
        set "NODE_ID=!NODE_ID!!char!"
    )
)

:: Generate and set POSTGRES_PASSWORD
for /L %%i in (1,1,%length%) do (
    set /A "index=!random! %% 62"
    call set "char=%%charset:~!index!,1%%"
    set "POSTGRES_PASSWORD=!POSTGRES_PASSWORD!!char!"
)

:: Generate and set ROCKET_SECRET_KEY
for /L %%i in (1,1,%length%) do (
    set /A "index=!random! %% 62"
    call set "char=%%charset:~!index!,1%%"
    set "ROCKET_SECRET_KEY=!ROCKET_SECRET_KEY!!char!"
)

:: Build DATABASE_URL
set DATABASE_URL=postgres://postgres:%POSTGRES_PASSWORD%@db:5432/mydb
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
docker-compose build --no-cache
docker-compose up -d
