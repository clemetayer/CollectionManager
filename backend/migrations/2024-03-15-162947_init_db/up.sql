-- Your SQL goes here
CREATE TABLE "tracks" (
	"id"	INTEGER NOT NULL UNIQUE,
	"deezer_id"	TEXT NOT NULL UNIQUE,
	"title"	TEXT NOT NULL,
	"url" TEXT NOT NULL,
	"artist" TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "collections" (
	"id"	INTEGER NOT NULL UNIQUE,
	"deezer_id" TEXT NOT NULL UNIQUE,
	"name"	TEXT NOT NULL,
	"url" TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "collection_dependencies" (
    "id" INTEGER NOT NULL UNIQUE,
	"parent_id"	INTEGER NOT NULL,
	"child_id"	INTEGER NOT NULL,
    PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("parent_id") REFERENCES collections (id),
	FOREIGN KEY("child_id") REFERENCES collections (id)
);
CREATE TABLE "tracks_in_collection" (
    "id" INTEGER NOT NULL UNIQUE,
	"track_id"	INTEGER NOT NULL,
	"collection_id"	INTEGER NOT NULL,
    PRIMARY KEY("id" AUTOINCREMENT)
	FOREIGN KEY("collection_id") REFERENCES collections (id),
	FOREIGN KEY("track_id") REFERENCES tracks (id)
);