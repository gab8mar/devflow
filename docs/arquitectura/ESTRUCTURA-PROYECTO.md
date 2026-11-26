# Estructura del proyecto

## Árbol objetivo

```
devflow/
├─ apps/
│  └─ desktop/
│     ├─ src/                          # React + TypeScript (solo presentación)
│     │  ├─ app/                       # composición, providers, bootstrap
│     │  │  ├─ App.tsx
│     │  │  ├─ providers.tsx
│     │  │  └─ windows/                # entrypoints por ventana (timer | settings | summary)
│     │  ├─ features/
│     │  │  ├─ timer/                  # cronómetro compacto y hover
│     │  │  ├─ task-switcher/          # proyecto → tarea → guardar y cambiar
│     │  │  ├─ daily-summary/          # resumen del día
│     │  │  ├─ settings/               # apariencia, comportamiento, productividad
│     │  │  └─ context-alert/          # sugerencia no invasiva (v2.0)
│     │  ├─ components/                # primitivas: Button, Icon, Tooltip, Popover
│     │  ├─ lib/
│     │  │  ├─ ipc/                    # bindings tipados + validación
│     │  │  ├─ format/                 # duración, fechas, moneda de tiempo
│     │  │  └─ theme/                  # tokens, detección de tema, contraste
│     │  └─ styles/                    # tokens.css, reset.css, globals.css
│     ├─ src-tauri/
│     │  ├─ src/
│     │  │  ├─ main.rs                 # bootstrap mínimo
│     │  │  ├─ lib.rs                  # builder Tauri, plugins, setup
│     │  │  ├─ commands/               # adaptadores IPC finos (1 archivo por área)
│     │  │  ├─ composition/            # wiring de repos, use cases, settings
│     │  │  ├─ windowing/              # tamaños, posición, clamp, drag, DPI
│     │  │  ├─ idle/                   # GetLastInputInfo, fullscreen detection
│     │  │  ├─ tray/                   # bandeja, single instance, autostart
│     │  │  └─ state.rs                # AppState (Arc<AppServices>)
│     │  ├─ capabilities/              # permisos Tauri mínimos
│     │  ├─ icons/
│     │  ├─ tauri.conf.json
│     │  └─ Cargo.toml
│     ├─ index.html
│     ├─ vite.config.ts
│     ├─ tsconfig.json
│     └─ package.json
├─ crates/
│  ├─ devtime-domain/
│  │  └─ src/
│  │     ├─ entities/                  # project.rs, task.rs, time_entry.rs, activity.rs
│  │     ├─ value_objects/             # duration.rs, time_range.rs, local_day.rs
│  │     ├─ services/                  # timer_state_machine.rs, accounting.rs
│  │     └─ errors.rs
│  ├─ devtime-application/
│  │  └─ src/
│  │     ├─ use_cases/                 # start, switch, pause, resume, close, recover, summary
│  │     ├─ ports/                     # repositories, clock, idle_sensor, gateways, secrets
│  │     ├─ dto.rs
│  │     └─ errors.rs
│  └─ devtime-infrastructure/
│     └─ src/
│        ├─ sqlite/                    # conexión, migrador, repos, queries
│        ├─ windows/                   # idle, ventana, credential manager
│        ├─ http/                      # client, retry, telemetry-free logging
│        ├─ identity/                  # entra_pkce.rs
│        └─ gateways/                  # ado, pwa_timesheet, mock
├─ packages/
│  └─ shared-types/                    # tipos TS generados desde Rust + esquemas
├─ db/
│  └─ migrations/
│     └─ 0001_init.sql                 # DDL canónico de referencia
├─ docs/                               # ver docs/README.md
├─ tests/
│  ├─ e2e/                             # Playwright (UI) + tauri-driver (nativo, nightly)
│  ├─ performance/                     # scripts de CPU/RAM/arranque 8 h
│  └─ fixtures/
├─ scripts/
│  ├─ dev.ps1
│  ├─ perf-sample.ps1
│  └─ release.ps1
├─ .editorconfig
├─ .gitignore
├─ Cargo.toml                          # workspace
├─ rust-toolchain.toml                 # toolchain fijado (stable + fmt + clippy)
├─ package.json                        # scripts raíz (npm workspaces)
├─ azure-pipelines.yml
└─ README.md
```

## Estado real (2026-11-25)

El árbol anterior es el **objetivo**. Lo implementado en F1–F2 es:

```
devflow/
├─ apps/desktop/
│  ├─ src/
│  │  ├─ main.tsx
│  │  ├─ app/App.tsx
│  │  ├─ features/timer/{TimerPill.tsx,TaskPicker.tsx,taskFilter.ts,taskFilter.test.ts,useTimer.ts,useTasks.ts,timer.css}
│  │  ├─ lib/format/{duration.ts,duration.test.ts}
│  │  ├─ lib/ipc/{commands.ts,errors.ts}
│  │  ├─ lib/timer/{elapsed.ts,elapsed.test.ts}
│  │  └─ styles/{tokens.css,globals.css}
│  └─ src-tauri/
│     ├─ src/
│     │  ├─ main.rs, lib.rs, state.rs
│     │  └─ commands/{mod.rs,health.rs,preferences.rs,tasks.rs,timer.rs}
│     └─ capabilities/default.json
├─ crates/
│  ├─ devtime-domain/src/{lib.rs,time_entry.rs}
│  ├─ devtime-application/src/{lib.rs,errors.rs,ports.rs}
│  │  └─ use_cases/{mod.rs,preferences.rs,tasks.rs,timer.rs}
│  └─ devtime-infrastructure/src/{lib.rs,system.rs}
│     └─ sqlite/{mod.rs,database.rs,time_entries.rs,tasks.rs}
├─ packages/shared-types/src/index.ts
├─ db/migrations/0001_init.sql
├─ docs/, README.md, Cargo.toml, package.json, azure-pipelines.yml
```

Aún **no existen** `entities/`, `value_objects/`, `services/`, `composition/`, `windowing/`,
`idle/`, `tray/`, `tests/e2e/` ni `scripts/`; se crearán al implementar F3–F5 y la cobertura.
El único entrypoint de ventana activo es `timer`; `settings` y `summary` son objetivos.

## Responsabilidad de cada carpeta

| Ruta | Qué contiene | Qué NO debe contener |
|------|--------------|----------------------|
| `apps/desktop/src` | Componentes, hooks, estilos, llamadas IPC | Reglas de tiempo, SQL, acceso a SO |
| `apps/desktop/src-tauri` | Composición Tauri, comandos, ventanas, tray | Lógica de dominio |
| `crates/devtime-domain` | Entidades, VOs, máquina de estados, totales | IO, Tauri, SQLite, HTTP |
| `crates/devtime-application` | Casos de uso y puertos | Implementaciones concretas |
| `crates/devtime-infrastructure` | Adaptadores: SQLite, Windows, HTTP, PKCE | Componentes de UI |
| `packages/shared-types` | Contrato IPC tipado y validadores | Lógica de negocio |
| `db/migrations` | DDL canónico; `devtime-infrastructure` lo embebe con `include_str!` y lo aplica al arrancar | Datos de usuario |
| `tests/e2e` | Flujos de usuario y humo de ventana | Pruebas unitarias |
| `scripts` | Automatización local y CI | Secretos |

## Convenciones

- **Rust:** `snake_case` módulos/funciones, `PascalCase` tipos, errores `thiserror`, sin `unwrap`
  en rutas de producción (`expect` solo en setup con mensaje).
- **TypeScript:** componentes `PascalCase.tsx`, hooks `useX.ts`, utilidades `camelCase.ts`;
  `strict: true`; sin `any` en el contrato IPC.
- **SQL:** tablas `snake_case` plural; columnas en `snake_case`; timestamps `*_at` en epoch ms.
- **IDs:** UUID v7 en texto (ordenable por tiempo y seguro para sync).
- **Comandos IPC:** verbo explícito y consistente con el caso de uso (`start_task`,
  `pause_current`, `get_today_totals`) y DTO versionado (`v1` implícito en el nombre del comando
  cuando rompa contrato).
- **Eventos Tauri → UI:** `app://state-changed`, `app://idle-changed`, `app://sync-status`;
  payload pequeño y tipado.
- **Nombres de ventanas:** `timer`, `settings`, `summary`.

## Contrato IPC

```ts
type EntryStatus = "running" | "paused" | "closed";

interface ActiveEntryDto {
  id: string;
  projectId: string;
  projectName: string;
  projectCode: string;
  projectColor: string;
  taskId: string;
  taskTitle: string;
  status: EntryStatus;
  startedAt: number;        // epoch ms UTC
  lastResumedAt: number | null;
  accumulatedSeconds: number;
  elapsedSeconds: number;   // calculado por el backend al responder
}

interface TimerSnapshotDto {
  active: ActiveEntryDto | null;
  idleState: "active" | "paused" | "idle";
  serverNowMs: number;
}

interface DailyTotalDto {
  projectId: string;
  projectName: string;
  projectCode: string;
  taskId: string;
  taskTitle: string;
  totalSeconds: number;
  entryCount: number;
}

interface TaskDto {
  id: string;
  projectId: string;
  projectName: string;
  projectCode: string;
  projectColor: string;
  title: string;
  externalId: string | null;
  lastUsedAt: number | null;
  isLocal: boolean;
}
```

Comandos actuales: `health_check`, `get_preference`, `set_preference`, `get_timer_snapshot`,
`start_task`, `pause_current`, `resume_current`, `close_current`, `get_today_totals`,
`list_tasks`, `create_task`.

Los DTO viven hoy en `packages/shared-types/src/index.ts` y en `apps/desktop/src/lib/ipc/commands.ts`.
La generación automática con `tauri-specta` y la validación con Zod están previstas, aún no implementadas.

## Configuración clave

| Archivo | Propósito |
|---------|-----------|
| `tauri.conf.json` | Ventana sin decoración, transparente, always-on-top, CSP estricta, updater, bundle |
| `Cargo.toml` (workspace) | Perfiles `release` con LTO, `panic = "abort"`, `strip` |
| `vite.config.ts` | Build por ventana, target WebView2, sourcemaps solo en dev |
| `package.json` | Scripts `dev`, `build`, `test`, `lint`, `tauri` |
| `azure-pipelines.yml` | Lint, test, build, empaquetado y firma en `windows-2022` |
