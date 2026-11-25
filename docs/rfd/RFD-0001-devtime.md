# RFD-0001 — DevTime: registro pasivo de tiempo para desarrolladores

| Campo | Valor |
|-------|-------|
| Estado | **Propuesta (Proposed)** |
| Fecha | 2026-11-23 |
| Autores | Arquitectura / Product / UX |
| Decisores | Tech Lead, Product Owner, Seguridad, Infraestructura |
| Relacionados | [Plan de desarrollo](../plan/PLAN-DESARROLLO.md), [Arquitectura](../arquitectura/ARQUITECTURA.md) |
| Nombre provisional | DevTime (sujeto a branding, ver Q5) |

> Un **RFD** (Request for Discussion) documenta una propuesta para que el equipo la discuta
> antes de comprometerse. No es un contrato final: vive y se actualiza con las decisiones.

---

## 1. Problema

Los desarrolladores registran sus horas en un Timesheet corporativo (tecnologías Microsoft,
alojado en Azure) **de memoria**, al final del día o de la semana. Consecuencias:

- Se reportan **horas estimadas**, no reales.
- Los reportes de esfuerzo pierden precisión y trazabilidad.
- Se pierde tiempo administrativo reconstruyendo el día.
- Los cambios de contexto (bug → reunión → soporte → feature) no quedan registrados.
- Existe una brecha entre esfuerzo real y esfuerzo reportado.

## 2. Objetivo

Construir una app de escritorio Windows que **acompañe** al desarrollador durante la jornada,
registre el tiempo real por tarea con mínimo esfuerzo y prepare el llenado del Timesheet.

### Objetivos medibles (MVP)

| Objetivo | Métrica |
|----------|---------|
| Bajo consumo | CPU inactiva ≤ 0.5 % de un núcleo; RAM privada total ≤ 150 MB; arranque ≤ 1.5 s |
| No invasiva | 0 popups automáticos; 0 robos de foco no solicitados; huella visual ≤ 132×28 px |
| Precisión | Error de registro ≤ 1 s por transición; pausa por inactividad retrocedida al último input |
| Adopción | Registrar/cambiar tarea en ≤ 2 clics; alta de tarea en ≤ 10 s |
| Continuidad | 0 pérdidas de datos ante cierre inesperado (WAL + recuperación de entrada `running`) |

### No objetivos (explícitos)

- **No** es monitoreo de empleados ni vigilancia de productividad.
- **No** captura contenido de teclado, pantalla, portapapeles ni URLs completas.
- **No** reemplaza al Timesheet: lo alimenta.
- **No** requiere conexión permanente ni telemetría.

## 3. Alcance propuesto

### MVP (Fases 0–5)

Cronómetro flotante, cambio rápido de tarea, registro local automático en SQLite, pausa por
inactividad, resumen diario, ajustes de apariencia/comportamiento, bandeja del sistema,
exportación CSV preparada para el Timesheet.

### Posterior

- **v1.0:** Entra ID (PKCE), importación de work items de Azure DevOps, envío real al Timesheet
  (outbox + idempotencia), instalador firmado y auto-update.
- **v2.0:** Detección inteligente de cambio de contexto (solo sugerencia visual).
- **v3.0:** Reportes empresariales y exportaciones agregadas.

Detalle en [MVP-Y-ROADMAP](../producto/MVP-Y-ROADMAP.md).

## 4. Propuesta técnica

| Componente | Elección |
|------------|----------|
| Shell nativo | **Tauri 2** (ventana y procesos nativos en Rust, WebView2 para UI) |
| UI | **React + TypeScript + Vite**, CSS Modules, Zustand |
| Dominio | Rust en crates hexagonales (`domain`, `application`, `infrastructure`) |
| Persistencia | **SQLite** embebido, WAL, migraciones versionadas |
| Integraciones | Puertos/adaptadores: Entra ID (PKCE + navegador del sistema), Azure DevOps REST, Timesheet REST/PWA |
| Seguridad | Tokens en Windows Credential Manager (DPAPI); DB local; sin telemetría por defecto |
| Distribución | MSI/NSIS por usuario, firma de código, updater firmado, Intune |

### ¿Por qué Tauri y no MAUI?

| Criterio | Tauri 2 + React/Rust | .NET MAUI (WinUI 3) |
|----------|----------------------|---------------------|
| Instalador | ~5–12 MB | ~80–150 MB self-contained |
| RAM privada total | ~100–150 MB (WebView2 incluido; medido 128 MB) | ~150–250 MB |
| Arranque en frío | 1–2 s (por medir en F5) | 2–4 s típico |
| Ventana overlay tiny/transparente | Control nativo directo (`winit` + Win32) | Posible, más fricción en layered/transparency |
| CPU inactiva | Casi nula (sin timer en Rust) | Mayor superficie de runtime/UI |
| Ecosistema UI | Web (flexible, rápido de iterar) | XAML nativo, curva distinta |
| Encaje corporativo Microsoft | Bueno (REST/SQLite/Entra) | Excelente (stack .NET) |
| Riesgo principal | Toolchain Rust y dependencia de WebView2 | Peso y arranque para una app siempre abierta |

Para una ventana diminuta siempre visible durante 8–12 h, **el costo dominante es el runtime**.
Tauri arranca un proceso Rust pequeño y delega el render a WebView2 (ya presente en el SO),
mientras MAUI carga el runtime .NET + WinUI. La diferencia de RAM/arranque es decisiva para el
principio de ligereza. El costo de Rust se acota manteniendo **superficie mínima**: dominio,
persistencia, inactividad y ventana; toda la UI en TypeScript.

**Mitigación si la organización veta Rust:** la arquitectura es hexagonal; un adaptador MAUI
podría reutilizar el dominio conceptual, pero se perderían las ventajas de peso/arranque.
Se documenta como alternativa, no como plan A.

### Alternativas descartadas

| Alternativa | Motivo |
|-------------|--------|
| Electron | 150–250 MB, RAM alta, arranque lento: contradice el principio de ligereza |
| WinUI 3 nativo (C#) | Excelente integración, pero mayor costo de UI/iteración y más peso que Tauri |
| PWA / web only | No puede ser always-on-top global ni detectar inactividad del SO |
| App de bandeja sin ventana | No cumple "visible cuando se necesita" ni el cambio de tarea en 2 clics |

## 5. Riesgos de la propuesta (resumen)

1. Toolchain Rust ausente en el equipo → instalar y estandarizar (bloquea Fase 1).
2. WebView2 no actualizado en flota → validar y empaquetar bootstrapper.
3. Transparencia/always-on-top en pantalla completa/RDP → matriz de pruebas por escenario.
4. Contrato real del Timesheet desconocido → diseñar puerto y mock; confirmar en Q1.
5. Detección de inactividad imprecisa → usar `GetLastInputInfo` y retroceder al último input.

Registro completo en [RIESGOS-Y-ESTIMACIONES](../negocio/RIESGOS-Y-ESTIMACIONES.md).

## 6. Decisiones abiertas (se necesitan para avanzar)

| ID | Pregunta | Impacto | Necesaria antes de |
|----|----------|---------|--------------------|
| Q1 | ¿Cuál es el contrato (endpoint, auth, payload) del Timesheet corporativo? | Alto | v1.0 (F6) |
| Q2 | ¿Tenant/App Registration de Entra ID y scopes aprobados? | Alto | v1.0 (F6) |
| Q3 | ¿Se exige cifrado de la DB local (SQLCipher) desde v1? | Medio | MVP (F2) |
| Q4 | ¿Telemetría/crash reporting opt-in permitido? | Bajo | F5 |
| Q5 | ¿"DevTime" es el nombre final y hay lineamientos de marca? | Bajo | F3 |
| Q6 | ¿Windows 10 22H2 es el mínimo soportado? ¿Hay equipos ARM? | Medio | F1 |
| Q7 | ¿Un usuario puede tener varias instancias (multi-sesión/RDP)? | Bajo | F4 |
| Q8 | ¿Restricción de antivirus/AppLocker para binarios no firmados? | Medio | F5 |

## 7. Criterio de aceptación de esta RFD

- [ ] Product valida alcance MVP y no objetivos.
- [ ] Arquitectura valida stack y riesgos.
- [ ] Seguridad valida modelo de tokens y privacidad.
- [ ] Infraestructura valida distribución (firma, Intune, WebView2).
- [ ] Q1–Q3 respondidas o con fecha comprometida.

## 8. Registro de decisiones

| Fecha | Decisión | Estado |
|-------|----------|--------|
| 2026-11-23 | Adoptar Tauri 2 + React/TS + Rust + SQLite como stack del MVP | Propuesta |
| 2026-11-23 | Arquitectura hexagonal con puertos/adaptadores y outbox para sync | Propuesta |
| 2026-11-23 | Sin telemetría por defecto; tokens en Credential Manager | Propuesta |
