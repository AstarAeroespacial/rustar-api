#!/usr/bin/env bash
set -euo pipefail

# Exporta el schema completo de Supabase (incluyendo schemas internos)
# Requiere: pg_dump, jq, .env con SUPABASE_DB_PASSWORD
#
# Uso:
#   export $(cat .env | grep -v '^#' | xargs)
#   ./scripts/export_supabase_schema.sh
#
# Output: supabase_schema_full.sql (incluye auth, storage, vault, etc.)

if [[ -z "${SUPABASE_DB_PASSWORD:-}" ]]; then
  echo "❌ ERROR: Variable SUPABASE_DB_PASSWORD no encontrada"
  echo "💡 Ejecuta: export \$(cat .env | grep -v '^#' | xargs)"
  exit 1
fi

OUTPUT_FILE="supabase_schema_full.sql"

echo "🔍 Exportando schema completo desde Supabase..."
echo "   Host: db.gxrcklaazsihvgbxxddy.supabase.co"
echo "   Database: postgres"
echo ""

# URL-encode la contraseña
PASSWORD_ENCODED=$(printf %s "$SUPABASE_DB_PASSWORD" | jq -sRr @uri)

# Exportar schema completo (sin datos)
PGPASSWORD="$SUPABASE_DB_PASSWORD" pg_dump \
  -h db.gxrcklaazsihvgbxxddy.supabase.co \
  -U postgres \
  -d postgres \
  -p 5432 \
  --schema-only \
  --no-owner \
  --no-acl \
  > "$OUTPUT_FILE"

echo "✅ Schema exportado a: $OUTPUT_FILE"
echo ""
echo "📊 Estadísticas del archivo:"
echo "   Tamaño: $(du -h "$OUTPUT_FILE" | cut -f1)"
echo "   Líneas: $(wc -l < "$OUTPUT_FILE")"
echo "   Tablas: $(grep -c 'CREATE TABLE' "$OUTPUT_FILE" || echo 0)"
echo "   Schemas: $(grep -c 'CREATE SCHEMA' "$OUTPUT_FILE" || echo 0)"
echo ""
echo "💡 Esquemas incluidos:"
grep 'CREATE SCHEMA' "$OUTPUT_FILE" | sed 's/CREATE SCHEMA /  - /' || echo "  (ninguno explícito, usando public)"
echo ""
echo "🔧 Para extraer solo el schema 'public', ejecuta:"
echo "   ./scripts/extract_public_schema.sh"
