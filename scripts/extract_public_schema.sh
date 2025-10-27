#!/usr/bin/env bash
set -euo pipefail

# Extrae solo el schema 'public' desde un dump completo de Supabase
# Requiere: supabase_schema_full.sql (generado por export_supabase_schema.sh)
#
# Uso:
#   ./scripts/extract_public_schema.sh
#
# Output: supabase_schema_public.sql (solo tablas del schema public)

INPUT_FILE="supabase_schema_full.sql"
OUTPUT_FILE="supabase_schema_public.sql"

if [[ ! -f "$INPUT_FILE" ]]; then
  echo "❌ ERROR: No se encontró $INPUT_FILE"
  echo "💡 Primero ejecuta: ./scripts/export_supabase_schema.sh"
  exit 1
fi

echo "🔍 Extrayendo schema 'public' desde $INPUT_FILE..."
echo ""

# Filtrar solo el contenido del schema public
# Elimina schemas de Supabase (auth, storage, vault, extensions, etc.)
awk '
BEGIN { in_public=1; skip_table=0 }

# Detectar creación de schemas internos de Supabase
/CREATE SCHEMA (auth|storage|vault|extensions|graphql|realtime|supabase_functions|pgsodium)/ { 
    in_public=0; next 
}

# Volver a public cuando aparece
/CREATE SCHEMA public/ { in_public=1; next }
/SET search_path = public/ { in_public=1 }

# Detectar tablas internas de Supabase en public
/CREATE TABLE.*(auth\.|storage\.|vault\.|_realtime|_supabase|supabase_functions|buckets|objects|migrations)/ {
    skip_table=1; next
}

# Fin de comando SQL
/;$/ {
    if (skip_table) {
        skip_table=0
        next
    }
}

# Imprimir solo si estamos en public y no es tabla interna
{
    if (in_public && !skip_table) {
        print
    }
}
' "$INPUT_FILE" > "$OUTPUT_FILE"

# Limpiar líneas vacías consecutivas
sed -i '/^$/N;/^\n$/d' "$OUTPUT_FILE"

echo "✅ Schema 'public' extraído a: $OUTPUT_FILE"
echo ""
echo "📊 Estadísticas:"
echo "   Tamaño: $(du -h "$OUTPUT_FILE" | cut -f1)"
echo "   Líneas: $(wc -l < "$OUTPUT_FILE")"
echo "   Tablas: $(grep -c 'CREATE TABLE' "$OUTPUT_FILE" || echo 0)"
echo ""
echo "📋 Tablas encontradas:"
grep 'CREATE TABLE' "$OUTPUT_FILE" | sed 's/CREATE TABLE /  - /' | sed 's/ (.*//' || echo "  (ninguna)"
echo ""
echo "💡 Siguiente paso:"
echo "   1. Revisa el archivo $OUTPUT_FILE"
echo "   2. Crea una nueva migration: migrations/$(date +%Y%m%d%H%M%S)_import_from_supabase.sql"
echo "   3. Copia el contenido relevante al archivo de migration"
echo "   4. Aplica localmente: sqlx migrate run"
echo "   5. Regenera cache: cargo sqlx prepare"
