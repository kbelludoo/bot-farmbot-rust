#[derive(Debug, Clone)]
pub struct AreaCandidate {
    pub id: &'static str,
    pub min_level: u32,
    pub max_level: u32,
    pub base_exp: f64,
    pub base_gold: f64,
}

pub static HUNTS_CATALOG: &[AreaCandidate] = &[
        AreaCandidate { id: "fase_1_bosque", min_level: 1, max_level: 15, base_exp: 100.0, base_gold: 50.0 },
        AreaCandidate { id: "fase_2_horta", min_level: 15, max_level: 35, base_exp: 260.0, base_gold: 140.0 },
        AreaCandidate { id: "fase_3_moinho", min_level: 35, max_level: 65, base_exp: 700.0, base_gold: 390.0 },
        AreaCandidate { id: "fase_4_mina", min_level: 65, max_level: 105, base_exp: 1800.0, base_gold: 1100.0 },
        AreaCandidate { id: "fase_5_pantano", min_level: 105, max_level: 155, base_exp: 4400.0, base_gold: 2800.0 },
        AreaCandidate { id: "fase_6_caverna_cristal", min_level: 155, max_level: 220, base_exp: 11000.0, base_gold: 7200.0 },
        AreaCandidate { id: "fase_7_vulcao", min_level: 220, max_level: 320, base_exp: 27000.0, base_gold: 18500.0 },
];

#[derive(Clone, Default)]
pub struct Catalog;

impl Catalog {
    pub fn select_best_hunt(&self, level: u32) -> &'static AreaCandidate {
        for area in HUNTS_CATALOG {
            if level >= area.min_level && level <= area.max_level {
                return area;
            }
        }
        &HUNTS_CATALOG[HUNTS_CATALOG.len() - 1]
    }
}
