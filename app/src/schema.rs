// @generated automatically by Diesel CLI.

diesel::table! {
    digimon (Name) {
        #[max_length = 1024]
        Name -> Varchar,
        #[max_length = 1024]
        Stage -> Nullable<Varchar>,
        #[max_length = 1024]
        Type -> Nullable<Varchar>,
        #[max_length = 1024]
        Attribute -> Nullable<Varchar>,
        Memory -> Nullable<Bigint>,
        #[sql_name = "Equip Slots"]
        Equip_Slots -> Nullable<Bigint>,
        #[sql_name = "HP lvl 1"]
        HP_lvl_1 -> Nullable<Bigint>,
        #[sql_name = "SP lvl 1"]
        SP_lvl_1 -> Nullable<Bigint>,
        #[sql_name = "ATK lvl 1"]
        ATK_lvl_1 -> Nullable<Bigint>,
        #[sql_name = "DEF lvl 1"]
        DEF_lvl_1 -> Nullable<Bigint>,
        #[sql_name = "INT lvl 1"]
        INT_lvl_1 -> Nullable<Bigint>,
        #[sql_name = "SPD lvl 1"]
        SPD_lvl_1 -> Nullable<Bigint>,
        #[sql_name = "HP lvl 50"]
        HP_lvl_50 -> Nullable<Bigint>,
        #[sql_name = "SP lvl 50"]
        SP_lvl_50 -> Nullable<Bigint>,
        #[sql_name = "ATK lvl 50"]
        ATK_lvl_50 -> Nullable<Bigint>,
        #[sql_name = "DEF lvl 50"]
        DEF_lvl_50 -> Nullable<Bigint>,
        #[sql_name = "INT lvl 50"]
        INT_lvl_50 -> Nullable<Bigint>,
        #[sql_name = "SPD lvl 50"]
        SPD_lvl_50 -> Nullable<Bigint>,
        #[sql_name = "HP lvl 99"]
        HP_lvl_99 -> Nullable<Bigint>,
        #[sql_name = "SP lvl 99"]
        SP_lvl_99 -> Nullable<Bigint>,
        #[sql_name = "ATK lvl 99"]
        ATK_lvl_99 -> Nullable<Bigint>,
        #[sql_name = "DEF lvl 99"]
        DEF_lvl_99 -> Nullable<Bigint>,
        #[sql_name = "INT lvl 99"]
        INT_lvl_99 -> Nullable<Bigint>,
        #[sql_name = "SPD lvl 99"]
        SPD_lvl_99 -> Nullable<Bigint>,
    }
}

diesel::table! {
    digivolution_requirements (Name) {
        #[max_length = 1024]
        Name -> Varchar,
        Level -> Nullable<Bigint>,
        #[max_length = 1024]
        HP -> Nullable<Varchar>,
        #[max_length = 1024]
        SP -> Nullable<Varchar>,
        #[max_length = 1024]
        ATK -> Nullable<Varchar>,
        #[max_length = 1024]
        DEF -> Nullable<Varchar>,
        #[max_length = 1024]
        INT -> Nullable<Varchar>,
        #[max_length = 1024]
        SPD -> Nullable<Varchar>,
        #[max_length = 1024]
        ABI -> Nullable<Varchar>,
        #[max_length = 1024]
        CAM -> Nullable<Varchar>,
        #[sql_name = "Extra Condition"]
        #[max_length = 1024]
        Extra_Condition -> Nullable<Varchar>,
    }
}

diesel::table! {
    digivolutions (Digimon_From, Digimon_To) {
        #[sql_name = "Digimon From"]
        #[max_length = 1024]
        Digimon_From -> Varchar,
        #[sql_name = "Digimon To"]
        #[max_length = 1024]
        Digimon_To -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    digimon,
    digivolution_requirements,
    digivolutions,
);
