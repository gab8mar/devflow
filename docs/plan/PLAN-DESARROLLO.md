# Plan de desarrollo DevTime

Documento maestro de fases. Cada fase tiene objetivo, entregables, criterios de salida y
estimación. Una fase no se cierra si no cumple sus criterios de salida.

**Supuestos de capacidad:** 1 desarrollador senior full-time + QA/UX parcial (0.5). Las
estimaciones son días-persona e incluyen pruebas y documentación de la fase.

## Tablero de fases

| Fase | Nombre | Objetivo | Est. | Depende de | Estado |
|------|--------|----------|------|------------|--------|
| F0 | Descubrimiento y diseño | RFD, arquitectura, UX, datos, plan | 3–4 d | — | **Completada** |
| F1 | Scaffolding | Monorepo Tauri+React+Rust+SQLite compilando | 3–4 d | F0, Rust instalado | **Completada** (CI pendiente de 1.ª ejecución) |
| F2 | Dominio y persistencia | Cronómetro y repositorios probados | 4–5 d | F1 | **En curso** (cronómetro, totales y catálogo de tareas; pendiente CRUD completo, historial y cobertura) |
| F3 | UI flotante | Ventana compacta, hover, switcher, temas | 5–6 d | F2 | **En curso** (arrastre y selector con alta local; pendiente búsqueda, proyectos y temas) |
| F4 | Productividad | Inactividad, resumen diario, tray, autostart | 4–5 d | F3 | Pendiente |
| F5 | Hardening y piloto | Empaquetado, firma, updater, piloto interno | 4–5 d | F4 | Pendiente |
| F6 | v1.0 Timesheet | Entra ID, ADO, sync real con outbox | 10–15 d | F5, Q1/Q2 | Pendiente |
| F7 | v2.0 Contexto | Sugerencia de cambio de tarea no invasiva | 8–10 d | F6 | Pendiente |
| F8 | v3.0 Reportes | Reportes y exportaciones empresariales | 10–15 d | F7 | Pendiente |

**MVP = F0–F5** (~20–24 d-persona, 5–6 semanas calendario con QA/UX parcial).
Camino completo hasta v3.0: ~65 d-persona + 20 % de contingencia.

---

## F0 — Descubrimiento y diseño

**Objetivo:** acordar qué se construye, cómo y con qué límites, antes de escribir producto.

**Entregables**
- RFD-0001 con alcance, alternativas y decisiones abiertas.
- Arquitectura, estructura de carpetas, modelo de datos y DDL.
- Diseño UX/UI con estados e interacciones.
- Estrategias de pruebas, seguridad y despliegue.
- Plan de fases, riesgos y estimaciones.
- Índice de documentación y bitácora.
- Índice CodeGraph y memoria del proyecto en Engram.

**Criterios de salida**
- [x] Documentación creada y enlazada desde el índice.
- [x] Decisiones de stack y arquitectura registradas.
- [ ] RFD revisada por Product/Seguridad/Infra (Q1–Q3 con responsable).
- [x] Toolchain Rust confirmado para F1 (Rust 1.100 + MSVC VS 2017).

## F1 — Scaffolding y esqueleto

**Objetivo:** repositorio que compila y abre una ventana Tauri con SQLite migrado.

**Entregables**
- Workspace Cargo (`crates/domain|application|infrastructure`) + app Tauri.
- React + TS + Vite con tokens de tema y layout mínimo.
- Migraciones SQLite ejecutadas al arrancar (WAL, foreign_keys).
- Comandos IPC de ping/health y errores tipados.
- Lint/format: `cargo fmt`, `cargo clippy -D warnings`, ESLint, `tsc --noEmit`.
- CI inicial (build + tests en `windows-2022`).

**Criterios de salida**
- `npm run tauri dev` abre ventana sin barra de título, always-on-top, 132×28.
- La DB se crea en `%LOCALAPPDATA%\com.sipecom.devtime\devtime.db` con esquema v1.
- CI verde; cero warnings.
- README actualizado con comandos reales.

## F2 — Dominio y persistencia

**Objetivo:** reglas de tiempo correctas e independientes de la UI.

**Entregables**
- Entidades `Project`, `Task`, `TimeEntry`, `ActivityEvent`, `Preferences`.
- Máquina de estados del cronómetro (`idle → running → paused → running → closed`).
- Casos de uso: iniciar, cambiar, pausar, reanudar, finalizar, recuperar tras crash.
- Repositorios SQLite + migraciones + pruebas con DB temporal.
- Cálculo de totales diarios y detección de solapamientos.
- Pruebas de propiedad para aritmética de tiempo (DST, medianoche, reloj ajustado).

**Avance 2026-11-25**
- [x] `TimeEntry` + máquina de estados + `recover_if_stale` (13 pruebas de dominio).
- [x] Casos de uso iniciar/cambiar, pausar, reanudar, finalizar, recuperar, totales y catálogo de tareas (12 pruebas).
- [x] Repositorio SQLite con `switch` transaccional, join proyecto/tarea, totales desde `v_daily_totals` y tareas locales (13 pruebas).
- [x] Comandos IPC y UI conectada: iniciar/pausar/reanudar, selector de tareas y alta local (8 pruebas frontend).
- [x] Recuperación al arranque (latido perdido > 120 s) y heartbeat cada 60 s.
- [ ] CRUD completo de `Project`/`Task` (editar, archivar, proyectos) y `ActivityEvent`.
- [ ] Detección de solapamientos, pruebas de propiedad/concurrencia y cobertura ≥ 80/70 %.

**Criterios de salida**
- Cobertura ≥ 80 % en `domain` y `application`; ≥ 70 % en infraestructura.
- Una sola entrada `running` garantizada por índice parcial + prueba concurrente.
- Recuperación tras cierre forzado sin pérdida > 1 s.

## F3 — UI flotante y cambio rápido

**Objetivo:** experiencia diaria agradable y discreta.

**Entregables**
- Estados: compacto, hover, switcher, minimizado, alerta de contexto (visual).
- Arrastre, posición recordada, clamp a área de trabajo multi-monitor.
- Menú contextual (cambiar, pausar, finalizar, resumen, configuración, salir).
- Switcher proyecto → tarea → "Guardar y cambiar" con búsqueda.
- Temas claro/oscuro, transparencia, color principal, tipografías.
- Accesibilidad: teclado, foco visible, contraste, `prefers-reduced-motion`.

**Avance 2026-11-25**
- [x] Arrastre real corregido: `data-tauri-drag-region` requiere `core:window:allow-start-dragging`;
  verificado moviendo la ventana de `414,2` a `244,186` con entrada nativa.
- [x] Selector de tareas 320×360: lista activa, tarea actual marcada, uso reciente primero,
  etiqueta `Local`/`PWA` y botón `+ Nueva tarea local`.
- [x] Alta local idempotente por título dentro de `Trabajo general`; `external_id` queda `NULL`
  para distinguirla de las tareas que importe el PWA.
- [x] Búsqueda incremental (título, proyecto y código, sin acentos), agrupación por proyecto,
  navegación con flechas/Enter/Esc, doble clic para cambiar y botón "Guardar y cambiar".
- [ ] Hover, menú contextual, posición recordada, temas y atajos.

**Criterios de salida**
- Cambiar de tarea en ≤ 2 clics y ≤ 3 s.
- Ningún popup automático; el foco solo se toma por acción del usuario.
- Presupuesto visual: compacto 132×28, hover 264×92, switcher 320×440.
- Checklist de accesibilidad aprobada.

## F4 — Productividad

**Objetivo:** que el registro sea fiel sin intervención.

**Entregables**
- Inactividad por `GetLastInputInfo` (poll 30 s), configurable 5/10/15 min.
- Pausa retrocedida al último input; reanudación explícita o configurable.
- Resumen diario (por proyecto/tarea, total, tiempo sin asignar).
- Bandeja del sistema, single-instance, autostart opt-in, iniciar minimizado.
- Exportación CSV compatible con plantilla del Timesheet.
- Registro de actividad local con retención de 90 días.

**Criterios de salida**
- Inactividad detectada con error ≤ 30 s respecto del umbral configurado.
- CPU inactiva ≤ 0.5 % y RAM privada total ≤ 150 MB en prueba de 8 h.
- Cierre inesperado recupera la tarea activa.

## F5 — Hardening y piloto

**Objetivo:** instalable confiable y usable por un grupo real.

**Entregables**
- MSI/NSIS por usuario, firma de código, updater firmado en Azure Blob.
- Pipeline de release con SBOM, `cargo audit`/`cargo deny`, `npm audit`.
- Pruebas de rendimiento y estabilidad de 8 h documentadas.
- Optimización de memoria de WebView2 si la línea base (128 MB privada) no baja de 100 MB.
- Onboarding (3 pantallas), guía rápida y texto de privacidad.
- Piloto con 5–10 desarrolladores, instrumentación local opt-in.

**Criterios de salida**
- Instalación sin admin y sin falsos positivos de AV en la flota piloto.
- Cero pérdidas de datos y cero bloqueos reportados en 2 semanas.
- Satisfacción piloto ≥ 4/5 y cambio de tarea percibido como "rápido".
- Go/No-Go documentado para v1.0.

## F6 — v1.0 Integración Timesheet

**Objetivo:** enviar horas reales al sistema corporativo.

**Entregables**
- Entra ID Authorization Code + PKCE con navegador del sistema y refresh rotativo.
- Importación de work items/proyectos desde Azure DevOps REST.
- Adaptador del Timesheet (contrato Q1) + outbox con reintentos y backoff.
- Idempotencia, mapeo de IDs externos, resolución de conflictos y auditoría.
- Estado de sincronización visible en el resumen diario.

**Criterios de salida**
- Envío diario idempotente: repetir no duplica horas.
- Funciona offline y sincroniza al recuperar red sin pérdida.
- Tokens nunca en SQLite ni en logs; rotación verificada.
- Pruebas de contrato contra mock y sandbox real.

## F7 — v2.0 Detección de contexto

**Objetivo:** sugerir cambios de tarea sin invadir.

**Entregables**
- Lectura de ventana en primer plano (título/proceso) y detección de Azure DevOps.
- Reglas configurables proyecto ↔ patrón; puntuación de confianza.
- Sugerencia solo visual (borde ámbar + icono); nunca cambio automático.
- Registro de sugerencias aceptadas/descartadas para calibrar.

**Criterios de salida**
- ≤ 1 sugerencia cada 15 min por defecto; silenciable.
- Precisión de sugerencias aceptadas ≥ 70 % en piloto.
- Sin lectura de contenido, solo metadatos de ventana.

## F8 — v3.0 Reportes

**Objetivo:** valor para el equipo y la organización.

**Entregables**
- Dashboard local (día/semana/proyecto), exportaciones CSV/XLSX.
- Agregados para reportes empresariales y Power BI (sin datos personales innecesarios).
- Comparativa real vs. estimado y detección de desvíos.

**Criterios de salida**
- Reportes reproducibles y auditables; privacidad revisada por Seguridad.
- Rendimiento del dashboard < 500 ms para 12 meses de datos.

---

## Forma de trabajo

- **Trunk-based** con ramas cortas y PR pequeño; Conventional Commits.
- **Definition of Done:** código + pruebas + docs + lint/typecheck + criterio de fase.
- **Registro obligatorio:** cada sesión cierra con entrada en la bitácora y memoria Engram.
- **Revisión de riesgos** quincenal y al cierre de cada fase.
- **Demo** al final de cada fase con el criterio de salida como guion.
