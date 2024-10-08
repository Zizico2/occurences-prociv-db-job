use database_utils::lookup_table;

#[lookup_table(table_name = "lu_occurrence_status")]
pub enum OccurrenceStatus {
    FirstDispatch, // 'first_dispatch'
    Dispatching,   // 'dispatching'
    SiteArrival,   // 'site_arrival'
    Active,        // 'active'
    Resolving,     // 'resolving'
    Concluding,    // 'concluding'
    Monitoring,    // 'monitoring'
}

#[lookup_table(table_name = "lu_grouped_status")]
pub enum GroupedStatus {
    Dispatching, // 'dispatching'
    Active,      // 'active'
    Resolving,   // 'resolving'
    Concluding,  // 'concluding'
}

#[lookup_table(table_name = "lu_csrepc")]
pub enum Csrepc {
    C1,  // 'Alto Minho'
    C2,  // 'Alto Tâmega e Barroso'
    C3,  // 'Área Metropolitana do Porto'
    C4,  // 'Ave'
    C5,  // 'Cávado'
    C6,  // 'Douro'
    C7,  // 'Tâmega e Sousa'
    C8,  // 'Terras de Trás-os-Montes'
    C9,  // 'Beira Baixa'
    C10, // 'Beiras e Serra da Estrela'
    C11, // 'Região de Aveiro'
    C12, // 'Região de Coimbra'
    C13, // 'Região de Leiria'
    C14, // 'Viseu Dão Lafões'
    C15, // 'Grande Lisboa'
    C16, // 'Península de Setúbal'
    C17, // 'Lezíria do Tejo'
    C18, // 'Médio Tejo'
    C19, // 'Oeste'
    C20, // 'Alentejo Central'
    C21, // 'Alentejo Litoral'
    C22, // 'Alto Alentejo'
    C23, // 'Baixo Alentejo'
    C24, // 'Algarve'
}

#[lookup_table(table_name = "lu_crepc")]
pub enum Crepc {
    C1, // 'Norte'
    C2, // 'Centro'
    C3, // 'Lisboa e Vale do Tejo'
    C4, // 'Algarve'
    C5, // 'Alentejo'
}

#[lookup_table(table_name = "lu_occurrence_kind")]
pub enum OccurrenceKindFamily {
    C1, // 'Riscos Naturais'
    C2, // 'Riscos Tecnológicos'
    C3, // 'Riscos Mistos'
    C4, // 'Proteção e assistência e pessoas e bens'
    C9, // 'Operações e estados de alerta'
}

#[lookup_table(table_name = "lu_occurrence_kind_c")]
pub enum OccurrenceKindGroup {
    C11, // 'Fenómenos Naturais'
    C21, // 'Indêndios urbanos ou em área urbanizável'
    C22, // 'Indêndios em equipamentos e produtos'
    C23, // 'Indêndios em transportes'
    C24, // 'Acidentes'
    C25, // 'Acidentes industriais e tecnológicos'
    C31, // 'Incêndios rurais'
    C33, // 'Comprometimento total ou parcial de segurança, serviços ou estruturas'
    C41, // 'Assistência em saúde'
    C42, // 'Intervenção em conflitos legais'
    C43, // 'Assistência e prevenção a atividades humanas'
    C91, // 'Operações'
}

#[lookup_table(table_name = "lu_occurrence_kind_c_c")]
pub enum OccurrenceKind {
    C1101, // 'Cheia'
    C1103, // 'Ventos fortes'
    C1105, // 'Sismo'
    C1107, // 'Nevões'
    C1109, // 'Ondas de calor'
    C1111, // 'Ondas de frio'
    C1113, // 'Secas'
    C1119, // 'Colapso de cavidades subterrâneas naturais'
    C1125, // 'Enxurrada/aluvião'
    C2101, // 'Habitacional'
    C2107, // 'Serviços administrativos'
    C2109, // 'Parque escolar'
    C2111, // 'Hospitalares e lares de idosos'
    C2113, // 'Espetáculos e reuniões públicas'
    C2115, // 'Hotelaria e restauração'
    C2117, // 'Áreas comerciais e gares de transporte'
    C2119, // 'Desporto e lazer'
    C2121, // 'Museus e galerias de arte'
    C2123, // 'Bibliotecas e arquivos'
    C2125, // 'Militar, forças de segurança e forças de socorro'
    C2127, // 'Indústria, oficina e armazém'
    C2129, // 'Edifícios degradados ou devolutos'
    C2201, // 'Equipamentos'
    C2203, // 'Produtos'
    C2301, // 'Rodoviário'
    C2303, // 'Aéreo'
    C2305, // 'Ferroviário'
    C2307, // 'Aquático'
    C2401, // 'Atropelamento rodoviário'
    C2403, // 'Colisão rodoviária'
    C2405, // 'Acidentes com veículos fora de estrada'
    C2407, // 'Despiste'
    C2409, // 'Acidente aéreo'
    C2411, // 'Atropelamento ferroviário'
    C2413, // 'Abalroamento ferroviário'
    C2415, // 'Choque entre veículos ou composições ferroviárias'
    C2417, // 'Descarrilamento ferroviário'
    C2419, // 'Afundamento ou adornamento'
    C2421, // 'Encalhe'
    C2423, // 'Colisão aquática'
    C2425, // 'Abalroamento aquático'
    C2501, // 'Radiológicos, dentro de uma instalação'
    C2503, // 'Químicos, dentro de uma instalação'
    C2505, // 'Biológicos, dentro de uma instalação'
    C2507, // 'Radiológicos, em trânsito'
    C2509, // 'Químicos, em trânsito'
    C2511, // 'Biológicos, em trânsito'
    C3101, // 'Povoamento florestal'
    C3103, // 'Mato'
    C3105, // 'Agrícola'
    C3107, // 'Consolidação de rescaldo'
    C3109, // 'Gestão de combustível'
    C3111, // 'Queima'
    C3301, // 'Queda de árvore'
    C3309, // 'Desabamento de estruturas edificadas'
    C3311, // 'Queda de elementos de construção em estruturas edificadas'
    C3313, // 'Movimento de massa'
    C3315, // 'Inundação de estruturas ou superfícies por precipitação intensa'
    C3321, // 'Dano ou queda de redes de fornecimento elétrico'
    C3323, // 'Dano em redes de abastecimento de água'
    C3325, // 'Dano em redes de abastecimento de gás'
    C3327, // 'Dano em oleodutos e gasodutos'
    C3329, // 'Queda de estruturas temporárias ou móveis'
    C3331, // 'Colapso de galerias e cavidades artificiais'
    C3333, // 'Rutura de barragens'
    C4111, // 'Pré-afogamento'
    C4113, // 'Afogamento'
    C4201, // 'Ameaça de explosão'
    C4203, // 'Explosão'
    C4207, // 'Suicídio/homicídio na forma tentada'
    C4209, // 'Suicídio/homicídio consumado'
    C4327, // 'Busca e resgate terrestre de pessoas'
    C4329, // 'Busca e resgate aquático de pessoas'
    C4331, // 'Busca e resgate terrestre de animais'
    C4333, // 'Busca e resgate aquático de animais'
    C4335, // 'Prevenção a queimadas'
    C4339, // 'Corte ou remoção de elementos em perigo de queda'
    C9103, // 'Pré-posicionamento de meios DECIR'
}
