# Diseño UX/UI

Filosofía: **"Visible cuando se necesita. Invisible cuando se está trabajando."**

## 1. Principios de interacción

1. El cronómetro se lee de un vistazo; el detalle aparece al interactuar.
2. Nada se abre, cambia ni suena sin acción del usuario.
3. Cambiar de tarea debe ser más rápido que abrir el navegador.
4. El estado siempre es visible: verde/azul (activo), ámbar (pausado/inactivo), gris (sin tarea).
5. Todo control tiene herramienta y atajo; nada depende del hover únicamente.

## 2. Inventario de estados

### 2.1 Compacto (reposo)

```
┌──────────────────┐
│ ◷ 01:42:15  ● ▶  │      132 × 28 px
└──────────────────┘
```

- Solo icono, tiempo, un punto de color de estado y un botón iniciar/pausar/reanudar.
- Arrastrable desde icono, tiempo y punto; el botón queda fuera de la región de arrastre.
- Sin borde visible con el tema oscuro; sombra suave.
- Al pasar el mouse, se expande (150 ms) a hover.

**Implementado en F2:** píldora de 132×28 con icono SVG, duración, punto de estado, botón y
caret para abrir tareas; clic en el botón inicia/pausa/reanuda y el resto arrastra. El arrastre
requiere `core:window:allow-start-dragging` en la capability de Tauri. Hover y menú contextual
siguen pendientes de F3.

### 2.2 Hover / expandido

```
┌──────────────────────────────┐
│ ⏱ 01:42:15        –  ⚙  ✕   │   264 × 92 px
│ Proyecto: ACTIVA             │
│ Tarea: Bug #542              │
│ ● En desarrollo              │
└──────────────────────────────┘
```

- `–` minimiza a píldora; `⚙` abre ajustes; `✕` cierra (con confirmación solo si hay tarea
  activa: "La tarea sigue registrando. ¿Salir de DevTime?").
- Los botones aparecen solo en hover y con opacidad 0.6 → 1 al pasar sobre ellos.
- Si la ventana está cerca del borde inferior de la pantalla, se expande hacia arriba.

### 2.3 Switcher (clic en el cronómetro o "Cambiar tarea")

```
┌────────────────────────────────┐
│ Cambiar tarea                  │   320 × 440 px
│ [ Buscar…                    ] │
│                                │
│ PROYECTOS                      │
│ ● ACTIVA                    5  │
│ ○ ALERTA                    2  │
│ ○ PROYECTO X                8  │
│                                │
│ TAREAS · ACTIVA                │
│ ✓ Bug #542  Login falla   8 m  │
│   Task #611 Refactor API  1 h  │
│   Task #620 Tests sync    2 h  │
│                                │
│ [ Guardar y cambiar        ]   │
└────────────────────────────────┘
```

- Paso 1: proyecto (lista corta con contador de tareas activas).
- Paso 2: tarea (búsqueda incremental, uso reciente primero).
- Botón principal siempre visible: "Guardar y cambiar" (Enter).
- `Esc` cierra sin cambios; las flechas navegan; `Tab` recorre.
- La tarea actual aparece marcada con ✓ y su tiempo acumulado.
- "＋ Nueva tarea" discreto al final de la lista (MVP: alta rápida de título).

**Implementado parcial (2026-11-25):** selector de 320×360 con lista de tareas activas, tarea
actual marcada, uso reciente primero, etiqueta `Local`/`PWA` y alta local con `Enter`/`Crear`.
Al seleccionar, la ventana vuelve a 132×28. Pendientes: búsqueda, lista de proyectos, botón
"Guardar y cambiar" y navegación por teclado.

### 2.4 Minimizado

```
┌───┐        ┌───┐
│ ⏱ │   o    │ ● │     28 × 28 px
└───┘        └───┘
```

- El punto hereda el color de estado; el tiempo aparece en tooltip al hover.
- Se restaura con clic (vuelve a compacto).

### 2.5 Inactividad detectada

- El punto y el tiempo pasan a ámbar; el cronómetro se congela.
- Tooltip: "Inactividad detectada. El contador está en pausa. Haz clic para reanudar."
- Borde ámbar suave (2 px) con pulso lento **solo** si `prefers-reduced-motion` no está activo.
- Al reanudar: un clic; opcionalmente automático si el usuario lo configura.

### 2.6 Sugerencia de cambio de contexto (v2.0)

```
┌────────────────────────────────┐
│ ⏱ 01:42:15  ⚠          – ⚙ ✕  │   borde ámbar suave
│ Posible cambio de tarea        │
└────────────────────────────────┘
```

Al hacer clic en el aviso (no en el cronómetro):

```
Posible cambio de tarea detectado.
Detectamos: Azure DevOps · Bug #611

[ Continuar actual ]  [ Cambiar tarea ]
```

- Nunca cambia automáticamente.
- Máximo una sugerencia cada 15 min (configurable; silenciable).
- Se descarta al cerrar o al continuar; se registra para calibrar.

### 2.7 Menú contextual (clic derecho)

```
Cambiar tarea
Pausar / Reanudar
Finalizar tarea
──────────────
Resumen diario
Configuración
──────────────
Salir
```

- "Finalizar tarea" pide confirmación implícita: cierra y deja estado "Sin tarea activa".
- "Pausar" cambia a "Reanudar" según estado.

### 2.8 Resumen diario (ventana)

```
┌──────────────────────────────────────────┐
│ Resumen de hoy — lunes 23 de noviembre   │
│ Total registrado              6 h 12 min │
│ Sin asignar                   0 h 18 min │
│                                          │
│ ACTIVA                                   │
│   Bug #542 Login falla        2 h 05 min │
│   Task #611 Refactor API      1 h 30 min │
│ ALERTA                                   │
│   Task #620 Tests sync        2 h 37 min │
│                                          │
│ [Exportar CSV]        [Cerrar]           │
└──────────────────────────────────────────┘
```

- Barra proporcional por proyecto (sin gráficos pesados en MVP).
- "Sin asignar" = tiempo detectado sin tarea activa (si aplica).
- Exportar CSV con columnas: fecha, proyecto, tarea, inicio, fin, duración, nota.

### 2.9 Ajustes (ventana)

Secciones con navegación lateral: **Apariencia · Comportamiento · Productividad · Datos**.

| Sección | Controles |
|---------|-----------|
| Apariencia | Tema (oscuro/claro/sistema), transparencia 60–100 %, color principal (5 presets + selector), radio 4–12 px, sombra on/off, tipografía (Segoe UI, Inter, Roboto, Cascadia Code) |
| Comportamiento | Always on top, iniciar con Windows, recordar posición, iniciar minimizado, atajo global (opcional) |
| Productividad | Inactividad (5/10/15 min), acción al detectar (pausar / preguntar), reanudación (manual/automática), recordatorios, alertas de contexto |
| Datos | Ruta de la DB, exportar CSV, backup manual, borrar historial de actividad, política de retención |

## 3. Sistema visual

### 3.1 Tokens (CSS variables)

| Token | Oscuro | Claro |
|-------|--------|-------|
| `--dt-bg` | `rgba(22, 27, 34, 0.92)` | `rgba(255, 255, 255, 0.92)` |
| `--dt-bg-solid` | `#161B22` | `#FFFFFF` |
| `--dt-fg` | `#E6EDF3` | `#1B1F24` |
| `--dt-muted` | `#8B949E` | `#57606A` |
| `--dt-border` | `#30363D` | `#D0D7DE` |
| `--dt-accent` | `#58A6FF` | `#0969DA` |
| `--dt-running` | `#3FB950` | `#1A7F37` |
| `--dt-paused` | `#D29922` | `#9A6700` |
| `--dt-alert` | `#E3B341` | `#BF8700` |
| `--dt-danger` | `#F85149` | `#CF222E` |
| `--dt-shadow` | `0 4px 16px rgba(0,0,0,.45)` | `0 4px 16px rgba(31,35,40,.15)` |

- Radio: 6 px compacto, 10 px paneles. Altura de control ≥ 24 px.
- Tipografía: 12 px compacto, 13 px paneles, 20 px título de resumen.
- Números tabulares (`font-variant-numeric: tabular-nums`) para evitar saltos.
- Iconos: SVG inline, trazo 1.5 px, 14–16 px (sin librería pesada).

### 3.2 Temas y transparencia

- Tema por `data-theme="dark|light"`; "sistema" sigue `prefers-color-scheme`.
- Transparencia aplicada al fondo de la ventana (alpha), no a los controles.
- La opacidad mínima 60 % mantiene contraste ≥ 4.5:1 sobre fondos típicos.
- Alto contraste de Windows: se respeta `forced-colors` (sin transparencia).

## 4. Interacciones y atajos

| Acción | Resultado |
|--------|-----------|
| Hover 150 ms | Expande a hover |
| Salir 400 ms | Colapsa (si no hay panel abierto) |
| Clic en cronómetro | Abre switcher |
| Clic derecho | Menú contextual |
| Arrastrar cuerpo compacto | Mueve ventana; guarda posición al soltar |
| `Esc` | Cierra panel/menú |
| `Enter` | Confirma "Guardar y cambiar" |
| `Ctrl+Shift+T` (opcional) | Mostrar/ocultar ventana |
| `Ctrl+Shift+P` (opcional) | Pausar/reanudar |

Regla anti-invasividad: **ningún atajo global se registra por defecto**; el usuario lo activa.

**Estado F2:** implementado el clic en el botón para iniciar/pausar/reanudar y el arrastre del
cuerpo. Hover, clic en el cronómetro para switcher, menú contextual, atajos y guardado de
posición quedan para F3.

## 5. Movimiento

- Expansión 150 ms `cubic-bezier(0.2, 0, 0, 1)`; colapso 120 ms.
- Sin animaciones en `prefers-reduced-motion: reduce` ni en alto contraste.
- Sin animación de la cifra; solo actualización del texto (tabular).
- Pulso de alerta: 2 s, opacidad 0.6 → 1, máximo 3 ciclos.

## 6. Accesibilidad

- Contraste mínimo 4.5:1 en texto y 3:1 en iconos/indicadores.
- Foco visible (anillo 2 px `--dt-accent`) y orden lógico de tabulación.
- Switcher operable 100 % con teclado; `aria-live="polite"` solo para cambios de estado
  relevantes (pausa por inactividad), nunca por cada segundo.
- Etiquetas accesibles en español; iconos con `aria-label`.
- Objetivos táctiles/ratón ≥ 24 px; zona de arrastre amplia.
- No depender solo del color: cada estado tiene icono o texto asociado.

## 7. Microcopy (español)

| Clave | Texto |
|-------|-------|
| `timer.idle` | Inactividad detectada. Contador en pausa. |
| `timer.no_task` | Sin tarea activa |
| `timer.running` | En curso |
| `timer.paused` | En pausa |
| `timer.start` | Iniciar tarea general |
| `timer.pause` | Pausar |
| `timer.resume` | Reanudar |
| `switcher.title` | Cambiar tarea |
| `switcher.search` | Buscar proyecto o tarea… |
| `switcher.confirm` | Guardar y cambiar |
| `switcher.newTask` | Nueva tarea |
| `menu.pause` | Pausar |
| `menu.resume` | Reanudar |
| `menu.close` | Finalizar tarea |
| `menu.summary` | Resumen diario |
| `menu.settings` | Configuración |
| `menu.quit` | Salir |
| `context.suggestion` | Posible cambio de tarea detectado. ¿Desea cambiar de actividad? |
| `context.keep` | Continuar actual |
| `context.switch` | Cambiar tarea |
| `exit.confirm` | La tarea sigue registrando. ¿Salir de DevTime? |
| `summary.total` | Total registrado |
| `summary.unassigned` | Sin asignar |
| `summary.export` | Exportar CSV |
| `recovered` | Se recuperó una entrada de {duration} tras un cierre inesperado. |

## 8. Anti-patrones prohibidos

- Popups modales automáticos, notificaciones toast por cada cambio, sonidos.
- Animaciones que llamen la atención en reposo.
- Cambiar de tarea sin confirmación.
- Sobreponerse al botón de inicio o a la barra de tareas (clamp de posición).
- Mostrar datos personales o títulos de ventana en la UI.
- Pedir login al arrancar si el usuario solo quiere registrar tiempo local.
