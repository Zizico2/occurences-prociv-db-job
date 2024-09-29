CREATE TABLE
    lu_occurrence_kind (
        "id" integer PRIMARY KEY,
        "value" varchar NOT NULL,
        "pt" varchar NOT NULL
    );

INSERT INTO
    lu_occurrence_kind ("id", "value", "pt")
VALUES
    (1, 'c1', 'Riscos Naturais'),
    (2, 'c2', 'Riscos Tecnológicos'),
    (3, 'c3', 'Riscos Mistos'),
    (
        4,
        'c4',
        'Proteção e assistência e pessoas e bens'
    ),
    (5, 'c9', 'Operações e estados de alerta');

CREATE TABLE
    lu_occurrence_kind_c (
        "id" integer PRIMARY KEY,
        "value" varchar NOT NULL UNIQUE,
        "occurrence_kind" integer REFERENCES lu_occurrence_kind ("id") NOT NULL,
        "pt" varchar NOT NULL,
        UNIQUE ("value", "occurrence_kind")
    );

INSERT INTO
    lu_occurrence_kind_c ("id", "occurrence_kind", "value", "pt")
VALUES
    (1, 1, 'c11', 'Fenómenos Naturais'),
    -- 
    (
        2,
        2,
        'c21',
        'Indêndios urbanos ou em área urbanizável'
    ),
    (
        3,
        2,
        'c22',
        'Indêndios em equipamentos e produtos'
    ),
    (4, 2, 'c23', 'Indêndios em transportes'),
    (5, 2, 'c24', 'Acidentes'),
    (
        6,
        2,
        'c25',
        'Acidentes industriais e tecnológicos'
    ),
    -- 
    (7, 3, 'c31', 'Incêndios rurais'),
    (
        8,
        3,
        'c33',
        'Comprometimento total ou parcial de segurança, serviços ou estruturas'
    ),
    -- 
    (9, 4, 'c41', 'Assistência em saúde'),
    (10, 4, 'c42', 'Intervenção em conflitos legais'),
    (
        11,
        4,
        'c43',
        'Assistência e prevenção a atividades humanas'
    ),
    -- 
    (12, 5, 'c91', 'Operações');