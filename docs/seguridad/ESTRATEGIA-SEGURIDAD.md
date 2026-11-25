# Estrategia de seguridad

## 1. Activos a proteger

| Activo | Sensibilidad | Dónde vive |
|--------|--------------|------------|
| Refresh/access tokens | Crítica | Windows Credential Manager (nunca SQLite) |
| Historial de tiempo | Alta (dato laboral) | SQLite local |
| Metadatos de actividad | Media | SQLite local |
| Binarios y actualizaciones | Alta (integridad) | Firma + updater firmado |
| Preferencias | Baja | SQLite local |

## 2. Modelo de amenazas (STRIDE resumido)

| Amenaza | Escenario | Control |
|---------|-----------|---------|
| Spoofing | App falsa pide login | Firma de código, updater firmado, nombre de publisher verificado |
| Tampering | Modificación de DB/entradas | Invariantes en DB, auditoría, checksum de migraciones, backup |
| Repudiation | "No registré esas horas" | `activity_history` local + `idempotency_key` + timestamps |
| Information disclosure | Token en logs/DB, PII en reportes | Redacción, Credential Manager, minimización, sin telemetría |
| Denial of service | DB bloqueada/corrupta | WAL, `busy_timeout`, single-instance, recuperación y backup |
| Elevation of privilege | Instalador pide admin | Instalación por usuario, sin servicios ni drivers |
| Supply chain | Dependencia comprometida | Lockfiles, `cargo deny`, `cargo audit`, SBOM, revisión de updates |

## 3. Almacenamiento de tokens

- **Windows Credential Manager** vía crate `keyring` (backend nativo, DPAPI por usuario).
- Clave por cuenta: `DevTime/{tenant}/{upn}/{provider}`.
- En SQLite solo `credential_ref` (identificador), `scopes` y `expires_at`.
- Access token en memoria; refresh token persistido; rotación al renovar.
- Borrado en logout: elimina credencial y metadatos de cuenta; el historial local permanece.
- Prohibido: tokens en `user_preferences`, logs, crash reports, URLs o variables de entorno.

## 4. Autenticación (Entra ID)

- Authorization Code + **PKCE (S256)**, redirect loopback, navegador del sistema.
- Sin client secrets en la app (cliente público).
- Validar `iss`, `tid`, `aud`, expiración y firma del token cuando aplique.
- Respetar MFA y Conditional Access; nunca intentar evadir.
- Un solo tenant; cuentas multi-tenant requieren decisión explícita (Q2).
- Timeout de login y cancelación limpia del listener.

## 5. Cifrado local

- **MVP:** SQLite sin cifrar; secretos siempre en Credential Manager; permisos de archivo
  restringidos al usuario (`%LOCALAPPDATA%`, ACL por usuario).
- **v1.0 (según Q3):** SQLCipher con clave derivada y almacenada en Credential Manager/DPAPI.
  Coste estimado: +1–2 MB y ~5–10 % de CPU en escrituras (medir).
- Backups: copia local con la misma protección; nunca en ubicaciones sincronizadas sin cifrar.
- Exportaciones CSV son texto plano: aviso explícito al usuario y borrado seguro opcional.

## 6. Endurecimiento de la app

- CSP estricta en Tauri: `default-src 'self'; img-src 'self' data:; style-src 'self' 'unsafe-inline';
  script-src 'self'; connect-src 'self' https://login.microsoftonline.com https://dev.azure.com`.
- Sin `eval`, sin contenido remoto, sin `<iframe>`.
- Capabilities de Tauri mínimas: solo ventanas, tray, updater, autostart, http a dominios
  permitidos y sql. Sin shell, sin filesystem amplio.
- Validación de entradas en el borde IPC (Zod + tipos generados).
- Sin `unwrap` en rutas de producción; errores saneados hacia la UI.
- Single instance para evitar corrupción por doble escritura.
- Auto-update con firma (minisign) y HTTPS; rechazar versiones no firmadas o downgrade.

## 7. Logging y privacidad

- Logs locales con rotación (14 días), nivel configurable; **sin** PII, tokens ni títulos.
- IDs internos y códigos de error sí; contenido de trabajo no.
- Telemetría/crash reporting **desactivada por defecto**; opt-in explícito, con vista previa
  del payload y opción de borrado.
- `activity_history` guarda metadatos mínimos con retención de 90 días.
- Aviso de privacidad en el onboarding: qué se guarda, dónde y por qué.

## 8. Cadena de suministro

- Lockfiles commiteados (`Cargo.lock`, `package-lock.json`).
- `cargo deny` (licencias, duplicados, advisories), `cargo audit`, `npm audit`.
- SBOM CycloneDX generado en CI y archivado por release.
- Dependencias nuevas: justificar tamaño, licencia y mantenimiento; evitar crates sin uso.
- Renovación de dependencias en ventana controlada, con pruebas completas.
- Reproducibilidad razonable: toolchain fijado (`rust-toolchain.toml`), Node fijado (`.nvmrc`).

## 9. Distribución y actualizaciones

- Firma de código (Azure Trusted Signing o certificado EV) en todos los binarios.
- Instalador por usuario, sin admin, sin servicios.
- Updater: manifiesto firmado en Azure Blob/Static Web App; HTTPS; verificación antes de aplicar.
- Rollout gradual (5 % → 25 % → 100 %) con telemetría opt-in de fallos de arranque.
- Rollback: publicar versión anterior como "latest" del canal y permitir downgrade controlado.

## 10. Respuesta a incidentes

1. Detectar (reporte interno, AV, advisory de dependencia).
2. Contener: despublicar versión, rotar secretos de firma/updater si aplica.
3. Erradicar: parche, pruebas, nueva firma.
4. Recuperar: release con aviso y migración de datos si hubo corrupción.
5. Lecciones: actualizar este documento y la bitácora; revisar amenazas.

Contacto y plazos según política corporativa de seguridad (a definir con el cliente).

## 11. Checklist de seguridad por release

- [ ] Sin secretos en repo, DB, logs ni artefactos.
- [ ] `cargo audit`, `cargo deny`, `npm audit` sin hallazgos altos/críticos.
- [ ] CSP y capabilities revisadas (sin permisos nuevos injustificados).
- [ ] Firmas de binarios y manifiesto de updater válidas.
- [ ] Prueba de logout/rotación de token.
- [ ] Revisión de privacidad del payload de sync (sin datos innecesarios).
- [ ] Permisos de archivos de DB/backups correctos.
