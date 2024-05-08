# [b1o.co](https://b1o.co/about)

A Full-Stack Rust application where users can vote for their favorite images. Inspired from the [Elo rating system](https://en.wikipedia.org/wiki/Elo_rating_system#Theory) used in chess, each image is assigned a rating. A [Multi-Layer Feed-Forward Network](https://en.wikipedia.org/wiki/Feedforward_neural_network#Mathematical_foundations) is then used to analyze voting patterns and image characteristics to predict accurate base rankings for new images.

> [!WARNING]
> At the moment, the API is deployed on render ([**_https://api.b1o.co_**](https://api.b1o.co)) which can lead to significant loading times when opening the web-app.

## Overview

- [`/backend`](/backend): Rust API

- [`/client`](/client): Next.js client

- [`/crawler`](/crawler): Perl script to scrape data (following a similar concept to [PageRank's web-indexing](https://en.wikipedia.org/wiki/Web_crawler#Overview)) 

- [`/database`](/database): database models, migrations and seeds

- [`/shared`](/shared): resources used by two or more components

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

## RUNNING ON DOCKER

First install Docker and docker-compose.
You can find instructions [here](https://docs.docker.com/get-docker/) and [here](https://docs.docker.com/compose/install/).

In theory it would be sufficient to run `docker-compose up` and, once everything is up and running, visit http://localhost:3000 to view the app.

> [!NOTE]
> docker-compose will start the app in production mode.

To push to your remote docker-repository run:

```
docker-compose up
docker tag b1o-backend:latest <docker-username>/<docker-repository>:backend
docker tag b1o-frontend:latest <docker-username>/<docker-repository>:frontend
docker push <docker-username>/<docker-repository>:backend
docker push <docker-username>/<docker-repository>:frontend
```

## RUNNING OUTSIDE DOCKER

### Database Setup

Make sure to have a Postgres database with all read and write priviledges running.

Example:

```bash
sudo -u postgres psql
create database b1o;
create user coreuser with encrypted password 'password';
grant all privileges on database b1o to coreuser;
exit
```

> [!NOTE]
> If you get an error message saying "connection refused", your postgres server may have installed on port 5433
> rather than the default 5432. To resolve this, try using the -p flag as follows:
>
> ```bash
> sudo -u postgres psql -p 5433
> ```

### ENV file

You'll need a `.env` file to hold necessary environment variables.
If you're running docker, it should be named `docker.env` and contain the following:

```
FRONTEND_HOST=localhost
FRONTEND_PORT=3000
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

MONGO_URL=http
```

### Running the App

[Cargo Make](https://github.com/sagiegurari/cargo-make) as a task running to simplify starting and stopping the
application.
To use cargo make, run `cargo install cargo-make`.

To start the application in development mode, run `cargo make start_all`.

To start the application in production mode, run `cargo make start_all_prod`.

> [!NOTE]
> On Windows, you might see the following error:
>
> ```powershell
> <e> [webpack-dev-middleware] Error: spawn npm ENOENT
> <e>     at ChildProcess._handle.onexit (node:internal/child_process:285:19)
> <e>     at onErrorNT (node:internal/child_process:483:16)
> <e>     at process.processTicksAndRejections (node:internal/process/task_queues:82:21) {
> <e>   errno: -4058,
> <e>   code: 'ENOENT',
> <e>   syscall: 'spawn npm',
> <e>   path: 'npm',
> <e>   spawnargs: [ 'install', '-g', 'wasm-pack' ]
> <e> }
> ```
>
> If you get this error, run the following command and then try again:
>
> ```
> npm install -g wasm-pack
> ```

To view all the available tasks, open [Makefile.toml](/Makefile.toml).

## CONTRIBUTING

If you find any bugs or have suggestions for improvement, please open a new issue or submit a pull request.

## LICENSE

This project is licensed under the GPL-3.0 license. See the [LICENSE](LICENSE) file for details.

> [!NOTE]
> The general concept has been inspired by [this](https://www.thecrimson.com/article/2003/11/4/hot-or-not-website-briefly-judges/) and [this](https://www.thecrimson.com/article/2003/11/19/facemash-creator-survives-ad-board-the/) article.


---

© Carlo Bortolan

> Carlo Bortolan &nbsp;&middot;&nbsp;
> GitHub [carlobortolan](https://github.com/carlobortolan) &nbsp;&middot;&nbsp;
> contact via [carlobortolan@gmail.com](mailto:carlobortolan@gmail.com)
