-- Table: public.pt_feature

-- DROP TABLE IF EXISTS public.pt_feature;

CREATE TABLE IF NOT EXISTS public.pt_feature
(
    feature_id uuid NOT NULL DEFAULT gen_random_uuid(),
    feature_name character varying(128) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT feature_pk PRIMARY KEY (feature_id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.pt_feature
    OWNER to postgres;
-- Index: pt_feature_idx01

-- DROP INDEX IF EXISTS public.pt_feature_idx01;

CREATE UNIQUE INDEX IF NOT EXISTS pt_feature_idx01
    ON public.pt_feature USING btree
    (feature_name COLLATE pg_catalog."default" ASC NULLS LAST)
    TABLESPACE pg_default;