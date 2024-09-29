CREATE TABLE
    lu_grouped_status (
        "id" integer PRIMARY KEY,
        "value" varchar NOT NULL,
        "pt" varchar NOT NULL
    );

INSERT INTO
    lu_grouped_status ("id", "value", "pt")
VALUES
    (1, 'dispatching', 'em despacho'),
    (2, 'active', 'em curso'),
    (3, 'resolving', 'em resolução'),
    (4, 'concluding', 'em conclusão');

CREATE TABLE
    lu_occurrence_status (
        "id" integer PRIMARY KEY,
        "value" varchar NOT NULL
    );

INSERT INTO
    lu_occurrence_status ("id", "value")
VALUES
    (1, 'first_dispatch'),
    (2, 'dispatching'),
    (3, 'site_arrival'),
    (4, 'active'),
    (5, 'resolving'),
    (6, 'concluding'),
    (7, 'monitoring');

CREATE TABLE
    lu_crepc (
        "id" integer PRIMARY KEY,
        "value" varchar NOT NULL UNIQUE,
        "pt" varchar NOT NULL,
        UNIQUE ("id", "value")
    );

INSERT INTO
    lu_crepc ("id", "value", "pt")
VALUES
    (1, 'c1', 'Norte'),
    (2, 'c2', 'Centro'),
    (3, 'c3', 'Lisboa e Vale do Tejo'),
    (4, 'c4', 'Algarve'),
    (5, 'c5', 'Alentejo');

CREATE TABLE
    lu_csrepc (
        "id" integer PRIMARY KEY,
        "value" varchar NOT NULL UNIQUE,
        "pt" varchar NOT NULL,
        UNIQUE ("id", "value")
    );

INSERT INTO
    lu_csrepc ("id", "value", "pt")
VALUES
    (1, 'C1', 'Alto Minho'),
    (2, 'C2', 'Alto Tâmega e Barroso'),
    (3, 'C3', 'Área Metropolitana do Porto'),
    (4, 'C4', 'Ave'),
    (5, 'C5', 'Cávado'),
    (6, 'C6', 'Douro'),
    (7, 'C7', 'Tâmega e Sousa'),
    (8, 'C8', 'Terras de Trás-os-Montes'),
    (9, 'C9', 'Beira Baixa'),
    (10, 'C10', 'Beiras e Serra da Estrela'),
    (11, 'C11', 'Região de Aveiro'),
    (12, 'C12', 'Região de Coimbra'),
    (13, 'C13', 'Região de Leiria'),
    (14, 'C14', 'Viseu Dão Lafões'),
    (15, 'C15', 'Grande Lisboa'),
    (16, 'C16', 'Península de Setúbal'),
    (17, 'C17', 'Lezíria do Tejo'),
    (18, 'C18', 'Médio Tejo'),
    (19, 'C19', 'Oeste'),
    (20, 'C20', 'Alentejo Central'),
    (21, 'C21', 'Alentejo Litoral'),
    (22, 'C22', 'Alto Alentejo'),
    (23, 'C23', 'Baixo Alentejo'),
    (24, 'C24', 'Algarve');

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