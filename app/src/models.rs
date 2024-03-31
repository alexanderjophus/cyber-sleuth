use diesel::prelude::*;

#[derive(serde::Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::digimon)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[allow(non_snake_case)]
pub struct Digimon {
    pub Name: String,
    pub Stage: Option<String>,
    pub Type: Option<String>,
    pub Attribute: Option<String>,
    pub Memory: Option<i64>,
    pub Equip_Slots: Option<i64>,
    pub HP_lvl_1: Option<i64>,
    pub SP_lvl_1: Option<i64>,
    pub ATK_lvl_1: Option<i64>,
    pub DEF_lvl_1: Option<i64>,
    pub INT_lvl_1: Option<i64>,
    pub SPD_lvl_1: Option<i64>,
    pub HP_lvl_50: Option<i64>,
    pub SP_lvl_50: Option<i64>,
    pub ATK_lvl_50: Option<i64>,
    pub DEF_lvl_50: Option<i64>,
    pub INT_lvl_50: Option<i64>,
    pub SPD_lvl_50: Option<i64>,
    pub HP_lvl_99: Option<i64>,
    pub SP_lvl_99: Option<i64>,
    pub ATK_lvl_99: Option<i64>,
    pub DEF_lvl_99: Option<i64>,
    pub INT_lvl_99: Option<i64>,
    pub SPD_lvl_99: Option<i64>,
}
