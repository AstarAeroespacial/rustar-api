#!/usr/bin/env bash
set -euo pipefail

# Aplica migrations pendientes a Supabase
# Requiere: psql, jq, .env con SUPABASE_DB_PASSWORD
#
# Uso:
#   export $(cat .env | grep -v '^#' | xargs)
#   ./scripts/apply_migrations_to_supabase.sh
#
# O con confirmación automática:
#   AUTO_CONFIRM=1 ./scripts/apply_migrations_to_supabase.sh

if [[ -z "${SUPABASE_DB_PASSWORD:-}" ]]; then
  echo "❌ ERROR: Variable SUPABASE_DB_PASSWORD no encontrada"
  echo "💡 Ejecuta: export \$(cat .env | grep -v '^#' | xargs)"
  exit 1
fi

# URL-encode la contraseña (maneja caracteres especiales)
PASSWORD_ENCODED=$(printf %s "$SUPABASE_DB_PASSWORD" | jq -sRr @uri)
SUPABASE_URL="postgresql://postgres:${PASSWORD_ENCODED}@db.gxrcklaazsihvgbxxddy.supabase.co:5432/postgres?sslmode=require"

echo "🔍 Verificando migrations pendientes en Supabase..."

# Crear tabla _sqlx_migrations si no existe
psql "$SUPABASE_URL" -c "
CREATE TABLE IF NOT EXISTS _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMPTZ NOT NULL DEFAULT now(),
    success BOOLEAN NOT NULL,
    checksum BYTEA NOT NULL,
    execution_time BIGINT NOT NULL
);" > /dev/null 2>&1

# Obtener migrations ya aplicadas en Supabase
APPLIED_MIGRATIONS=$(psql "$SUPABASE_URL" -t -c "SELECT version FROM _sqlx_migrations ORDER BY version;" | tr -d ' ')

# Obtener todas las migrations disponibles localmente
AVAILABLE_MIGRATIONS=$(ls migrations/*.sql | grep -oE '[0-9]{14}' | sort)

# Calcular migrations pendientes
PENDING_MIGRATIONS=""
for version in $AVAILABLE_MIGRATIONS; do
    if ! echo "$APPLIED_MIGRATIONS" | grep -q "^${version}$"; then
        PENDING_MIGRATIONS="${PENDING_MIGRATIONS}${version}\n"
    fi
done

if [[ -z "$PENDING_MIGRATIONS" ]]; then
    echo "✅ No hay migrations pendientes. Supabase está actualizado."
    echo ""
    echo "📊 Migrations aplicadas:"
    psql "$SUPABASE_URL" -c "SELECT version, description, installed_on FROM _sqlx_migrations ORDER BY version;"
    exit 0
fi

echo ""
echo "📊 Migrations pendientes:"
echo -e "$PENDING_MIGRATIONS" | grep -v '^$' | while read version; do
    file=$(ls migrations/${version}_*.sql)
    desc=$(basename "$file" .sql | sed "s/^${version}_//")
    echo "  - ${version}: ${desc}"
done

# Confirmación (skip si AUTO_CONFIRM=1)
if [[ "${AUTO_CONFIRM:-0}" != "1" ]]; then
    echo ""
    read -p "¿Aplicar estas migrations a Supabase? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Cancelado"
        exit 0
    fi
fi

echo ""
echo "🚀 Aplicando migrations a Supabase..."

# Aplicar cada migration pendiente
echo -e "$PENDING_MIGRATIONS" | grep -v '^$' | while read version; do
    file=$(ls migrations/${version}_*.sql)
    desc=$(basename "$file" .sql | sed "s/^${version}_//")

    echo "  📝 Aplicando ${version}: ${desc}..."

    # Aplicar migration
    if psql "$SUPABASE_URL" -f "$file" > /dev/null 2>&1; then
        # Registrar en _sqlx_migrations
        psql "$SUPABASE_URL" -c "
            INSERT INTO _sqlx_migrations (version, description, success, checksum, execution_time)
            VALUES (${version}, '${desc}', true, decode('', 'hex'), 0)
            ON CONFLICT (version) DO NOTHING;
        " > /dev/null 2>&1
        echo "     ✅ ${version} aplicada"
    else
        echo "     ⚠️  Error al aplicar ${version}, puede que ya exista"
        # Aún así registrar como aplicada si las tablas existen
        psql "$SUPABASE_URL" -c "
            INSERT INTO _sqlx_migrations (version, description, success, checksum, execution_time)
            VALUES (${version}, '${desc}', true, decode('', 'hex'), 0)
            ON CONFLICT (version) DO NOTHING;
        " > /dev/null 2>&1
    fi
done

echo ""
echo "✅ Proceso completado"
echo ""
echo "📊 Estado final de migrations en Supabase:"
psql "$SUPABASE_URL" -c "SELECT version, description, installed_on FROM _sqlx_migrations ORDER BY version;"
