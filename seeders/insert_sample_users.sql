-- INSERT SAMPLE ADMIN
INSERT INTO public.users(name, email, password, phone, created_at, updated_at) VALUES
(
  'Admin',
  'admin@rivvtech.com',
  '$2b$12$Wzy2Cknw.zDV6WyZNbirle9nbqxmEtJ77whBxZOxD4eUGdjxBsy2y', --hashed password: "admin123"
  '+6281234567890',
  CURRENT_TIMESTAMP,
  CURRENT_TIMESTAMP
)
ON CONFLICT (email) DO NOTHING;

-- INSERT SAMPLE KEEPER
INSERT INTO public.users(name, email, password, phone, created_at, updated_at) VALUES
(
  'Keeper',
  'keeper@rivvtech.com',
  '$2b$12$xZDQCLb6oOkO0NCDmSuM0e8oLoHBsvH3s8WQLFtGbE6aAP7Msk/BG', --hashed password: "keeper123"
  '+6281234567891',
  CURRENT_TIMESTAMP,
  CURRENT_TIMESTAMP
)
ON CONFLICT (email) DO NOTHING;

-- INSERT SAMPLE MANAGER
INSERT INTO public.users(name, email, password, phone, created_at, updated_at) VALUES
(
  'Manager',
  'manager@rivvtech.com',
  '$2b$12$eij0YA7M.GjG0Eqpp9jvFOUJL3Ti/M8BvrvO.C04jDietJL.8OeYy', --hashed password: "manager123"
  '+6281234567892',
  CURRENT_TIMESTAMP,
  CURRENT_TIMESTAMP
)
ON CONFLICT (email) DO NOTHING;

-- ASSIGN ROLE TO USER
INSERT INTO public.user_role(user_id, role_id, created_at, updated_at)
SELECT u.id, r.id, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
FROM public.users u
CROSS JOIN public.roles r
WHERE u.email = 'admin@rivvtech.com' AND r.name = 'Admin'
ON CONFLICT (user_id) DO NOTHING;

-- ASSIGN ROLE TO USER
INSERT INTO public.user_role(user_id, role_id, created_at, updated_at)
SELECT u.id, r.id, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
FROM public.users u
CROSS JOIN public.roles r
WHERE u.email = 'manager@rivvtech.com' AND r.name = 'Manager'
ON CONFLICT (user_id) DO NOTHING;

-- ASSIGN ROLE TO USER
INSERT INTO public.user_role(user_id, role_id, created_at, updated_at)
SELECT u.id, r.id, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
FROM public.users u
CROSS JOIN public.roles r
WHERE u.email = 'keeper@rivvtech.com' AND r.name = 'Keeper'
ON CONFLICT (user_id) DO NOTHING;

--verify inserted data
SELECT u.id, u.name, u.email, u.phone, r.name as role_name, u.created_at, u.updated_at
FROM public.users u
LEFT JOIN public.user_role ur ON u.id = ur.user_id
LEFT JOIN public.roles r ON ur.role_id = r.id
WHERE u.email IN ('admin@rivvtech.com', 'manager@rivvtech.com', 'keeper@rivvtech.com')
ORDER BY u.id;
