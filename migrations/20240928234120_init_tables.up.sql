CREATE TABLE
    lu_grouped_status (
        "id" integer PRIMARY KEY,
        "en" varchar NOT NULL,
        "pt" varchar NOT NULL
    );

INSERT INTO
    lu_grouped_status ("id", "en", "pt")
VALUES
    (1, 'dispatching', 'em despacho'),
    (2, 'active', 'em curso'),
    (3, 'resolving', 'em resolução'),
    (4, 'concluding', 'em conclusão');

CREATE TABLE
    lu_occurrence_status (
        "id" integer PRIMARY KEY,
        "value" varchar NOT NULL,
        "en" varchar NOT NULL,
        "pt" varchar NOT NULL
    );

CREATE TABLE
    lu_crepc (
        "id" integer PRIMARY KEY,
        "pt" varchar NOT NULL
    );

INSERT INTO
    lu_crepc ("id", "pt")
VALUES
    (1, 'Norte'),
    (2, 'Centro'),
    (3, 'Lisboa e Vale do Tejo'),
    (4, 'Algarve'),
    (5, 'Alentejo');

CREATE TABLE
    lu_csrepc (
        "id" integer PRIMARY KEY,
        "value" varchar NOT NULL,
        "en" varchar NOT NULL,
        "pt" varchar NOT NULL
    );

CREATE TABLE
    lu_occurrence_kind (
        "id" integer PRIMARY KEY,
        "value" varchar NOT NULL,
        "en" varchar NOT NULL,
        "pt" varchar NOT NULL
    );

CREATE TABLE
    occurrences (
        "id" integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
        "location" geography NOT NULL,
        "kind" integer NOT NULL REFERENCES lu_occurrence_kind ("id"),
        "grouped_status" integer NOT NULL REFERENCES lu_grouped_status ("id"),
        "occurrence_status" integer NOT NULL REFERENCES lu_occurrence_status ("id"),
        "crepc" integer NOT NULL REFERENCES lu_crepc ("id"),
        "csrepc" integer NOT NULL REFERENCES lu_csrepc ("id"),
        "anepc_id" varchar NOT NULL,
        "number_of_operatives" integer NOT NULL,
        "number_of_land_means" integer NOT NULL,
        "number_of_water_means" integer NOT NULL,
        "number_of_air_means" integer NOT NULL,
        "start_date" timestamptz NOT NULL,
        "data_generated_at" timestamptz NOT NULL
    );