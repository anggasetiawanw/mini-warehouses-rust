DB_URL=${1:-"postgresql://postgres:password@localhost:5432/mini_warehouses"}

if ! command -v psql >/dev/null 2>&1; then
    echo "Error: psql is not installed or not in PATH."
    echo "Install PostgreSQL client tools and re-run this script."
    exit 127
fi

echo "Running seeders with DB_URL: $DB_URL"
echo "===================================="

run_sql_file() {
    local sql_file=$1
    local description=$2

    echo "Running: $description"
    if psql "$DB_URL" -v ON_ERROR_STOP=1 -f "$sql_file"; then
        echo "Successfully ran: $description"
    else
        echo "Error running: $description"
        exit 1
    fi
    echo "------------------------------------"

}

echo "Starting seeding process..."
run_sql_file "seeders/insert_default_roles.sql" "Insert Default Roles"
run_sql_file "seeders/insert_sample_users.sql" "Insert Sample Users"
echo "Seeding process completed successfully!"
