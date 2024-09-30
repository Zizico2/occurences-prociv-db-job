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
    (1, 'c1', 'Alto Minho'),
    (2, 'c2', 'Alto Tâmega e Barroso'),
    (3, 'c3', 'Área Metropolitana do Porto'),
    (4, 'c4', 'Ave'),
    (5, 'c5', 'Cávado'),
    (6, 'c6', 'Douro'),
    (7, 'c7', 'Tâmega e Sousa'),
    (8, 'c8', 'Terras de Trás-os-Montes'),
    (9, 'c9', 'Beira Baixa'),
    (10, 'c10', 'Beiras e Serra da Estrela'),
    (11, 'c11', 'Região de Aveiro'),
    (12, 'c12', 'Região de Coimbra'),
    (13, 'c13', 'Região de Leiria'),
    (14, 'c14', 'Viseu Dão Lafões'),
    (15, 'c15', 'Grande Lisboa'),
    (16, 'c16', 'Península de Setúbal'),
    (17, 'c17', 'Lezíria do Tejo'),
    (18, 'c18', 'Médio Tejo'),
    (19, 'c19', 'Oeste'),
    (20, 'c20', 'Alentejo Central'),
    (21, 'c21', 'Alentejo Litoral'),
    (22, 'c22', 'Alto Alentejo'),
    (23, 'c23', 'Baixo Alentejo'),
    (24, 'c24', 'Algarve');

