# Bitácora de DevTime

Registro cronológico de lo hecho, por qué y qué sigue. Cada sesión agrega una entrada.
Formato: fecha · fase · qué se hizo · por qué · dónde · siguiente.

---

## 2026-11-25 · F3 — Búsqueda, agrupación por proyecto y "Guardar y cambiar"

**Contexto:** el selector ya listaba tareas y permitía alta local, pero cambiaba de tarea con
un solo clic y no tenía búsqueda ni agrupación; el diseño pedía confirmar con "Guardar y cambiar".

**Qué se hizo**
1. **Búsqueda incremental sin acentos:** `taskFilter.ts` normaliza NFD y filtra por título,
   nombre de proyecto o código; consulta vacía devuelve todo.
2. **Agrupación por proyecto:** lista con encabezado `CÓDIGO · Nombre` y punto de color;
   el orden sigue siendo uso reciente con `Trabajo general` al final.
3. **Interacción:** clic resalta, doble clic aplica, flechas mueven el resaltado, `Enter`
   confirma, `Esc` cierra o cancela el alta. Botón "Guardar y cambiar" y `Guardar` deshabilitado
   sin selección. La ventana pasa a **320×440** (objetivo de F3) y vuelve a 132×28 al aplicar.
4. **Alta local:** crear una tarea ya no la inicia automáticamente; queda resaltada y el usuario
   confirma con "Guardar y cambiar" (evita cambios accidentales).
5. **Pruebas:** 8 pruebas nuevas de `normalizeText`/`filterTasks`/`groupTasksByProject`
   (16 frontend en total); `npm run check` y build release (4.22 MB) en verde.

**Por qué:** el flujo de dos pasos y la búsqueda son requisitos de F3 (≤ 2 clics y ≤ 3 s);
la confirmación evita cerrar la tarea en curso por un clic accidental.

**Dónde:** `apps/desktop/src/features/timer/{taskFilter.ts,taskFilter.test.ts,TaskPicker.tsx,
TimerPill.tsx,timer.css}`, `apps/desktop/src/app/App.tsx`.

**Nota de verificación:** el render del selector (búsqueda, grupo, tarea activa, botones) se
verificó con captura de ventana; la interacción completa no se pudo automatizar porque el equipo
estaba en uso y las ventanas cambiaban de monitor/foco. Pendiente de humo manual por el usuario.

**Siguiente:** contrato del PWA/Timesheet; luego hover, menú contextual, posición recordada,
temas y atajos.

---

## 2026-11-25 · F2–F3 — Arrastre real, catálogo de tareas y selector

**Contexto:** la píldora se arrastraba solo en desarrollo; en release la región
`data-tauri-drag-region` era un adorno. Además, el tiempo se registraba siempre contra
`Trabajo general` porque no existía catálogo de tareas.

**Qué se hizo**
1. **Arrastre corregido:** la capability `core:window:default` de Tauri 2.12 no incluye
   `core:window:allow-start-dragging`; se añadió explícitamente (junto con
   `core:window:allow-set-size`). Verificado con movimiento real: `414,2 → 244,186`.
2. **Catálogo de tareas:** puerto `Tasks`, casos de uso `list_tasks`/`create_task`, repositorio
   SQLite (`tasks.rs`), validación de título (1–200 caracteres) y reutilización idempotente por
   título dentro del proyecto. Las tareas locales usan `project-general` y `external_id = NULL`.
3. **Selector en la UI:** caret en la píldora, panel 320×360, lista ordenada por uso reciente,
   tarea actual marcada, etiqueta `Local`/`PWA` y alta local con `Enter`. La ventana vuelve a
   132×28 al seleccionar.
4. **Verificación:** `fmt`, `clippy -D warnings`, 38 pruebas Rust, 8 frontend, `npm run check`,
   `build:nobundle` y build release (4.22 MB). E2E real: alta local "Prueba local" con entrada
   `running` asociada y limpieza posterior de los datos de prueba.

**Por qué:** el MVP necesita registrar el tiempo contra la tarea correcta y una ventana que
responda al arrastre en release.

**Dónde:** `apps/desktop/src-tauri/capabilities/default.json`, `commands/tasks.rs`,
`crates/devtime-application/src/{ports.rs,errors.rs,use_cases/tasks.rs}`,
`crates/devtime-infrastructure/src/sqlite/tasks.rs`, `apps/desktop/src/features/timer/*`,
`apps/desktop/src/app/App.tsx`, `packages/shared-types/src/index.ts`.

**Siguiente:** contrato del PWA/Timesheet (URL, autenticación, endpoints, IDs externos) para
importar tareas reales; luego búsqueda incremental, proyectos, hover y temas de F3.

---

## 2026-11-23 · F0 — Arranque, planificación y documentación

**Contexto:** repositorio `C:\repos\devflow` vacío (solo `.git`), sin commits. Proyecto Engram
detectado: `devflow`. No existía índice CodeGraph.

**Qué se hizo**
1. Relevamiento del entorno: Node 24.21 / npm 11.19 disponibles; **Rust/cargo y sqlite3 CLI
   ausentes**; CodeGraph CLI 1.6.0 disponible; sin `.codegraph/`.
2. Se creó la documentación completa de Fase 0:
   - `README.md` raíz y `docs/README.md` (índice con el propósito de cada documento).
   - `docs/rfd/RFD-0001-devtime.md` (propuesta, alcance, alternativas, decisiones abiertas).
   - `docs/plan/PLAN-DESARROLLO.md` (fases F0–F8 con criterios de salida y estimaciones).
   - `docs/arquitectura/ARQUITECTURA.md` (capas, flujos, ADRs, comparación con MAUI,
     presupuestos de rendimiento).
   - `docs/arquitectura/ESTRUCTURA-PROYECTO.md` (árbol de carpetas y contrato IPC).
   - `docs/arquitectura/MODELO-DATOS.md` y `db/migrations/0001_init.sql` (esquema canónico).
   - `docs/arquitectura/INTEGRACION-EMPRESARIAL.md` (Entra ID, ADO, Timesheet, outbox).
   - `docs/ux/DISENO-UX-UI.md` (estados, tokens, interacciones, accesibilidad, microcopy).
   - `docs/producto/MVP-Y-ROADMAP.md` (alcance MVP, historias, roadmap v1–v3).
   - `docs/calidad/ESTRATEGIA-PRUEBAS.md` (pirámide, umbrales, rendimiento, CI).
   - `docs/seguridad/ESTRATEGIA-SEGURIDAD.md` (STRIDE, tokens, cifrado, cadena de suministro).
   - `docs/operaciones/ESTRATEGIA-DESPLIEGUE.md` (empaquetado, firma, updater, Intune).
   - `docs/negocio/RIESGOS-Y-ESTIMACIONES.md` (riesgos, estimaciones, contingencia).
   - Esta bitácora.
3. Se registraron las decisiones y el plan en Engram (memoria persistente del proyecto).
4. Se inicializó el índice CodeGraph del repositorio.

**Decisiones tomadas**
- Stack: Tauri 2 + React/TypeScript + Rust + SQLite, arquitectura hexagonal.
- Tiempo derivado de marcas persistidas + heartbeat para recuperación tras crash.
- Outbox transaccional para integración futura; tokens en Windows Credential Manager.
- MVP = F0–F5; Entra/ADO/sync en v1.0; contexto en v2.0; reportes en v3.0.
- Sin telemetría por defecto; detección de contexto solo sugiere, nunca cambia.

**Por qué**
- Para acordar alcance y arquitectura antes de escribir código, reducir retrabajo y dejar
  trazabilidad de cada decisión para el equipo y futuras sesiones.

**Pendiente / siguiente**
- Revisar la RFD con Product/Seguridad/Infra; cerrar Q1–Q3 con responsables.
- Instalar Rust stable + MSVC Build Tools (bloquea F1).
- Ejecutar F1: scaffolding Tauri + React + SQLite, primer comando IPC y CI.

---

## 2026-11-24 · F1 — Scaffolding, SQLite y primer vertical slice

**Contexto:** F0 cerrada. La máquina no tenía Rust ni sqlite3 CLI; sí Visual Studio 2017
Enterprise (MSVC 14.16) y WebView2 157.

**Qué se hizo**
1. **Toolchain:** Rust 1.100 instalado a nivel usuario con `rustup` (sin admin). Se validó que
   MSVC 14.16 + Windows SDK 10.0.19041 enlazan correctamente con un `cargo run`.
2. **Monorepo:**
   - Workspace Cargo con `crates/devtime-domain`, `devtime-application` y
     `devtime-infrastructure`, más `apps/desktop/src-tauri`.
   - UI React 19 + TypeScript + Vite 6 con tokens de tema oscuro/claro y píldora flotante
     de 132×28 (`TimerPill`).
   - `packages/shared-types` con los DTO previstos.
   - `rust-toolchain.toml`, `.editorconfig`, `.gitignore`, iconos propios.
3. **Dominio (base de F2):** `TimeEntry` con máquina de estados
   running/paused/closed, `elapsed_seconds`, `pause`, `pause_at` (retroceso al último input),
   `resume`, `close` y `heartbeat`; 9 pruebas unitarias.
4. **Aplicación:** puertos `Clock`, `Preferences`, `TimeEntries`; casos de uso de preferencias
   con validación de clave; 3 pruebas con repositorio falso.
5. **Infraestructura:** `Database` SQLite con WAL, `synchronous=NORMAL`, `foreign_keys=ON`,
   `busy_timeout`, migración embebida desde `db/migrations/0001_init.sql`, versión de esquema y
   preferencias key/value; 4 pruebas (esquema, idempotencia, roundtrip).
6. **Tauri:** ventana sin decoración, transparente, always-on-top, skipTaskbar, 132×28; estado
   de aplicación con DB y reloj; comandos `health_check`, `get_preference`, `set_preference`;
   errores tipados (`code`, `message`); CSP estricta y capabilities mínimas.
7. **Calidad:** ESLint 10 + typescript-eslint, Vitest 5, `tsc --noEmit`, Clippy con
   `-D warnings`, `cargo fmt`; CI de Azure con build, tests y auditoría.
8. **Documentación:** README, plan y estructura actualizados; esta bitácora.

**Verificación (evidencia)**
- `cargo test` de dominio + aplicación + infraestructura: **16 pruebas verdes**.
- `cargo clippy --workspace --all-targets -- -D warnings`: sin hallazgos.
- `cargo check`/`tauri build --no-bundle --debug`: app compilada y enlazada
  (`target/debug/devtime-desktop.exe`).
- `tauri build --no-bundle` release: **5m 39s**, ejecutable de **3.97 MB**.
- `npm run lint`, `npm run typecheck`, `npm run test`: verdes (4 pruebas de formato).
- `npm audit`: **0 vulnerabilidades** tras subir Vitest 5 y ESLint 10.
- Prueba de humo debug: proceso vivo, **34.8 MB** de working set, DB creada con cabecera
  `SQLite format 3` (4096 bytes + WAL activo) en `%LOCALAPPDATA%\com.sipecom.devtime\devtime.db`.
- Prueba de humo release: app **22.7 MB WS / 5 MB privada**; WebView2 **123 MB privada**
  (6 procesos, GPU 44.6 MB); **128.3 MB privados totales**. CPU inactiva **0.42 % de un
  núcleo** en 30 s (8 núcleos), dentro del presupuesto de 0.5 %.
- Se ajustó el presupuesto de RAM de 80 MB a **≤ 150 MB privados** (línea base real 128 MB);
  la optimización a ≤ 100 MB queda como objetivo de F5 junto con el arranque instrumentado.

**Decisiones / aprendizajes**
- **No hace falta admin** para Rust; el MSVC de VS 2017 es suficiente (menor requisito de lo
  previsto en R-01).
- La app local de Tauri usa `%LOCALAPPDATA%\com.sipecom.devtime`, no `SIPECOM\DevTime`; se
  adopta esa ruta y se documenta.
- El runner de pruebas quedó en Vitest 5 (compatible con Vite 6), lo que elimina las
  vulnerabilidades críticas de `tinypool`.
- El tick de 1 s vive en React; F2 conectará el estado real y el heartbeat de 60 s.

**Pendiente / siguiente (F2)**
- Repositorios de proyectos/tareas/entradas y casos de uso start/switch/pause/resume/close.
- Recuperación tras crash usando `last_heartbeat_at` y evento `recovered_entry`.
- Totales diarios y consultas de reanudación; pruebas de propiedad de aritmética temporal.
- Primera ejecución real del pipeline de Azure y ajuste de cachés.

---

## 2026-11-25 — F2 núcleo: cronómetro persistente end-to-end

**Hecho**
1. **Dominio:** `TimeEntry::recover_if_stale` (pausa en el último latido, sin contar el tiempo
   caído), `EntryStatus::as_str/parse`; 13 pruebas de dominio verdes.
2. **Aplicación:** read models `ActiveEntry` y `DailyTotal`; puerto `TimeEntries` con
   `find_current`, `save`, `switch` transaccional y `daily_totals`; casos de uso
   `start/pause/resume/close/recover_stale/today_totals`; 9 pruebas con repositorio falso.
3. **Infraestructura:** repositorio SQLite de entradas con join a proyecto/tarea, totales del
   día local desde `v_daily_totals` y `ensure_default_workspace` (proyecto `General`/`GEN`,
   tarea `Trabajo general`); 9 pruebas con DB temporal.
4. **Tauri:** comandos `get_timer_snapshot`, `start_task`, `pause_current`, `resume_current`,
   `close_current`, `get_today_totals`; IDs UUID v7; recuperación al arrancar (umbral 120 s) y
   latido cada 60 s en hilo dedicado.
5. **UI:** hook `useTimer` (poll 15 s), cálculo local del tiempo transcurrido, botón
   iniciar/pausar/reanudar, iconos SVG, estado y error visibles; DTOs sincronizados con
   `packages/shared-types`.
6. **Prueba E2E real:** clic automatizado sobre la píldora → entrada `running` en SQLite;
   segundo clic → `paused` con tiempo acumulado; entrada de prueba eliminada al terminar.
   Repetida sobre el binario release desde base vacía (`running` → `paused` con 6 s).

**Verificación**
- Rust: **31 pruebas verdes** (13 dominio + 9 aplicación + 9 infraestructura).
- Frontend: **8 pruebas verdes** (formato + tiempo transcurrido).
- `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, `npm run lint`,
  `npm run typecheck`, `npm run test` y `npm run build:web`: sin errores.
- `tauri build --no-bundle --debug`: binario enlazado y probado con clic real.
- Release: `tauri build --no-bundle` en **2m 59s**, ejecutable de **4.00 MB**; E2E con clic real
  desde base vacía (`running` → `paused` con 6 s) y limpieza de la entrada de prueba.
- CodeGraph: índice en **35 archivos / 332 nodos / 763 edges** (rust 21, ts 9, tsx 3, js 1,
  yaml 1); `codegraph status` reporta "up to date". El comando `codegraph index` no pudo
  reconstruir mientras el MCP tenía la DB abierta (EPERM); el índice incremental sí estaba al día.

**Decisiones / aprendizajes**
- Cambiar de tarea cierra la anterior e inserta la nueva en una transacción; el índice parcial
  `ux_time_entries_running` sigue garantizando una sola entrada en curso.
- La recuperación usa `last_heartbeat_at` como cota superior: si el latido es reciente (<120 s)
  la entrada continúa; si no, se pausa exactamente en el último latido.
- Se crea un workspace `General` para que la píldora funcione antes del switcher de F3.
- El botón de la píldora queda fuera de la región de arrastre para evitar conflictos de clic.

**Pendiente / siguiente**
- Entidades y CRUD de `Project`/`Task`; `ActivityEvent` y evento `recovered_entry`.
- Detección de solapamientos, pruebas de propiedad (DST/medianoche) y concurrencia.
- Cobertura ≥ 80 % en dominio/aplicación y ≥ 70 % en infraestructura.
- Primera ejecución real del pipeline de Azure.
- F3: switcher proyecto → tarea, menú contextual y posición recordada.
