CREATE TABLE
    lu_grouped_status (
        "id" integer PRIMARY KEY,
        "value" varchar NOT NULL,
        "display_en" varchar NOT NULL,
        "display_pt" varchar NOT NULL
    );

INSERT INTO
    lu_grouped_status ("id", "value", "display_en", "display_pt")
VALUES
    (1, 'dispatching', 'dispatching', 'em despacho'),
    (2, 'active', 'active', 'em curso'),
    (3, 'resolving', 'resolving', 'em resolução'),
    (4, 'concluding', 'concluding', 'em conclusão');

CREATE TABLE
    lu_occurrence_status (
        "id" integer PRIMARY KEY,
        "value" varchar NOT NULL,
        "display_en" varchar NOT NULL,
        "display_pt" varchar NOT NULL
    );

CREATE TABLE
    occurrences (
        "id" integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
        "grouped_status" integer NOT NULL REFERENCES lu_grouped_status ("id"),
        "occurrence_status" integer NOT NULL REFERENCES lu_occurrence_status ("id"),
        "anepc_id" varchar NOT NULL,
        "number_of_operatives" integer NOT NULL,
        "number_of_land_means" integer NOT NULL,
        "number_of_water_means" integer NOT NULL,
        "number_of_air_means" integer NOT NULL
    );