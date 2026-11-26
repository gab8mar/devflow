# DevTime

> "Trabaja normalmente. DevTime recuerda por ti."

Aplicación de escritorio Windows, ligera y no invasiva, que registra el tiempo real
invertido por tarea y prepara el llenado automático del Timesheet corporativo.

**Estado actual:** Fase 2 en curso — cronómetro persistente end-to-end (iniciar, pausar,
reanudar, finalizar, recuperación tras cierre y totales diarios), ventana flotante arrastrable
y catálogo de tareas con alta local. Siguiente: integración del PWA, CRUD de proyectos/tareas,
historial de actividad y búsqueda del switcher.

Qué funciona hoy:
- Píldora 132×28 always-on-top, arrastrable, con tiempo derivado de SQLite y botón iniciar/pausar/reanudar.
- Selector de tareas 320×440 con búsqueda sin acentos, agrupación por proyecto, teclado
  (flechas/Enter/Esc), alta local y botón "Guardar y cambiar".
- Persistencia de entradas (`running`/`paused`/`closed`) y cambio de tarea transaccional.
- Recuperación tras cierre inesperado usando el último heartbeat (umbral 120 s) y heartbeat cada 60 s.
- Totales diarios por proyecto/tarea (comando `get_today_totals`) y workspace `General` por defecto.
- E2E real verificado en release: arrastre, pausa con clic, selector y alta local con entrada asociada.

## Arranque rápido

```powershell
npm install
npm run dev            # Tauri + Vite en modo desarrollo
npm run check          # lint + typecheck + tests
npm run build:nobundle # compila el .exe sin instalador
npm run build          # compila y empaqueta (NSIS)
```

La base de datos se crea automáticamente en:
`%LOCALAPPDATA%\com.sipecom.devtime\devtime.db`

## Requisitos de desarrollo

- Node.js 24 y npm 11
- Rust stable (`rustup`) con MSVC
- Visual Studio 2017+ con C++ Build Tools y Windows SDK 10
- WebView2 Runtime (presente en Windows 10/11 actualizado)

## Estructura

| Ruta | Contenido |
|------|-----------|
| `apps/desktop` | UI React/TypeScript + shell Tauri |
| `crates/devtime-domain` | Entidades y reglas de tiempo |
| `crates/devtime-application` | Casos de uso y puertos |
| `crates/devtime-infrastructure` | SQLite, reloj del sistema |
| `db/migrations` | DDL canónico embebido en el binario |
| `docs` | Documentación del proyecto |

## Documentación

- Índice y propósito de cada documento: [docs/README.md](docs/README.md)
- Propuesta a discutir: [docs/rfd/RFD-0001-devtime.md](docs/rfd/RFD-0001-devtime.md)
- Plan por fases: [docs/plan/PLAN-DESARROLLO.md](docs/plan/PLAN-DESARROLLO.md)
- Arquitectura: [docs/arquitectura/ARQUITECTURA.md](docs/arquitectura/ARQUITECTURA.md)
- Bitácora de avance: [docs/bitacora/BITACORA.md](docs/bitacora/BITACORA.md)

## Stack

| Capa | Tecnología |
|------|------------|
| Shell de escritorio | Tauri 2 (WebView2, ventana nativa Rust) |
| UI | React 19 + TypeScript + Vite (CSS Modules, Zustand) |
| Dominio y casos de uso | Rust (crates hexagonales) |
| Persistencia local | SQLite (WAL) vía `rusqlite` |
| Integraciones futuras | Microsoft Entra ID, Azure DevOps, REST/PWA Timesheet |
