# CollectionManager

A music "collection" manager (kind of like playlist that contains tracks and other playlists)

Made to manage music lists in another way than playlists.

A collection can contain tracks (like a playlist), but also other collections, integrating the tracks from the other collection in the collection.

It was made to be used with Deezer.

From a technical point of view, this will automatically add the tracks from the children collections in the parent collection.

Usage example :
let's say you have a rock playlist and many other related playlists (90's rock, hard rock, etc.), the collection manager will allow you to have the rock playlist "contain" its related playlists, meaning the rock playlist will have the tracks of the related playlist and be updated if tracks are added in the related playlists.

## Disclaimer

### Limitations

Since this is not a native Deezer solution, this app have limitations :

- If a track is added in a child collection, you will have to trigger the update manually
- this app will **never** remove tracks from a playlist, you will have to do this manually. This app can only remove the "concept" of collection that will only affect the track update.
  -For instance if you have a collection "parent collection" that contains a collection "children collection", if you remove "children collection", "children collection" will still appear on deezer and its tracks will still be in "parent collection", but if you add tracks to "children collection", "parent collection" will not be updated.
- This app allow cyclic dependencies (i.e : "collection 1" containing "collection 2" and "collection 2" containing "collection 1" is valid, this will result in both collections having the same tracks). However, to avoid infinite loops while updating, a maximum number of iterations `MAX_COLLECTION_DEPTH` has been set in the `.env` file in the backend
- A rate limiter has been added to avoid querying the Deezer API too much (by default 40 queries per 5 seconds). the parameters can be tweaked in the `.env` file in the backend

### Maintenance

Since this is an app I made for mostly a personnal use, and mostly to learn new technical things, the maintenance on this app may be limited.

## Usage

- First, you have to create your deezer API token and add it in the .env file, along with your deezer id. To do this, follow [this guide](https://developers.deezer.com/api/oauth) with at least the permission "manage_library".
- you will then have to create the sqlite database in the `backend/db` folder. To do this :
  - install [diesel](https://diesel.rs/) : `cd backend`, `cargo install diesel_cli` (cargo needs to be installed first)
  - `diesel setup` (this will create collections.sqlite3 in the db folder)
  - `diesel migration run` (this will create the database model)

### With docker compose

- cd `ci/local`
- `sudo docker-compose up`
- the app is running on `http://localhost:5173`

### Without docker

- cd `backend`
- `RUST_LOG=info cargo run`
- in another terminal, `cd frontend`
- `npm install` (the first time)
- `npm run dev`

## Technical stuff

### Versions

#### Backend

- rustc 1.76.0
- cargo 1.76.0
- diesel 2.2.0

#### Frontend

- npm 10.8.1
- node v22.3.0

#### Other

- Docker version 26.1.4, build 5650f9b102
- Docker Compose version 2.27.1

### Customizing

This app was intended to work with the Deezer API, but it can be customized to work with any other music service

### Testing

#### Integration tests

##### Backend

- `cd backend`
- copy `db/collections.sqlite3` as `db/collections_tests.sqlite3`
- Then run the mocked deezer api + the backend
  - `cd backend`, `RUST_LOG=info cargo run -- integration_tests`, in another terminal, `cd ci/integration_back_mockserver_only`, `sudo docker-compose up`
  - or `cd ci/integration_back`, `sudo docker-compose up`
- Install launch [bruno](https://www.usebruno.com/), import the collections in `backend/bruno`, and execute any test suite

##### Frontend

- `cd frontend`
- run the mocked backend + the frontend
  - `cd frontend`, `npm run dev`, in another terminal, `cd ci/integration_front_mockserver_only`, `sudo docker-compose up`
  - or `cd ci/integration_front`, `sudo docker-compose up`
- in another terminal `cd frontend`, `npm run cy:open`, then run the integration_tests suite

#### System tests

- run the mocked deezer api + the backend
  - `cd backend`, `RUST_LOG=info cargo run -- integration_tests`, in another terminal, `cd ci/integration_back_mockserver_only`, `sudo docker-compose up`
  - or `cd ci/integration_back`, `sudo docker-compose up`
- in another terminal, run the frontend `cd frontend`, `npm run dev`
- in another terminal, `cd frontend`, `npm run cy:open`, then run the system_tests suite
