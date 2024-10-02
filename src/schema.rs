// @generated automatically by Diesel CLI.

diesel::table! {
    pt_account (account_number, plan_id, plan_ver_id, customer_id, trading_group_id) {
        
        account_number -> Varchar,
        plan_id -> Uuid,
        plan_ver_id -> Int4,
        customer_id -> Uuid,
        trading_group_id -> Uuid,
        pass_upgrade_breach_reset_group_id -> Nullable<Uuid>,
        competition_id -> Nullable<Uuid>,
        
        account_trading_login -> Nullable<Varchar>,
        
        account_trading_password -> Nullable<Varchar>,
        
        account_type -> Nullable<Varchar>,
        account_enabled -> Bool,
        account_archived -> Bool,
        account_created_ts -> Timestamp,
        account_lastupdate_ts -> Nullable<Timestamp>,
    }
}