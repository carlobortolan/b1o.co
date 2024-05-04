# TickTack

A Full-Stack Rust application empowered by WebAssembly (WASM).

> [!IMPORTANT]
> As of February 18, 2024, this project has been put on hold and will probably not be worked on or finished in the near future.

> [!NOTE]
> At the moment_ [___https://ticktack-api.onrender.com___](https://ticktack-api.onrender.com/watches) _hosts the API and_ [___https://ticktack.carlobortolan.com___](https://ticktack.carlobortolan.com) _hosts the Frontend. You can find the docker-repository for the Frontend-image at `docker.io/carlobortolan/ticktack:frontend`.

> __DISCLAIMER__: _This is inspired by the [rust-fullstack-demo](https://github.com/toadslop/rust-fullstack-demo) as seen in [__"Tokyo Rust
Meetup - Fullstack Web Dev in Rust"__](https://www.youtube.com/watch?v=5el5aFoJ8ws) and used to have a fun project for
> experimenting with Rust and WASM._

## FUNCTIONALITY

WASM based web-app that keeps track of the most polular mechanical wristwatches at the time, allowing users to rate and comment their favorite timepieces.

<!-- Or in the words of Ben Clymer (/GPT3.5):
> _"With every click, TickTack transports you into a realm where watch connoisseurs unite in a symphony of vibrant
conversations and captivating visual narratives. Delve into a luminous palette of watch photography, each frame a
testament to the artistry, craftsmanship, and boundless creativity of watchmakers from across the globe._
>
> _As you traverse the opulent landscapes of TickTack, be prepared to engage in a realm of intellectual discourse and
enlightening dialogues. Connect with fellow horological explorers who share a fervor for the intricacies of watchmaking._
>
> _Capture your own horological adventures through the lens of your timepiece, sharing your chronometric masterpieces
with an audience that awaits your narrative. Experience the thrill of camaraderie as passionate watch collectors,
seasoned aficionados, and wide-eyed novices converge, embracing the boundless spectrums of time and color that unite us
all._
>
> _Join the luminous world of TickTack, where time is an exquisite work of art, and watches become the brushstrokes that
paint your wrist with unparalleled elegance. Let the vibrant spirit of TickTack ignite your imagination, as we celebrate
the hues, stories, and the infinite palette of possibilities that watches bring to our lives."_ -->

## CONFIG

<details><summary><b>Start backend</b></summary>

`cargo build`

`cargo install cargo-make`

`cargo run --bin backend`

**OR**

`cargo make start_back_prod`
</details>

<details><summary><b>Start frontend</b></summary>

(`npm install`)

(`npm install -g pnpm`)

(`npm install -g wasm-pack`)

`pnpm install --dir ./frontend`

`pnpm run --dir ./frontend build`

`pnpm run --dir ./frontend dev`

**OR**

`cargo make start_front_prod`

**OR**

`simple-http-server ./frontend/dist -i -p 8000 --nocache --try-file ./frontend/dist/index.html`

> [!NOTE]
> You might have to `Set-ExecutionPolicy RemoteSigned` to run pnpm commands on Windows.

</details>

## ITEMS DEMONSTRATED IN THIS APP

### General

* Compile time environment variables
* Runtime environment variables
* Sharing entities between frontend and backend
* Dockerizing Rust frontends and backends
* Running tasks with [Cargo Make](https://github.com/sagiegurari/cargo-make)

### Database

* Object Relational Mapping
* Seeding with random data
* Seeding with CSV data
* Entity Definitions

### Backend

* Routing
* Middleware
* CORS configuration
* Application State
* Database connections

### Frontend

* Asynchronous data fetching
* Functional components
* State management with hooks
* Posting data
* Building with webpack

## RUNNING ON DOCKER

First install Docker.
You can find instructions [here](https://docs.docker.com/get-docker/).

Next, install `docker-compose` to build and run several docker containers simultaneously.
Instructions can be found [here](https://docs.docker.com/compose/install/).

In theory it would be sufficient to run `docker-compose up` and, once everything is up and running, visit http://localhost:8000 to view the app.

Note that docker-compose will start the app in production mode.

To push to your remote docker-repository run:
```
docker-compose up
docker tag ticktack-backend:latest <docker-username>/<docker-repository>:backend
docker tag ticktack-frontend:latest <docker-username>/<docker-repository>:frontend
docker push <docker-username>/<docker-repository>:backend
docker push <docker-username>/<docker-repository>:frontend
```


## RUNNING OUTSIDE DOCKER

### Database Installation

This app requires a Postgres 14 database.

Downloads for the various operating systems can be
found [here](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads).

#### Windows

Refer to this [article](https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql/).

After installation, open your terminal and try running "psql".
Keep in mind that you will need to set the version to 14.

#### Mac

Refer to this [article](https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql-macos/)

#### Linux

Refer to this [article](https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql-linux/)

### Database Setup

Once you have a Postgres server up and running, create a database and make sure your database user has read and write
privileges.

Example:

```bash
sudo -u postgres psql
create database coredb;
create user coreuser with encrypted password 'password';
grant all privileges on database coredb to coreuser;
exit
```

NOTE: If you get an error message saying "connection refused", your postgres server may have installed on port 5433
rather than the default 5432. To resolve this, try using the -p flag as follows:

```bash
sudo -u postgres psql -p 5433
```

### ENV file

You'll need a file to hold necessary environment variables.
If you're running docker, it should be named `docker.env` and contain the following:

```
FRONTEND_HOST=localhost
FRONTEND_PORT=8000
FRONTEND_PROTOCOL=http

DATABASE_PROTOCOL=postgres
DATABASE_PORT=5432
DATABASE_URL=<SECRET>
POSTGRES_USER=<SECRET>
POSTGRES_PASSWORD=<SECRET>
POSTGRES_DB=<SECRET>
POSTGRES_HOST=<SECRET>

BACKEND_HOST=localhost
BACKEND_HOST_INTERNAL=0.0.0.0
BACKEND_PORT=8080
BACKEND_PROTOCOL=http
```

If you're running it outside of docker, the file should be called `.env` and should contain the following:

```
FRONTEND_HOST=localhost
FRONTEND_PORT=8000
FRONTEND_PROTOCOL=http

DATABASE_PROTOCOL=postgres
DATABASE_PORT=5432
DATABASE_URL=<SECRET>
POSTGRES_USER=<SECRET>
POSTGRES_PASSWORD=<SECRET>
POSTGRES_DB=<SECRET>
POSTGRES_HOST=<SECRET>

BACKEND_HOST=localhost
BACKEND_PORT=8080
BACKEND_PROTOCOL=http
```

### Running the App

[Cargo Make](https://github.com/sagiegurari/cargo-make) as a task running to simplify starting and stopping the
application.
To use cargo make, run `cargo install cargo-make`.

To start the application in development mode, run `cargo make start_all`.

To start the application in production mode, run `cargo make start_all_prod`.

Note: On Windows, you might see the following error:

```powershell
<e> [webpack-dev-middleware] Error: spawn npm ENOENT
<e>     at ChildProcess._handle.onexit (node:internal/child_process:285:19)
<e>     at onErrorNT (node:internal/child_process:483:16)
<e>     at process.processTicksAndRejections (node:internal/process/task_queues:82:21) {
<e>   errno: -4058,
<e>   code: 'ENOENT',
<e>   syscall: 'spawn npm',
<e>   path: 'npm',
<e>   spawnargs: [ 'install', '-g', 'wasm-pack' ]
<e> }
```

If you get this error, run the following command and then try again:

```
npm install -g wasm-pack
```

To view all the available tasks, open [Makefile.toml](/Makefile.toml).

## CONTRIBUTING

Contributions are welcome! If you find a bug or have an idea for a new feature, please open an issue or submit a pull
request.

## LICENSE

This project is licensed under the GPL-3.0 license. See the [LICENSE](LICENSE) file for details.

---

© Carlo Bortolan

> Carlo Bortolan &nbsp;&middot;&nbsp;
> GitHub [carlobortolan](https://github.com/carlobortolan) &nbsp;&middot;&nbsp;
> contact via [carlobortolan@gmail.com](mailto:carlobortolan@gmail.com)
