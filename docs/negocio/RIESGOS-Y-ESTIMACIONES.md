# Riesgos y estimaciones

## 1. Registro de riesgos

Probabilidad/Impacto: A = alto, M = medio, B = bajo. Revisión quincenal y al cierre de fase.

| ID | Riesgo | Prob. | Impacto | Mitigación | Disparador |
|----|--------|-------|---------|------------|------------|
| R-01 | Toolchain Rust ausente o bloqueado por políticas | B | A | **Mitigado 2026-11-24:** Rust 1.100 instalado sin admin y compila con MSVC 14.16 (VS 2017) + SDK 19041 | F1 no compila |
| R-02 | WebView2 desactualizado en la flota | M | A | Bootstrapper/evergreen, detección en instalador, prueba en imágenes corporativas | Fallo de render en piloto |
| R-03 | Contrato del Timesheet desconocido o inestable (Q1) | A | A | Puerto + mock + CSV en MVP; congelar contrato antes de F6 | F6 sin sandbox |
| R-04 | Always-on-top/transparencia falla en fullscreen, RDP o multi-monitor | M | M | Matriz de pruebas, ocultar en fullscreen, clamp, ajustes | Reportes de solapamiento |
| R-05 | Detección de inactividad imprecisa | M | M | `GetLastInputInfo`, backdating al último input, prueba de 8 h | Diferencia > 30 s |
| R-06 | Curva de aprendizaje de Rust retrasa el MVP | M | A | Superficie Rust mínima, dominio acotado, pair review | F2 excede estimación 50 % |
| R-07 | Robo de foco o interrupciones percibidas | M | A | Nunca `focus()` automático, pruebas de teclado, piloto temprano | Quejas del piloto |
| R-08 | Pérdida o corrupción de datos | B | A | WAL, heartbeat, backup pre-migración, recuperación, pruebas de kill | `integrity_check` falla |
| R-09 | Falsos positivos de antivirus/EDR | M | A | Firma de código, reputación, envío previo a AV, instalador estándar | Bloqueo en piloto |
| R-10 | Scope creep (reportes/contexto antes de tiempo) | A | M | MVP cerrado, RFD como contrato, gate por fase | Nuevas historias sin RFD |
| R-11 | Baja adopción por fricción percibida | M | A | Cambio en ≤ 2 clics, onboarding, piloto, feedback semanal | Uso < 60 % del piloto |
| R-12 | Exposición de tokens/secretos | B | A | Credential Manager, redacción, auditoría, checklist | Hallazgo en revisión |
| R-13 | Dependencia de una sola persona | M | M | Documentación viva, bitácora, PR review, estándares | Ausencia del dev |
| R-14 | Cifrado local exigido tarde (Q3) | M | M | Diseñar abstracción de DB; evaluar SQLCipher temprano | Requisito de seguridad |

## 2. Estimaciones por fase

| Fase | Días-persona | Incertidumbre | Comentario |
|------|--------------|---------------|------------|
| F0 Diseño | 3–4 | Baja | Documentación y acuerdos |
| F1 Scaffolding | 3–4 | Media | Depende de instalar Rust |
| F2 Dominio/persistencia | 4–5 | Media | Mayor densidad de pruebas |
| F3 UI flotante | 5–6 | Media | Iteración UX y ventana nativa |
| F4 Productividad | 4–5 | Media | Idle, resumen, tray, export |
| F5 Hardening/piloto | 4–5 | Alta | Firma, updater, feedback |
| **MVP (F0–F5)** | **23–29** | — | ~5–6 semanas calendario |
| F6 v1.0 Integración | 10–15 | Alta | Depende de Q1/Q2 |
| F7 v2.0 Contexto | 8–10 | Alta | Heurísticas y calibración |
| F8 v3.0 Reportes | 10–15 | Media | Depende de datos históricos |

Contingencia recomendada: **+20 %** sobre cada fase con incertidumbre alta.

## 3. Complejidad por módulo

| Módulo | Complejidad | Razón |
|--------|-------------|-------|
| Máquina de estados del cronómetro | M | Invariantes, recuperación, DST |
| Persistencia SQLite + migraciones | M | Transacciones, WAL, evolución |
| Ventana nativa (transparencia, always-on-top, DPI) | M–A | Comportamiento del SO y multi-monitor |
| Switcher y temas | M | UX, teclado, accesibilidad |
| Inactividad | M | Win32, backdating, configuración |
| Outbox/sync | A | Idempotencia, conflictos, offline |
| Entra ID PKCE | M–A | Loopback, rotación, Conditional Access |
| Adaptador ADO | M | REST, paginación, rate limit |
| Adaptador Timesheet | A | Contrato real y semántica de periodos |
| Detección de contexto | A | Falsos positivos y confianza |
| Reportes | M | Agregados y rendimiento |

## 4. Supuestos

1. Un desarrollador senior full-time y QA/UX parcial; si es media persona, duplicar calendario.
2. Windows 10 22H2+ x64; ARM no requerido en MVP.
3. Existe App Registration y sandbox de Timesheet/ADO para v1.0.
4. La organización permite distribuir binarios firmados vía Intune.
5. No se exige cifrado de DB en MVP; si se exige, +2–4 días.
6. El Timesheet acepta granularidad por entrada o agregado diario (Q1).

## 5. Plan de contingencia

- **Si Rust se bloquea:** evaluar MAUI con el mismo diseño hexagonal (aceptando peso/arranque) o
  un prototipo WinUI 3; decisión en RFD con impacto en presupuestos no funcionales.
- **Si F6 se bloquea por contrato:** mantener CSV como entrega de valor y posponer sync;
  comunicar a stakeholders.
- **Si el piloto no adopta:** entrevistas, ajustar fricción (posición, atajos, onboarding) antes
  de agregar funciones.
- **Si el rendimiento no cumple:** perfilado ETW, reducir polls, diferir escrituras, revisar
  tamaño del bundle y plugins de Tauri.

## 6. Indicadores de salud del proyecto

- Desviación de estimación por fase (< 25 %).
- Cobertura y deuda técnica (warnings, TODOs).
- Uso real en piloto (días activos, cambios de tarea/día).
- Precisión percibida (encuesta semanal breve).
- Incidentes de datos: objetivo cero.
