CREATE TABLE
    occurrences (
        "id" integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
        "location" geography NOT NULL,
        "kind" integer NOT NULL REFERENCES lu_occurrence_kind_c_c ("id"),
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
        "data_generated_at" timestamptz NOT NULL,
        UNIQUE ("anepc_id", "data_generated_at")
    );