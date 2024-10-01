-- -- Table: public.pt_account

-- -- DROP TABLE IF EXISTS public.pt_account;

-- CREATE TABLE IF NOT EXISTS public.pt_account
-- (
--     account_number character varying(32) COLLATE pg_catalog."default" NOT NULL,
--     plan_id uuid NOT NULL,
--     plan_ver_id integer NOT NULL,
--     customer_id uuid NOT NULL,
--     trading_group_id uuid NOT NULL,
--     pass_upgrade_breach_reset_group_id uuid,
--     competition_id uuid,
--     account_trading_login character varying(128) COLLATE pg_catalog."default",
--     account_trading_password character varying(128) COLLATE pg_catalog."default",
--     account_type character varying(32) COLLATE pg_catalog."default",
--     account_enabled boolean NOT NULL,
--     account_archived boolean NOT NULL,
--     account_created_ts timestamp without time zone NOT NULL,
--     account_lastupdate_ts timestamp without time zone,
--     account_platform_url character varying(512) COLLATE pg_catalog."default",
--     CONSTRAINT account_pk PRIMARY KEY (account_number, plan_id, plan_ver_id, customer_id, trading_group_id),
--     CONSTRAINT "Refpt_competition52" FOREIGN KEY (competition_id)
--         REFERENCES public.pt_competition (competition_id) MATCH SIMPLE
--         ON UPDATE NO ACTION
--         ON DELETE NO ACTION,
--     CONSTRAINT "Refpt_plan_m2m_customer34" FOREIGN KEY (plan_id, plan_ver_id, customer_id)
--         REFERENCES public.pt_plan_m2m_customer (plan_id, plan_ver_id, customer_id) MATCH SIMPLE
--         ON UPDATE NO ACTION
--         ON DELETE NO ACTION,
--     CONSTRAINT "Refpt_trading_group33" FOREIGN KEY (trading_group_id)
--         REFERENCES public.pt_trading_group (trading_group_id) MATCH SIMPLE
--         ON UPDATE NO ACTION
--         ON DELETE NO ACTION,
--     CONSTRAINT "Refpt_trading_group50" FOREIGN KEY (pass_upgrade_breach_reset_group_id)
--         REFERENCES public.pt_trading_group (trading_group_id) MATCH SIMPLE
--         ON UPDATE NO ACTION
--         ON DELETE NO ACTION
-- )

-- TABLESPACE pg_default;

-- ALTER TABLE IF EXISTS public.pt_account
--     OWNER to postgres;
-- -- Index: pt_account_idx01

-- -- DROP INDEX IF EXISTS public.pt_account_idx01;

-- CREATE INDEX IF NOT EXISTS pt_account_idx01
--     ON public.pt_account USING btree
--     (competition_id ASC NULLS LAST)
--     TABLESPACE pg_default;
-- -- Index: pt_account_idx02

-- -- DROP INDEX IF EXISTS public.pt_account_idx02;

-- CREATE INDEX IF NOT EXISTS pt_account_idx02
--     ON public.pt_account USING btree
--     (account_trading_login COLLATE pg_catalog."default" ASC NULLS LAST)
--     TABLESPACE pg_default;
-- -- Index: pt_account_idx03

-- -- DROP INDEX IF EXISTS public.pt_account_idx03;

-- CREATE INDEX IF NOT EXISTS pt_account_idx03
--     ON public.pt_account USING btree
--     (account_type COLLATE pg_catalog."default" ASC NULLS LAST)
--     TABLESPACE pg_default;
-- -- Index: pt_account_idx04

-- -- DROP INDEX IF EXISTS public.pt_account_idx04;

-- CREATE INDEX IF NOT EXISTS pt_account_idx04
--     ON public.pt_account USING btree
--     (account_enabled ASC NULLS LAST)
--     TABLESPACE pg_default;
-- -- Index: pt_account_idx05

-- -- DROP INDEX IF EXISTS public.pt_account_idx05;

-- CREATE INDEX IF NOT EXISTS pt_account_idx05
--     ON public.pt_account USING btree
--     (account_archived ASC NULLS LAST)
--     TABLESPACE pg_default;
-- -- Index: pt_account_idx06

-- -- DROP INDEX IF EXISTS public.pt_account_idx06;

-- CREATE INDEX IF NOT EXISTS pt_account_idx06
--     ON public.pt_account USING btree
--     (account_created_ts ASC NULLS LAST)
--     TABLESPACE pg_default;
-- -- Index: pt_account_idx07

-- -- DROP INDEX IF EXISTS public.pt_account_idx07;

-- CREATE INDEX IF NOT EXISTS pt_account_idx07
--     ON public.pt_account USING btree
--     (account_lastupdate_ts ASC NULLS LAST)
--     TABLESPACE pg_default;