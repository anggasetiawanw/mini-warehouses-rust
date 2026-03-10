INSERT INTO public.roles(name,created_at,updated_at) VALUES
('Admin', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('Keeper', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('Manager', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT (name) DO NOTHING;

SELECT id,name, created_at, updated_at FROM public.roles ORDER BY id;
