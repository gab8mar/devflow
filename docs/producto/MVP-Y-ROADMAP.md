# MVP y roadmap

## 1. Alcance exacto del MVP

**Regla:** si una funcionalidad no está en la lista "Incluye", no entra al MVP.

### Incluye

| # | Funcionalidad | Por qué es MVP |
|---|---------------|----------------|
| 1 | Cronómetro flotante always-on-top, sin decoración, transparente, arrastrable | Es el producto |
| 2 | Posición recordada y clamp a pantalla; modo minimizado | No invadir |
| 3 | CRUD local de proyectos y tareas | Sin catálogo no hay registro |
| 4 | Iniciar / pausar / reanudar / finalizar tarea | Ciclo básico |
| 5 | Cambio rápido (proyecto → tarea → Guardar y cambiar) | Núcleo de precisión |
| 6 | Registro automático local en SQLite con WAL | Base de confianza |
| 7 | Pausa por inactividad configurable (5/10/15) retrocedida al último input | Horas reales |
| 8 | Resumen diario por proyecto/tarea | Feedback inmediato |
| 9 | Menú contextual y bandeja del sistema | Acceso sin ventana |
| 10 | Ajustes: tema, transparencia, color, tipografía, always-on-top, autostart, minimizar al inicio | Adopción |
| 11 | Exportación CSV con formato de Timesheet | Valor antes de integración |
| 12 | Recuperación tras cierre inesperado | Cero pérdida |
| 13 | Single instance, autostart opt-in | Operación diaria |
| 14 | Instalador por usuario + auto-update firmado | Distribución real |

### No incluye (explícito)

- Login Entra ID, Azure DevOps, sync real con Timesheet → **v1.0**.
- Detección de contexto → **v2.0**.
- Dashboard/reportes y exportaciones empresariales → **v3.0**.
- Multi-dispositivo, nube, backend propio, analítica de equipo.
- Captura de teclado, pantalla, portapapeles, URLs o títulos de ventana.
- Temas personalizados avanzados o plugins.

## 2. Historias de usuario y criterios de aceptación

### US-01 — Iniciar tarea
**Como** desarrollador **quiero** iniciar una tarea en dos clics **para** no perder tiempo.

- Dado que no hay tarea activa, al elegir proyecto y tarea y pulsar "Guardar y cambiar",
  se crea una entrada `running` con `started_at = now`.
- El cronómetro muestra 00:00:00 y avanza sin escrituras por segundo.
- Si ya hay una tarea activa, se cierra antes de abrir la nueva (misma transacción lógica).

### US-02 — Cambiar tarea
- Clic en el cronómetro abre el switcher con la tarea actual marcada.
- "Guardar y cambiar" cierra la anterior (duración calculada) y abre la nueva en < 300 ms.
- `Esc` no cambia nada.

### US-03 — Pausar / reanudar
- "Pausar" acumula el tiempo transcurrido y deja `status = paused`.
- "Reanudar" fija `last_resumed_at = now` sin sumar el tiempo en pausa.
- El color pasa a ámbar de forma inmediata.

### US-04 — Inactividad
- Sin input durante el umbral, la entrada se pausa **en el timestamp del último input**.
- No aparece ningún popup; el indicador cambia a ámbar.
- Al volver, un clic reanuda (o automático, si está configurado).
- El tiempo inactivo nunca se suma a la tarea.

### US-05 — Resumen diario
- Muestra total del día local y desglose por proyecto/tarea, incluyendo entradas cerradas.
- La entrada en curso aparece como "en curso" y no infla el total cerrado.
- Exportar CSV produce un archivo con fecha, proyecto, tarea, inicio, fin y duración.

### US-06 — Ajustes
- Los cambios de tema/transparencia se aplican sin reiniciar.
- "Iniciar con Windows" y "Minimizar al iniciar" persisten y se respetan.
- Restaurar valores por defecto no borra datos.

### US-07 — Recuperación
- Tras un cierre forzado, al arrancar se recupera la entrada abierta hasta el último heartbeat.
- Se avisa en el resumen, sin modal, con duración recuperada.

### US-08 — Bandeja y salida
- Cerrar la ventana minimiza a bandeja si hay tarea activa (configurable).
- "Salir" desde el menú cierra la tarea activa de forma segura y termina el proceso.

## 3. Criterios de salida del MVP (gate)

- [ ] Todas las historias US-01…US-08 aceptadas.
- [ ] CPU inactiva ≤ 0.5 %, RAM privada total ≤ 150 MB, arranque ≤ 1.5 s (prueba de 8 h).
- [ ] 0 pérdidas de datos en pruebas de cierre forzado y corte de energía simulado.
- [ ] Cobertura dominio/aplicación ≥ 80 %.
- [ ] Instalador firmado, sin admin, con auto-update verificado.
- [ ] Piloto de 2 semanas con 5–10 devs, satisfacción ≥ 4/5.
- [ ] Documentación y bitácora actualizadas; RFD sin decisiones bloqueantes para v1.0.

## 4. Roadmap

### MVP — "Registrar bien" (Fases F0–F5)

Prototipo funcional: cronómetro, cambio de tarea, persistencia, inactividad, resumen, CSV,
instalador. Objetivo: demostrar que el registro real es más fácil que recordar.

### v1.0 — "Cerrar el ciclo" (F6)

- Entra ID (PKCE, navegador del sistema) y sesión silenciosa.
- Importación de proyectos y work items desde Azure DevOps.
- Envío real al Timesheet con outbox, idempotencia y resolución de conflictos.
- Estado de sincronización visible y auditoría local.
- Distribución corporativa (Intune), onboarding y soporte.
- **Gate:** un dev completa su semana sin abrir el Timesheet manualmente.

### v2.0 — "Entender el contexto" (F7)

- Detección de ventana en primer plano y de Azure DevOps.
- Reglas proyecto ↔ patrón, puntuación de confianza.
- Sugerencia visual discreta; nunca cambio automático.
- Calibración con aceptación/descarte.
- **Gate:** ≥ 70 % de sugerencias aceptadas y ≤ 1 cada 15 min.

### v3.0 — "Dar valor al equipo" (F8)

- Reportes locales (día/semana/proyecto), real vs. estimado.
- Exportaciones CSV/XLSX y Power BI.
- Agregados para gestión con privacidad revisada.
- **Gate:** reportes auditables, < 500 ms con 12 meses de datos.

## 5. Fuera de roadmap (por ahora)

- App móvil o web personal.
- Integración con Jira/GitHub (posible adaptador futuro).
- Facturación, costes por hora o nómina.
- Gamificación o rankings de productividad (contradice el propósito).
