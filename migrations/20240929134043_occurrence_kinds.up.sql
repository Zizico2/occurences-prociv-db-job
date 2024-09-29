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

CREATE TABLE
    lu_occurrence_kind_c_c (
        "id" integer PRIMARY KEY,
        "value" varchar NOT NULL UNIQUE,
        "occurrence_kind_c" integer REFERENCES lu_occurrence_kind_c ("id") NOT NULL,
        "pt" varchar NOT NULL,
        UNIQUE ("value", "occurrence_kind_c")
    );

INSERT INTO
    lu_occurrence_kind_c_c ("id", "occurrence_kind_c", "value", "pt")
VALUES
    -- Riscos Naturais
    (1, 1, 'c1101', 'Cheia'),
    (2, 1, 'c1103', 'Ventos fortes'),
    (3, 1, 'c1105', 'Sismo'),
    (4, 1, 'c1107', 'Nevões'),
    (5, 1, 'c1109', 'Ondas de calor'),
    (6, 1, 'c1111', 'Ondas de frio'),
    (7, 1, 'c1113', 'Secas'),
    (8, 1, 'c1119', 'Colapso de cavidades subterrâneas naturais'),
    (9, 1, 'c1125', 'Enxurrada/aluvião'),

    -- Riscos Tecnológicos - Incêndios urbanos ou em área urbanizável
    (10, 2, 'c2101', 'Habitacional'),
    (11, 2, 'c2107', 'Serviços administrativos'),
    (12, 2, 'c2109', 'Parque escolar'),
    (13, 2, 'c2111', 'Hospitalares e lares de idosos'),
    (14, 2, 'c2113', 'Espetáculos e reuniões públicas'),
    (15, 2, 'c2115', 'Hotelaria e restauração'),
    (16, 2, 'c2117', 'Áreas comerciais e gares de transporte'),
    (17, 2, 'c2119', 'Desporto e lazer'),
    (18, 2, 'c2121', 'Museus e galerias de arte'),
    (19, 2, 'c2123', 'Bibliotecas e arquivos'),
    (20, 2, 'c2125', 'Militar, forças de segurança e forças de socorro'),
    (21, 2, 'c2127', 'Indústria, oficina e armazém'),
    (22, 2, 'c2129', 'Edifícios degradados ou devolutos'),

    -- Riscos Tecnológicos - Incêndios em equipamentos e produtos
    (23, 3, 'c2201', 'Equipamentos'),
    (24, 3, 'c2203', 'Produtos'),

    -- Riscos Tecnológicos - Incêndios em transportes
    (25, 4, 'c2301', 'Rodoviário'),
    (26, 4, 'c2303', 'Aéreo'),
    (27, 4, 'c2305', 'Ferroviário'),
    (28, 4, 'c2307', 'Aquático'),

    -- Riscos Tecnológicos - Acidentes
    (29, 5, 'c2401', 'Atropelamento rodoviário'),
    (30, 5, 'c2403', 'Colisão rodoviária'),
    (31, 5, 'c2405', 'Acidentes com veículos fora de estrada'),
    (32, 5, 'c2407', 'Despiste'),
    (33, 5, 'c2409', 'Acidente aéreo'),
    (34, 5, 'c2411', 'Atropelamento ferroviário'),
    (35, 5, 'c2413', 'Abalroamento ferroviário'),
    (36, 5, 'c2415', 'Choque entre veículos ou composições ferroviárias'),
    (37, 5, 'c2417', 'Descarrilamento ferroviário'),
    (38, 5, 'c2419', 'Afundamento ou adornamento'),
    (39, 5, 'c2421', 'Encalhe'),
    (40, 5, 'c2423', 'Colisão aquática'),
    (41, 5, 'c2425', 'Abalroamento aquático'),

    -- Riscos Tecnológicos - Acidentes industriais e tecnológicos
    (42, 6, 'c2501', 'Radiológicos, dentro de uma instalação'),
    (43, 6, 'c2503', 'Químicos, dentro de uma instalação'),
    (44, 6, 'c2505', 'Biológicos, dentro de uma instalação'),
    (45, 6, 'c2507', 'Radiológicos, em trânsito'),
    (46, 6, 'c2509', 'Químicos, em trânsito'),
    (47, 6, 'c2511', 'Biológicos, em trânsito'),

    -- Riscos Mistos - Incêndios rurais
    (48, 7, 'c3101', 'Povoamento florestal'),
    (49, 7, 'c3103', 'Mato'),
    (50, 7, 'c3105', 'Agrícola'),
    (51, 7, 'c3107', 'Consolidação de rescaldo'),
    (52, 7, 'c3109', 'Gestão de combustível'),
    (53, 7, 'c3111', 'Queima'),

    -- Riscos Mistos - Comprometimento total ou parcial de segurança, serviços ou estruturas
    (54, 8, 'c3301', 'Queda de árvore'),
    (55, 8, 'c3309', 'Desabamento de estruturas edificadas'),
    (56, 8, 'c3311', 'Queda de elementos de construção em estruturas edificadas'),
    (57, 8, 'c3313', 'Movimento de massa'),
    (58, 8, 'c3315', 'Inundação de estruturas ou superfícies por precipitação intensa'),
    (59, 8, 'c3321', 'Dano ou queda de redes de fornecimento elétrico'),
    (60, 8, 'c3323', 'Dano em redes de abastecimento de água'),
    (61, 8, 'c3325', 'Dano em redes de abastecimento de gás'),
    (62, 8, 'c3327', 'Dano em oleodutos e gasodutos'),
    (63, 8, 'c3329', 'Queda de estruturas temporárias ou móveis'),
    (64, 8, 'c3331', 'Colapso de galerias e cavidades artificiais'),
    (65, 8, 'c3333', 'Rutura de barragens'),

    -- Proteção e assistência a pessoas e bens - Assistência em saúde
    (66, 9, 'c4111', 'Pré-afogamento'),
    (67, 9, 'c4113', 'Afogamento'),

    -- Proteção e assistência a pessoas e bens - Intervenção em conflitos legais
    (68, 10, 'c4201', 'Ameaça de explosão'),
    (69, 10, 'c4203', 'Explosão'),
    (70, 10, 'c4207', 'Suicídio/homicídio na forma tentada'),
    (71, 10, 'c4209', 'Suicídio/homicídio consumado'),

    -- Proteção e assistência a pessoas e bens - Assistência e prevenção a atividades humanas
    (72, 11, 'c4327', 'Busca e resgate terrestre de pessoas'),
    (73, 11, 'c4329', 'Busca e resgate aquático de pessoas'),
    (74, 11, 'c4331', 'Busca e resgate terrestre de animais'),
    (75, 11, 'c4333', 'Busca e resgate aquático de animais'),
    (76, 11, 'c4335', 'Prevenção a queimadas'),
    (77, 11, 'c4339', 'Corte ou remoção de elementos em perigo de queda'),

    -- Operações e estados de alerta
    (78, 12, 'c9103', 'Pré-posicionamento de meios DECIR');
