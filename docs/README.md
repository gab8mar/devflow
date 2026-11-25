# Documentación de DevTime

Esta carpeta es la **fuente de verdad** del proyecto. Cada documento tiene un propósito
único; si algo cambia en el producto, se actualiza el documento correspondiente y se deja
constancia en la [bitácora](bitacora/BITACORA.md).

## Estado actual (2026-11-25)

- **F1 cerrada** (CI escrita, pendiente de primera ejecución en Azure): Tauri 2.12 + React 19 +
  Rust + SQLite WAL, ventana 132×28 arrastrable, build release de 4.22 MB.
- **F2 núcleo implementado:** cronómetro persistente con iniciar/cambiar, pausar, reanudar,
  finalizar, recuperación por heartbeat y totales diarios; catálogo de tareas con alta local y
  selector conectado; UI y E2E con entrada real.
- **Verificación:** 38 pruebas Rust + 8 frontend, `fmt`/`clippy`/`lint`/`typecheck`/build en verde.
- **Pendiente de F2:** integración con el PWA (contrato Q1), CRUD completo de proyectos/tareas,
  `ActivityEvent`, solapamientos, cobertura y pruebas de propiedad/concurrencia. Después: F3
  (búsqueda, proyectos, hover y temas).
- El detalle cronológico está en la [bitácora](bitacora/BITACORA.md); el tablero de fases, en el
  [plan](plan/PLAN-DESARROLLO.md).

## Mapa de documentos

| Documento | Para qué sirve | Audiencia | Se actualiza cuando |
|-----------|----------------|-----------|---------------------|
| [rfd/RFD-0001-devtime.md](rfd/RFD-0001-devtime.md) | Propuesta formal: problema, alcance, alternativas, decisiones abiertas. Es el acuerdo de "qué queremos construir y por qué". | Product, Arquitectura, Seguridad | Cambia el alcance, el stack o una decisión de fondo |
| [plan/PLAN-DESARROLLO.md](plan/PLAN-DESARROLLO.md) | Fases F0–F8, entregables, criterios de salida, estimaciones y dependencias. Es el tablero maestro. | Todo el equipo | Se cierra o se re-planea una fase |
| [arquitectura/ARQUITECTURA.md](arquitectura/ARQUITECTURA.md) | Clean Architecture, módulos, flujos de ejecución, ADRs, comparación con MAUI y presupuestos de rendimiento. | Devs, Arquitectura | Cambia una decisión técnica |
| [arquitectura/ESTRUCTURA-PROYECTO.md](arquitectura/ESTRUCTURA-PROYECTO.md) | Árbol de carpetas, responsabilidad de cada una, convenciones de nombres y contrato IPC. | Devs | Se agrega o mueve un módulo |
| [arquitectura/MODELO-DATOS.md](arquitectura/MODELO-DATOS.md) | Modelo entidad-relación, tablas, índices, vistas, retención y políticas de tiempo/zona horaria. DDL canónico en [`db/migrations/0001_init.sql`](../db/migrations/0001_init.sql). | Devs, DBA | Cambia el esquema |
| [arquitectura/INTEGRACION-EMPRESARIAL.md](arquitectura/INTEGRACION-EMPRESARIAL.md) | Puertos y adaptadores para Entra ID, Azure DevOps y Timesheet (PWA/REST), outbox, idempotencia y conflictos. | Devs, Integraciones, Seguridad | Se define o cambia un contrato externo |
| [ux/DISENO-UX-UI.md](ux/DISENO-UX-UI.md) | Estados de la ventana flotante, dimensiones, tokens visuales, interacciones, microcopy y accesibilidad. | UX, Frontend, Product | Cambia una interacción o el sistema visual |
| [producto/MVP-Y-ROADMAP.md](producto/MVP-Y-ROADMAP.md) | Alcance exacto del MVP, historias de usuario con criterios de aceptación y roadmap v1.0/v2.0/v3.0. | Product, Devs, Stakeholders | Se negocia alcance |
| [calidad/ESTRATEGIA-PRUEBAS.md](calidad/ESTRATEGIA-PRUEBAS.md) | Pirámide de pruebas, herramientas, cobertura, e2e, protocolo de rendimiento y CI. | QA, Devs | Cambia la suite o los umbrales |
| [seguridad/ESTRATEGIA-SEGURIDAD.md](seguridad/ESTRATEGIA-SEGURIDAD.md) | Activos, amenazas STRIDE, almacenamiento de tokens, cifrado, CSP, cadena de suministro y privacidad. | Seguridad, Devs | Nuevo requisito o hallazgo |
| [operaciones/ESTRATEGIA-DESPLIEGUE.md](operaciones/ESTRATEGIA-DESPLIEGUE.md) | Empaquetado MSI/NSIS, firma de código, auto-update, Intune, canales y rollback. | DevOps, Soporte | Cambia el pipeline o el modelo de distribución |
| [negocio/RIESGOS-Y-ESTIMACIONES.md](negocio/RIESGOS-Y-ESTIMACIONES.md) | Registro de riesgos con probabilidad/impacto/mitigación y estimación por módulo y fase. | Product, Arquitectura | Revisión quincenal |
| [bitacora/BITACORA.md](bitacora/BITACORA.md) | Diario cronológico: qué se hizo, por qué, qué herramienta se usó y qué sigue. | Todo el equipo | Cada sesión de trabajo |

## Convenciones

1. **RFD antes que código** para cambios de alcance o arquitectura.
2. **Bitácora al cerrar cada sesión**: qué, por qué, dónde y qué sigue.
3. **Documento + código en el mismo PR** cuando cambia un contrato (esquema, IPC, API).
4. **Sin secretos en documentos ni en SQLite** (ver seguridad).
5. Diagramas en Mermaid para que se versionen junto al texto.
6. Idioma: español para producto/documentación; inglés para identificadores de código.
