// @generated automatically by Diesel CLI.

diesel::table! {
    pt_account (account_number, plan_id, plan_ver_id, customer_id, trading_group_id) {
        #[max_length = 32]
        account_number -> Varchar,
        plan_id -> Uuid,
        plan_ver_id -> Int4,
        customer_id -> Uuid,
        trading_group_id -> Uuid,
        pass_upgrade_breach_reset_group_id -> Nullable<Uuid>,
        competition_id -> Nullable<Uuid>,
        #[max_length = 128]
        account_trading_login -> Nullable<Varchar>,
        #[max_length = 128]
        account_trading_password -> Nullable<Varchar>,
        #[max_length = 32]
        account_type -> Nullable<Varchar>,
        account_enabled -> Bool,
        account_archived -> Bool,
        account_created_ts -> Timestamp,
        account_lastupdate_ts -> Nullable<Timestamp>,
        #[max_length = 512]
        account_platform_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    pt_account_cash_history (ch_id) {
        ch_id -> Int4,
        #[max_length = 32]
        account_number -> Varchar,
        plan_id -> Uuid,
        plan_ver_id -> Int4,
        customer_id -> Uuid,
        trading_group_id -> Uuid,
        ch_datetime -> Timestamp,
        ch_debt -> Numeric,
        ch_credit -> Numeric,
        ch_balance -> Numeric,
        #[max_length = 32]
        ch_status -> Nullable<Varchar>,
        #[max_length = 256]
        ch_comment -> Nullable<Varchar>,
    }
}

diesel::table! {
    pt_account_details (account_number, plan_id, plan_ver_id, customer_id, trading_group_id) {
        #[max_length = 32]
        account_number -> Varchar,
        plan_id -> Uuid,
        plan_ver_id -> Int4,
        customer_id -> Uuid,
        trading_group_id -> Uuid,
        order_id -> Nullable<Uuid>,
        #[max_length = 64]
        account_display_name -> Nullable<Varchar>,
        account_current_balance -> Nullable<Numeric>,
        account_current_swaps -> Nullable<Numeric>,
        account_current_commissions -> Nullable<Numeric>,
        account_current_profit -> Nullable<Numeric>,
        account_current_equity -> Nullable<Numeric>,
        account_prev_equity -> Nullable<Numeric>,
        account_prev_balance -> Nullable<Numeric>,
        account_daily_loss -> Nullable<Numeric>,
        account_total_loss -> Nullable<Numeric>,
        account_breached -> Nullable<Bool>,
        account_passed -> Nullable<Bool>,
        account_upgraded -> Nullable<Bool>,
        account_resetted -> Nullable<Bool>,
        account_withdrawal_allowed -> Nullable<Bool>,
        account_hit_profit_target -> Nullable<Bool>,
        account_visible_leaderboard -> Nullable<Bool>,
        account_last_history_sync -> Nullable<Timestamp>,
        account_last_order_date_sync -> Nullable<Timestamp>,
        account_last_trade_day_date_sync -> Nullable<Timestamp>,
        account_unique_trade_day_count -> Nullable<Int4>,
        #[max_length = 128]
        account_flow_status -> Nullable<Varchar>,
        #[max_length = 256]
        account_flow_reason -> Nullable<Varchar>,
        account_last_status_change -> Nullable<Timestamp>,
        account_disable_payout -> Bool,
        account_credit_trading_days -> Nullable<Int4>,
        #[max_length = 128]
        account_source_notes -> Nullable<Varchar>,
    }
}

diesel::table! {
    pt_account_metric_history (met_id) {
        met_id -> Int4,
        #[max_length = 32]
        account_number -> Varchar,
        plan_id -> Uuid,
        plan_ver_id -> Int4,
        customer_id -> Uuid,
        trading_group_id -> Uuid,
        met_date -> Nullable<Timestamp>,
        met_active_trading_days -> Nullable<Int4>,
        met_current_profit -> Nullable<Numeric>,
        met_current_commission -> Nullable<Numeric>,
        met_current_swaps -> Nullable<Numeric>,
        met_current_balance -> Nullable<Numeric>,
        met_current_equity -> Nullable<Numeric>,
        met_days_since_initial_deposit -> Nullable<Int4>,
        met_prev_equity -> Nullable<Numeric>,
        met_prev_balance -> Nullable<Numeric>,
        met_daily_loss -> Nullable<Numeric>,
        met_total_loss -> Nullable<Numeric>,
        met_enabled -> Nullable<Bool>,
        met_breached -> Nullable<Bool>,
        met_passed -> Nullable<Bool>,
        met_hit_profit_target -> Nullable<Bool>,
        met_last_history_sync -> Nullable<Timestamp>,
        met_last_order_date_sync -> Nullable<Timestamp>,
        met_last_trade_day_date_sync -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pt_account_metric_history_slow (account_number, plan_id, plan_ver_id, customer_id, trading_group_id, met_date) {
        #[max_length = 32]
        account_number -> Varchar,
        plan_id -> Uuid,
        plan_ver_id -> Int4,
        customer_id -> Uuid,
        trading_group_id -> Uuid,
        met_date -> Timestamp,
        met_active_trading_days -> Nullable<Int4>,
        met_current_profit -> Nullable<Numeric>,
        met_current_commission -> Nullable<Numeric>,
        met_current_swaps -> Nullable<Numeric>,
        met_current_balance -> Nullable<Numeric>,
        met_current_equity -> Nullable<Numeric>,
        met_days_since_initial_deposit -> Nullable<Int4>,
        met_prev_equity -> Nullable<Numeric>,
        met_prev_balance -> Nullable<Numeric>,
        met_daily_loss -> Nullable<Numeric>,
        met_total_loss -> Nullable<Numeric>,
        met_enabled -> Nullable<Bool>,
        met_breached -> Nullable<Bool>,
        met_passed -> Nullable<Bool>,
        met_hit_profit_target -> Nullable<Bool>,
        met_last_history_sync -> Nullable<Timestamp>,
        met_last_order_date_sync -> Nullable<Timestamp>,
        met_last_trade_day_date_sync -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pt_account_opentrades (trade_id, account_number, plan_id, plan_ver_id, customer_id, trading_group_id) {
        #[max_length = 32]
        trade_id -> Varchar,
        #[max_length = 32]
        account_number -> Varchar,
        plan_id -> Uuid,
        plan_ver_id -> Int4,
        customer_id -> Uuid,
        trading_group_id -> Uuid,
        #[max_length = 32]
        pat_symbol -> Nullable<Varchar>,
        pat_digits -> Nullable<Int4>,
        pat_cmd -> Nullable<Int4>,
        pat_volume -> Nullable<Int4>,
        pat_open_time -> Nullable<Timestamp>,
        pat_open_price -> Nullable<Numeric>,
        pat_sl -> Nullable<Numeric>,
        pat_tp -> Nullable<Numeric>,
        pat_close_time -> Nullable<Timestamp>,
        pat_expiration -> Nullable<Timestamp>,
        pat_reason -> Nullable<Int4>,
        pat_conv_rate1 -> Nullable<Numeric>,
        pat_conv_rate2 -> Nullable<Numeric>,
        pat_conv_rate3 -> Nullable<Numeric>,
        pat_commission -> Nullable<Numeric>,
        pat_commission_agent -> Nullable<Numeric>,
        pat_swaps -> Nullable<Numeric>,
        pat_close_price -> Nullable<Numeric>,
        pat_profit -> Nullable<Numeric>,
        pat_taxes -> Nullable<Numeric>,
        #[max_length = 128]
        pat_comment -> Nullable<Varchar>,
        pat_balance -> Nullable<Numeric>,
        pat_margin_rate -> Nullable<Numeric>,
        pat_server_timestamp -> Nullable<Timestamp>,
        #[max_length = 32]
        pat_magic -> Nullable<Varchar>,
        pat_created_ts -> Timestamp,
        pat_lastupdate_ts -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pt_account_trades_history (trade_id, account_number, plan_id, plan_ver_id, customer_id, trading_group_id) {
        #[max_length = 32]
        trade_id -> Varchar,
        #[max_length = 32]
        account_number -> Varchar,
        plan_id -> Uuid,
        plan_ver_id -> Int4,
        customer_id -> Uuid,
        trading_group_id -> Uuid,
        #[max_length = 32]
        ath_symbol -> Nullable<Varchar>,
        ath_digits -> Nullable<Int4>,
        ath_cmd -> Nullable<Int4>,
        ath_volume -> Nullable<Int4>,
        ath_open_time -> Nullable<Timestamp>,
        ath_open_price -> Nullable<Numeric>,
        ath_sl -> Nullable<Numeric>,
        ath_tp -> Nullable<Numeric>,
        ath_close_time -> Nullable<Timestamp>,
        ath_expiration -> Nullable<Timestamp>,
        ath_reason -> Nullable<Int4>,
        ath_conv_rate1 -> Nullable<Numeric>,
        ath_conv_rate2 -> Nullable<Numeric>,
        ath_conv_rate3 -> Nullable<Numeric>,
        ath_commission -> Nullable<Numeric>,
        ath_commission_agent -> Nullable<Numeric>,
        ath_swaps -> Nullable<Numeric>,
        ath_close_price -> Nullable<Numeric>,
        ath_profit -> Nullable<Numeric>,
        ath_taxes -> Nullable<Numeric>,
        #[max_length = 128]
        ath_comment -> Nullable<Varchar>,
        ath_balance -> Nullable<Numeric>,
        ath_margin_rate -> Nullable<Numeric>,
        ath_server_timestamp -> Nullable<Timestamp>,
        #[max_length = 32]
        ath_magic -> Nullable<Varchar>,
        ath_created_ts -> Timestamp,
        ath_lastupdate_ts -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pt_account_withdrawal (wd_id) {
        wd_id -> Int4,
        #[max_length = 32]
        account_number -> Varchar,
        plan_id -> Uuid,
        plan_ver_id -> Int4,
        customer_id -> Uuid,
        trading_group_id -> Uuid,
        actioned_by_principal_id -> Nullable<Uuid>,
        payment_method_id -> Nullable<Uuid>,
        wd_datetime -> Timestamp,
        wd_amount -> Numeric,
        wd_final_amount -> Nullable<Numeric>,
        #[max_length = 32]
        wd_status -> Nullable<Varchar>,
        #[max_length = 256]
        wd_comment -> Nullable<Varchar>,
        #[max_length = 1024]
        wd_response_comment -> Nullable<Varchar>,
    }
}

diesel::table! {
    pt_account_withdrawal_details (wdd_id, wd_id) {
        wdd_id -> Int4,
        wd_id -> Int4,
        #[max_length = 256]
        wdd_item_description -> Varchar,
        #[max_length = 256]
        wdd_item_comment -> Nullable<Varchar>,
        wdd_debit -> Nullable<Numeric>,
        wdd_credit -> Nullable<Numeric>,
        wdd_is_credit -> Bool,
        wdd_timedate -> Timestamp,
    }
}

diesel::joinable!(pt_account_withdrawal_details -> pt_account_withdrawal (wd_id));

diesel::allow_tables_to_appear_in_same_query!(
    pt_account,
    pt_account_cash_history,
    pt_account_details,
    pt_account_metric_history,
    pt_account_metric_history_slow,
    pt_account_opentrades,
    pt_account_trades_history,
    pt_account_withdrawal,
    pt_account_withdrawal_details,
);
