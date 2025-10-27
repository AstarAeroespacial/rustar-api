# Scripts de Gestión de Base de Datos

Este directorio contiene scripts para gestionar las migraciones y esquemas entre la base de datos local y Supabase.

## 📋 Scripts Disponibles

### apply_migrations_to_supabase.sh

Aplica migraciones pendientes a la base de datos de Supabase.

**Uso:**

```bash
export $(cat .env | grep -v '^#' | xargs)
./scripts/apply_migrations_to_supabase.sh
```

### export_supabase_schema.sh

Exporta el esquema completo de Supabase a un archivo SQL.

### extract_public_schema.sh

Extrae solo el schema public desde un dump completo.
