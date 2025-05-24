# Developer ReadMe
Thank you for your interest in contributing to the NeighborGoods project! This document contains instructions on how to set up your development environment, select issues, and get them merged into the main branch.

## Tech Stack
NeighborGoods is built using the following technologies:
- Rust: serves as the main programming language for the project.
- [Rocket.rs](rocket.rs): is a web framework built for rust, and shapes everything about how the app functions.
- [Handlebars](https://handlebarsjs.com/): is a templating engine that we use to display pages to users, via server-side rendering.
- Postgres: serves as our database.
- [Diesel](https://diesel.rs/): this is our ORM solution, and also handles migrations through a cli.
- Docker: responsible for running the app, and used in development

## Getting a local development version working
### Install Prerequisites
To run the app, you theoretically only need to install docker desktop, clone the repository, fill out a `.env` file,  and run `docker compose up --build -d`. However, for development purposes it is much easier to install several items on your local machine. With that in mind, the following items are recommended for development:
- rust
  - rust-analyzer
- diesel_cli
  - postgres
- docker desktop

#### 1. Install Rust
Rust installation is the most straightforward step. go to [their install page](https://www.rust-lang.org/tools/install) and follow the instructions. Once rust is installed, you'll probably also want to install the rust-analyzer extension for VSCode if that's what you're using to develop with, or the equivalent syntax highlighting/language server for the IDE you're using.

#### 2. Install Diesel CLI
The diesel cli is used to run generate and run migrations for the database. These are the full instructions for [installing the cli](https://github.com/diesel-rs/diesel/blob/master/diesel_cli/README.md), but the basics are listed here as well.

For our purposes, Diesel CLI requires `openssl` and `libpq` to be installed on the host machine. Most machines come with openssl, but libpq is not standard. When developing on Mac/unix, installing postgres through a package manager like brew should be sufficient: `brew install postgresql` or `brew install libpq`. Make sure that the installed packages are available in your PATH before attempting to install the diesel cli:
```
export PATH="/opt/homebrew/opt/libpq/bin:$PATH" && export LIBRARY_PATH=$LIBRARY_PATH:/opt/homebrew/opt/libpq/lib
```
Installing for windows is a bit more involved. Download and install [Postgres](https://www.postgresql.org/download/windows/), then you will need to set the environment variables for your installation.

Typically, PostgreSQL 17 would be installed in:

`C:\Program Files\PostgreSQL\17`

You need to set these environment variables:

1. `PQ_LIB_DIR` - Points to the lib directory
2. `LIBPQ_DIR` - Points to PostgreSQL installation

To configure your environment variables, click the Start button, then type "environment properties" into the search bar and hit Enter. In the System Properties window, click "Environment Variables."

Under "System variables" section, click "New"
1. Add variable:
   - Variable name: `PQ_LIB_DIR`
   - Variable value: `C:\Program Files\PostgreSQL\17\lib`
2. Repeat to add:
   - Variable name: `LIBPQ_DIR`
   - Variable value: `C:\Program Files\PostgreSQL\17`
3. Click OK to save

While in the Environment Variables dialog:

1. Find the "Path" variable in System variables
2. Click "Edit"
3. Click "New"
4. Add: `C:\Program Files\PostgreSQL\17\bin`
5. Click OK to save

-------

Once this is complete, you can install the diesel cli (for all operating systems) by running the following in terminal:
```
cargo install diesel_cli --no-default-features --features "postgres"
```

#### 3. Install Docker Desktop
[Docker Desktop](https://www.docker.com/products/docker-desktop/) can be installed for your system using their pre-built installers. Once docker desktop has been installed, all of the pre-reqs have been met!

### Cold Start
Starting the application for the first time requires following these steps:

1. clone the NeighborGoods repo: `git clone git@github.com:Springs-DSA/neighborgoods-rs.git`
2. Run start.cmd for Windows, or start.sh for Linux
   ### Linux Example
    ```
    ./start.sh --node-name "My Node Name" --node-description "This is my test node"
    ```
    
    ### Windows Example
    ```
    .\start.cmd --node-name "My Node Name" --node-description "This is my test node"
    ```
3. once everything has built successfully, you will need to run the migrations. run `diesel migration run` in the project root.
4. after compiling for a moment, your development instance should be available at `localhost:8000`!

## Developing NeighborGoods
When developing, the docker containers are set to rebuild upon changes being detected in the source code. Rust is a compiled language, so the first build will take a while, but after that rebuilds should be relatively quick. Changes you make to the code should be reflected in the running app in a few seconds.

We are using git issues to track what needs to be done. If you are unsure where to begin, start by going to [the repo issues page](https://github.com/Springs-DSA/neighborgoods-rs/issues) and picking out an issue with a `good first issue` tag. Make a comment on the issue to let others know you're working on it, and make a branch off of main with the format `ng-<issue number>-short-name`. When you're done with the issue, make a PR against main, and wait for at least one approving review before merging.