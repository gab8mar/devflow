# Estrategia de pruebas

## 1. Pirámide

| Nivel | Alcance | Herramientas | Objetivo |
|-------|---------|--------------|----------|
| Unitario Rust | Dominio y casos de uso | `cargo test`, `proptest` | Lógica de tiempo correcta |
| Integración Rust | Repos SQLite, migraciones, adaptadores | `tempfile`, `wiremock` | Persistencia e integración reales |
| Unitario UI | Hooks, formateadores, componentes | Vitest, Testing Library | Comportamiento de UI |
| E2E UI | Flujos sobre el bundle web | Playwright (IPC mockeado) | Switcher, resumen, ajustes |
| E2E nativo | Ventana always-on-top, drag, tray, idle | `tauri-driver` + pruebas manuales guiadas | Comportamiento del SO |
| Rendimiento | CPU/RAM/arranque/DB | Scripts PowerShell + ETW | Presupuestos no funcionales |
| Seguridad | Dependencias, CSP, secretos | `cargo audit`, `cargo deny`, `npm audit`, revisión | Sin vulnerabilidades conocidas |

## 2. Cobertura y puertas

- `devtime-domain` y `devtime-application`: **≥ 80 %** de líneas y ramas.
- `devtime-infrastructure`: **≥ 70 %** (excluyendo bindings triviales de Win32).
- Frontend: **≥ 60 %** en hooks y lógica; componentes visuales por comportamiento.
- Puerta de CI: falla si baja del umbral o si hay warnings de lint.
- **Estado 2026-11-25:** umbrales definidos pero aún no medidos; la suite actual es 38 pruebas
  Rust + 8 pruebas frontend (incluye catálogo/alta local y validación de títulos). La cobertura
  se instrumentará con `cargo-llvm-cov` y `@vitest/coverage-v8` (pendiente F2).

## 3. Casos críticos del dominio

1. `start → pause → resume → close` produce duración = suma de tramos activos.
2. Pausa por inactividad retrocede al último input, no al momento de detección.
3. Cambio de tarea cierra la anterior con duración exacta y abre la siguiente.
4. Entrada que cruza medianoche se atribuye al día de inicio (regla MVP).
5. Cambio de hora/DST: la duración es diferencia absoluta de instantes.
6. Reloj del sistema ajustado hacia atrás: no produce duración negativa; se registra evento.
7. Crash simulado: recuperación hasta `last_heartbeat_at`, nunca hasta "ahora".
8. Única entrada `running` bajo intentos concurrentes (índice + transacción).
9. Totales diarios coinciden con la suma de entradas cerradas.
10. Idempotencia: reenviar el mismo lote no duplica horas.

## 4. Pruebas de persistencia

- Migración desde DB vacía y desde la versión anterior con datos semilla.
- `PRAGMA integrity_check` y `foreign_key_check` tras escenarios de estrés.
- Concurrencia: dos lecturas + una escritura; sin `SQLITE_BUSY` no controlado.
- Recuperación tras kill del proceso con WAL pendiente.
- Corrupción simulada: la app no arranca en bucle; ofrece backup/exportación.

## 5. Pruebas de UI y nativas

Playwright (contra Vite con IPC simulado):
- Abrir switcher, buscar, seleccionar y confirmar.
- Estados de color y microcopy por estado.
- Ajustes aplican tokens sin recargar.
- Teclado: `Esc`, `Enter`, flechas, foco visible.

Nativo (tauri-driver, nightly + checklist manual por release):
- Ventana sin decoración, always-on-top, transparente, arrastrable (verificado 2026-11-25 con
  movimiento real de ventana) y selector que redimensiona a 320×360.
- No roba foco al actualizar el tiempo.
- Clamp multi-monitor y DPI 100/125/150/200 %.
- Ocultarse en pantalla completa (juego/presentación).
- Tray: mostrar/ocultar/pausar/salir; single instance.
- Autostart con `--minimized`.

## 6. Protocolo de rendimiento

**Escenario:** 8 h continuas con tarea activa, pausas e inactividad; PC de referencia i5/16 GB
Windows 11, WebView2 actualizado.

| Métrica | Método | Presupuesto |
|---------|--------|-------------|
| CPU promedio inactiva | `Get-Counter '\Process(*)\% Processor Time'` cada 60 s | ≤ 0.5 % de un núcleo (medido 0.42 %) |
| CPU pico (cambio de tarea) | ETW/PerfView | ≤ 5 % durante < 1 s |
| RAM privada total | `Get-Process` (app + WebView2), `PrivateMemorySize64` | ≤ 150 MB (medido ~128 MB); WS no comparable por memoria compartida |
| Arranque | marca en log local (proceso → primer render) | ≤ 1.5 s caliente / 2.5 s frío |
| Escritura DB p95 | instrumentación de repos | ≤ 10 ms |
| Apertura switcher | Playwright trace | ≤ 100 ms |
| Crecimiento DB | 1 año de uso simulado | ≤ 50 MB |
| Batería | comparación de descarga en laptop | sin impacto perceptible |

Automatización: `scripts/perf-sample.ps1` recolecta y compara contra umbrales; corre en nightly
y en release candidate. Resultados se archivan en `reports/perf/`.

## 7. Pipeline CI (Azure Pipelines, `windows-2022`)

El pipeline real está en [`azure-pipelines.yml`](../../azure-pipelines.yml) y hoy ejecuta:
`cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --workspace`, `npm run lint`,
`npm run typecheck`, `npm run test`, `npm audit` y `npm run build:nobundle`.
El bloque siguiente es el **objetivo**; cobertura, `cargo deny`, empaquetado, firma y E2E aún no
están conectados.

```yaml
stages:
  - stage: Validate
    jobs:
      - job: LintAndTest
        steps:
          - npm ci
          - cargo fmt --all -- --check
          - cargo clippy --all-targets -- -D warnings
          - cargo test --workspace
          - npm run lint
          - npm run typecheck
          - npm run test -- --coverage
  - stage: Package
    dependsOn: Validate
    jobs:
      - job: Build
        steps:
          - npm run tauri build
          - cargo audit
          - cargo deny check
          - npm audit --audit-level=high
          - publish: artifacts (msi, nsis, sbom)
  - stage: Smoke
    dependsOn: Package
    jobs:
      - job: E2E
        steps:
          - npm run test:e2e
          - run perf-sample.ps1 (smoke 30 min)
```

## 8. Datos de prueba

- Fixtures de proyectos/tareas con nombres largos, acentos y emojis (validar truncado).
- Entradas sintéticas: 12 meses, 20 tareas/día, para rendimiento y reportes.
- Escenarios de zona horaria: UTC−5, UTC+1, DST activo/inactivo.
- Nunca datos reales de empleados en repositorios ni capturas.

## 9. Manejo de flakiness

- E2E con esperas por condición, no por tiempo; reintentos máximos 2 con cuarentena.
- Pruebas de tiempo con `Clock` inyectable (nunca `sleep` para lógica).
- Toda prueba que falle intermitentemente se etiqueta, se investiga y no se ignora.

## 10. Checklist de release

- [ ] Suite completa verde y cobertura sobre umbral.
- [ ] Rendimiento dentro de presupuesto (reporte adjunto).
- [ ] Pruebas manuales nativas (matriz DPI/monitor/fullscreen).
- [ ] `cargo audit`, `cargo deny`, `npm audit` sin hallazgos altos.
- [ ] Instalador firmado y auto-update probado desde la versión anterior.
- [ ] Backup/restauración y recuperación tras crash verificados.

## 11. Estado actual de la suite (2026-11-25)

| Área | Pruebas | Qué cubre |
|------|---------|-----------|
| Dominio | 13 | Estados, pausa/reanudación, cierre, `pause_at`, reloj hacia atrás, heartbeat, `recover_if_stale`, roundtrip de estados |
| Aplicación | 9 | Preferencias y casos de uso del timer con repositorio falso: start/switch, pause, resume, close, recuperación, errores |
| Infraestructura | 9 | Esquema/migración, preferencias, `switch` transaccional, persistencia de pausa/resume, totales diarios, recuperación, workspace idempotente |
| Frontend | 8 | `formatDuration` y cálculo local de tiempo transcurrido (running, paused, reloj inválido) |
| E2E manual automatizado | 3 ejecuciones | Clic real sobre la píldora en debug y release: inicia, pausa y conserva el tiempo; limpieza de datos de prueba |

**Comandos verificados**
```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check
npm run build:web
npm run build:nobundle
```

**Brechas conocidas**
- Sin cobertura instrumentada ni prueba de concurrencia del índice `ux_time_entries_running`.
- Sin pruebas de propiedad (DST, medianoche) ni E2E nativo con `tauri-driver`.
- El E2E de clic es un script externo de humo, no parte aún de `tests/e2e`.
