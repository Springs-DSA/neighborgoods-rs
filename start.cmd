@echo off
setlocal ENABLEEXTENSIONS ENABLEDELAYEDEXPANSION

:: Default all variables to empty
set NODE_NAME=
set NODE_DESCRIPTION=
set NODE_LAT=
set NODE_LNG=
set NODE_ID=

:: Check if .env file exists
set OVERWRITE_ENV=n
if exist .env (
    echo A .env file already exists.
    set /p OVERWRITE_ENV="Do you want to overwrite it? (y/n) [default: n]: "
    if /i "!OVERWRITE_ENV!"=="" set OVERWRITE_ENV=n
)

:: If not overwriting, load existing values
if /i "!OVERWRITE_ENV!"=="n" (
    echo Reusing values from existing .env file...
    for /f "usebackq tokens=1,* delims==" %%A in (".env") do (
        set "key=%%A"
        set "value=%%B"
        if /i "!key!"=="POSTGRES_PASSWORD" set POSTGRES_PASSWORD=!value!
        if /i "!key!"=="ROCKET_SECRET_KEY" set ROCKET_SECRET_KEY=!value!
        if /i "!key!"=="NODE_ID" set NODE_ID=!value!
        if /i "!key!"=="NODE_NAME" set NODE_NAME=!value!
        if /i "!key!"=="NODE_DESCRIPTION" set NODE_DESCRIPTION=!value!
        if /i "!key!"=="NODE_LAT" set NODE_LAT=!value!
        if /i "!key!"=="NODE_LNG" set NODE_LNG=!value!
    )
)

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

:: Generate POSTGRES_PASSWORD only if not set
if not defined POSTGRES_PASSWORD (
    set POSTGRES_PASSWORD=
    for /L %%i in (1,1,%length%) do (
        set /A "index=!random! %% 62"
        call set "char=%%charset:~!index!,1%%"
        set "POSTGRES_PASSWORD=!POSTGRES_PASSWORD!!char!"
    )
)

:: Generate ROCKET_SECRET_KEY only if not set
if not defined ROCKET_SECRET_KEY (
    for /f %%A in ('powershell -NoProfile -Command "[Convert]::ToBase64String((1..32 | ForEach-Object {Get-Random -Minimum 0 -Maximum 256}) -as [byte[]])"') do (
        set ROCKET_SECRET_KEY=%%A
    )
)

:: Generate NODE_ID only if not set
if not defined NODE_ID (
    set NODE_ID=
    for /L %%i in (1,1,%length%) do (
        set /A "index=!random! %% 62"
        call set "char=%%charset:~!index!,1%%"
        set "NODE_ID=!NODE_ID!!char!"
    )
)

:: Write to .env
if /i "!OVERWRITE_ENV!"=="y" (
    echo Writing new values to .env file...
    (
        echo POSTGRES_PASSWORD=%POSTGRES_PASSWORD%
        echo NODE_ID=%NODE_ID%
        echo NODE_NAME="%NODE_NAME%"
        echo NODE_DESCRIPTION="%NODE_DESCRIPTION%"
        echo NODE_LAT=%NODE_LAT%
        echo NODE_LNG=%NODE_LNG%
        echo DATABASE_URL=postgres://postgres:%POSTGRES_PASSWORD%@localhost:5444/neighborgoods
        echo DOCKER_DATABASE_URL=postgres://postgres:%POSTGRES_PASSWORD%@db:5432/neighborgoods
        echo ROCKET_ADDRESS=0.0.0.0
        echo ROCKET_PORT=8000
        echo ROCKET_SECRET_KEY=%ROCKET_SECRET_KEY%
    ) > .env
) else (
    echo Skipping .env overwrite. Using existing values.
)

:: Run docker-compose
::docker compose build --no-cache
docker compose up --build -d
