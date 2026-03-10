-- Add up migration script here
CREATE TABLE IF NOT EXISTS public.warehouses
(
  id bigserial PRIMARY KEY,
  name varchar(255) NOT NULL,
  address text NOT NULL,
  photo text,
  phone text,
  created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
  deleted_at timestamp with time zone
);

ALTER TABLE public.warehouses
  ADD CONSTRAINT warehouses_name_unique UNIQUE (name);

CREATE INDEX idx_warehouses_name ON public.warehouses (name);
CREATE INDEX idx_warehouses_deleted_at ON public.warehouses (deleted_at);
CREATE INDEX idx_warehouses_created_at ON public.warehouses (created_at);
