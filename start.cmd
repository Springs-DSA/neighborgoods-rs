@echo off
setlocal ENABLEEXTENSIONS ENABLEDELAYEDEXPANSION

:: Default all variables to empty
set NODE_NAME=
set NODE_DESCRIPTION=
set NODE_LAT=
set NODE_LNG=
set NODE_ID=
set POSTGRES_PASSWORD=
set ROCKET_SECRET_KEY=
set DATABASE_URL=
set DOCKER_DATABASE_URL=
set ROCKET_ADDRESS=
set ROCKET_PORT=

:: Check if .env file exists
set OVERWRITE_ENV=n
if exist .env (
    echo A .env file already exists.
    set /p OVERWRITE_ENV="Do you want to overwrite it? (y/n) [default: n]: "
    if /i "!OVERWRITE_ENV!"=="" set OVERWRITE_ENV=n
) else (
    set OVERWRITE_ENV=y
)

:: If not overwriting, load existing values
if /i "!OVERWRITE_ENV!"=="n" (
    echo Reusing values from existing .env file...
    for /f "usebackq tokens=1,* delims==" %%A in (".env") do (
        set "key=%%A"
        set "value=%%B"
        if /i "!key!"=="POSTGRES_PASSWORD" set POSTGRES_PASSWORD=!value!
        if /i "!key!"=="ROCKET_SECRET_KEY" set ROCKET_SECRET_KEY=!value!
        if /i "!key!"=="DATABASE_URL" set DATABASE_URL=!value!
        if /i "!key!"=="DOCKER_DATABASE_URL" set DOCKER_DATABASE_URL=!value!
        if /i "!key!"=="ROCKET_ADDRESS" set ROCKET_ADDRESS=!value!
        if /i "!key!"=="ROCKET_PORT" set ROCKET_PORT=!value!
        if /i "!key!"=="NODE_ID" set NODE_ID=!value!
        if /i "!key!"=="NODE_NAME" set NODE_NAME="!value!"
        if /i "!key!"=="NODE_DESCRIPTION" set NODE_DESCRIPTION="!value!"
        if /i "!key!"=="NODE_LAT" set NODE_LAT=!value!
        if /i "!key!"=="NODE_LNG" set NODE_LNG=!value!
    )

    if not defined POSTGRES_PASSWORD (
        if defined DATABASE_URL (
            rem Strip the protocol prefix
            set "TEMP_URL=!DATABASE_URL:postgres://=!"

            rem TEMP_URL now is: postgres:password@localhost:5444/neighborgoods

            rem Strip user (postgres:)
            for /f "tokens=2 delims=:" %%A in ("!TEMP_URL!") do set "AFTER_USER=%%A"

            rem AFTER_USER now is: password@localhost:5444/neighborgoods

            rem Extract password (split at @)
            for /f "tokens=1 delims=@" %%B in ("!AFTER_USER!") do set "POSTGRES_PASSWORD=%%B"

            echo WARNING: POSTGRES_PASSWORD is not set, but found DATABASE_URL. Setting POSTGRES_PASSWORD to value derived from DATABASE_URL
            echo( >> .env
            echo POSTGRES_PASSWORD=!POSTGRES_PASSWORD! >> .env
        ) else (
            echo WARNING: POSTGRES_PASSWORD is not set nor is DATABASE_URL. This will be generated and added to .env
        )
    )

    if not defined ROCKET_SECRET_KEY (
        echo WARNING: ROCKET_SECRET_KEY is not set. This will be generated and added to .env
    )

    if not defined NODE_ID (
        echo WARNING: NODE_ID is not set. This will be generated and added to .env
    )

    if not defined DATABASE_URL (
        echo WARNING: DATABASE_URL is not set. This will be generated and added to .env
    )

    if not defined DOCKER_DATABASE_URL (
        echo WARNING: DOCKER_DATABASE_URL is not set. This will be generated and added to .env
    )

    if not defined ROCKET_ADDRESS (
        echo WARNING: ROCKET_ADDRESS is not set. Adding default 0.0.0.0 to .env.
        set ROCKET_ADDRESS="0.0.0.0"
        echo( >> .env
        echo ROCKET_ADDRESS=!ROCKET_ADDRESS! >> .env
    )

    if not defined ROCKET_PORT (
        echo WARNING: ROCKET_PORT is not set. Adding default 8000 to .env.
        set ROCKET_PORT="8000"
        echo( >> .env
        echo ROCKET_PORT=!ROCKET_PORT! >> .env
    )

    :: Optional values
    if not defined NODE_LAT (
        echo OPTIONAL: NODE_LAT is not set.
    )

    if not defined NODE_LNG (
        echo OPTIONAL: NODE_LNG is not set.
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
if not defined NODE_NAME (
    if "%NODE_NAME%"=="" (
        set /p NODE_NAME=Enter NODE_NAME:
    )
)

if not defined NODE_DESCRIPTION (
    if "%NODE_DESCRIPTION%"=="" (
        set /p NODE_DESCRIPTION=Enter NODE_DESCRIPTION:
    )
)

:: Setup unique value generation
set charset=ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789
set length=32

:: Generate POSTGRES_PASSWORD only if not set
if not defined POSTGRES_PASSWORD (
    echo Generating Postgress password...
    for /L %%i in (1,1,%length%) do (
        set /A "index=!random! %% 62"
        call set "char=%%charset:~!index!,1%%"
        set "POSTGRES_PASSWORD=!POSTGRES_PASSWORD!!char!"
    )

    if /i "!OVERWRITE_ENV!"=="n" (
        echo Adding missing Postgres password to .env...
        echo( >> .env
        echo POSTGRES_PASSWORD=!POSTGRES_PASSWORD! >> .env
    )
    echo Done!
)

:: Generate ROCKET_SECRET_KEY only if not set
if not defined ROCKET_SECRET_KEY (
    echo Generating Rocket secret key...
    for /f %%A in ('powershell -NoProfile -Command "[Convert]::ToBase64String((1..32 | ForEach-Object {Get-Random -Minimum 0 -Maximum 256}) -as [byte[]])"') do (
        set ROCKET_SECRET_KEY=%%A
    )
    if /i "!OVERWRITE_ENV!"=="n" (
        echo Adding missing Rocket secret key to .env...
        echo( >> .env
        echo ROCKET_SECRET_KEY=%ROCKET_SECRET_KEY% >> .env
    )
    echo Done!
)

:: Generate NODE_ID only if not set
if not defined NODE_ID (
    echo Generating node ID...
    for /L %%i in (1,1,%length%) do (
        set /A "index=!random! %% 62"
        call set "char=%%charset:~!index!,1%%"
        set "NODE_ID=!NODE_ID!!char!"
    )
    if /i "!OVERWRITE_ENV!"=="n" (
        echo Adding missing node ID to .env...
        echo( >> .env
        echo NODE_ID=!NODE_ID! >> .env
    )
    echo Done!
)

:: Generate database URL only if not set
if not defined DATABASE_URL (
    echo Defining database URL...
    set DATABASE_URL=postgres://postgres:!POSTGRES_PASSWORD!@localhost:5444/neighborgoods
    if /i "!OVERWRITE_ENV!"=="n" (
        echo Adding missing database URL to .env...
        echo( >> .env
        echo DATABASE_URL=!DATABASE_URL! >> .env
    )
    echo Done!
)

:: Generate docker database URL only if not set
if not defined DOCKER_DATABASE_URL (
    echo Defining docker database URL...
    set DOCKER_DATABASE_URL=postgres://postgres:!POSTGRES_PASSWORD!@db:5432/neighborgoods
    if /i "!OVERWRITE_ENV!"=="n" (
        echo Adding missing docker database URL to .env...
        echo( >> .env
        echo DOCKER_DATABASE_URL=!DOCKER_DATABASE_URL! >> .env
    )
    echo Done!
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
)

:: Run docker-compose
::docker compose build --no-cache
docker compose up --build -d
