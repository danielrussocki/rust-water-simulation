# Plan técnico: reescritura del simulador de agua

**Estado:** aprobado. La [spec aprobada](spec.md) y la constitución prevalecen sobre este plan.

## Punto de partida y límites

El repositorio no contiene todavía una aplicación Rust ni un manifiesto Cargo. La referencia funcional y visual es el commit fijado en la spec. Los slices siguientes entregan subconjuntos observables de esa spec; que un slice cubra parcialmente un RF no significa que todo el RF esté terminado. Ningún comportamiento provisional debe presentarse como equivalente a un criterio que aún no satisface.

Antes de implementar se requieren la aprobación de este plan y un `tasks.md` activo. Cada slice comienza escribiendo sus pruebas de comportamiento, incluidas las de extremo a extremo, y termina ejecutándolas junto con las regresiones y las comprobaciones aplicables. Ningún slice sin pruebas de extremo a extremo puede declararse terminado. No se inician slices con dependencias pendientes.

## Arquitectura técnica propuesta

### Componentes y responsabilidades

Son responsabilidades lógicas, no una distribución anticipada de archivos:

| Componente | Responsabilidad |
|---|---|
| Aplicación y presentación GPUI | Ventana Windows 11, ayuda, mensajes, eventos de entrada y composición de la vista. No calcula la física. |
| Coordinación de interacción | Interpreta eventos de ratón, tacto y teclado, selecciona la interacción activa según la prioridad de la referencia y envía acciones al estado correspondiente. |
| Simulación | Mantiene y actualiza el agua y la física de la esfera con los valores y reglas del original, independientemente de GPUI. |
| Estado de escena | Conserva esfera, cámara, luz, pausa y parámetros necesarios para presentar y modificar la escena. |
| Presentación de escena | Produce la imagen del recipiente, esfera, agua y entorno a partir del estado; calcula los efectos aprobados y entrega el resultado a la vista GPUI. |
| Recursos y fallos | Valida los assets originales, comunica fallos con contexto y preserva la última imagen cuando corresponda. |
| Adaptación gráfica | Selecciona la ejecución gráfica normal o la alternativa y presenta sus resultados bajo el mismo contrato observable. |

### Modelo de datos

- **Agua:** campo de alturas, velocidades y normales; perturbaciones iniciales y acciones del usuario con posición, radio y fuerza. Sus valores y evolución corresponden a la referencia.
- **Esfera:** centro actual y anterior, radio, velocidad y estado de física activada. Los límites, gravedad y rebote son los fijados por la spec.
- **Vista:** ángulos y proyección de cámara, dirección de luz, dimensiones disponibles y escala de pantalla necesarias para dibujar y convertir entradas.
- **Sesión:** pausa, estado de arrastre y tipo de interacción; la prioridad de selección es esfera, agua y fondo.
- **Recursos y presentación:** identidad y disponibilidad de cada asset, último cuadro mostrado y estado de error. No se añade persistencia de usuario.

### Interfaces y contratos

- **Entrada → interacción:** eventos con posición, tipo de contacto y estado de teclas; el resultado identifica la acción de esfera, agua, cámara, pausa, gravedad o luz. Los eventos sobre la ayuda no arrastran la escena.
- **Interacción → simulación/escena:** acciones expresadas en coordenadas de la escena, sin dependencias de controles GPUI. El avance recibe el intervalo transcurrido; se omite si supera un segundo y no avanza automáticamente en pausa.
- **Escena/simulación → presentación:** estado coherente de agua, esfera, cámara y luz, más dimensiones; una actualización produce un cuadro visible y señaliza los errores. Las interacciones admitidas en pausa actualizan la imagen sin avanzar la simulación.
- **Adaptación gráfica → presentación:** ambas rutas aceptan la misma escena y deben cumplir los mismos criterios visuales y funcionales. La alternativa se podrá activar bajo condiciones controladas de prueba sin añadir un control de usuario no previsto en la spec.
- **Recursos/fallos → aplicación:** resultado con todos los assets inválidos al inicio o con causa y contexto; ante fallo irrecuperable posterior, conserva el último cuadro disponible y bloquea la interacción con la escena.
- **Ayuda → sistema:** la activación de un enlace solicita abrir `https://example.com` en el navegador predeterminado; si falla, comunica la causa disponible.

### Decisiones y alternativas

1. **Rust y GPUI:** exigidos por el proyecto. GPUI gestiona ventana, ayuda y entrada; la simulación no depende de sus controles.
2. **Gráficos:** se propone una ruta acelerada compatible con Windows para reproducir los cálculos y efectos originales, y una ruta alternativa para RF-11. Una biblioteca gráfica como `wgpu` es candidata, no una dependencia ya seleccionada ni una garantía de integración. La elección concreta, versión y licencias se justificarán por RF-1/RF-2/RF-11 cuando se necesiten.
3. **Integración con GPUI:** no se presupone una API de texturas compartidas. El primer slice visual debe verificar en Windows la presentación dentro de GPUI, la entrada y el redimensionamiento pertinentes. Se evaluará una vía compatible con GPUI y, si la transferencia de cuadros impide cumplir rendimiento, una integración nativa adecuada; no se aceptará una apariencia o comportamiento distintos sin aprobación. La discusión [GPUI sobre dibujo GPU personalizado](https://github.com/zed-industries/zed/discussions/66371) ilustra el riesgo, no demuestra capacidades de la versión elegida.
4. **Alternativa gráfica:** se evaluará una ruta de menor dependencia del hardware, incluida ejecución por software cuando sea viable. No se afirma que cumpla 30 FPS ni que cubra todos los dispositivos: debe probarse en el equipo del usuario. Si es inviable, se comunica el bloqueo; no se rebajan silenciosamente la fidelidad ni el mínimo.
5. **Pruebas:** reloj, entrada, aleatoriedad y fallos controlables mediante interfaces de prueba, sin introducir opciones visibles ajenas a la spec. Se usan pruebas reales de extremo a extremo sobre la aplicación, complementadas por pruebas de simulación e integración. Los dobles no sustituyen las pruebas de la ruta real.
6. **Recursos:** conservar los assets y atribuciones de la referencia y revisar sus condiciones de uso antes de incorporarlos. La spec no establece dónde residirán: esa decisión no debe impedir detectar todos los assets ausentes o dañados.

**Alternativas descartadas:** sustituir GPUI por otra interfaz, usar la demo WebGL original como aplicación final, introducir nuevas opciones de usuario, aplazar toda integración a un slice final o asumir que una imagen estática satisface la simulación. No se descarta una técnica gráfica concreta sin medirla.

## Reglas comunes a todos los slices

- Cada slice integra solo los componentes necesarios y entrega comportamiento de la spec en una aplicación ejecutable; no hay preparación técnica independiente.
- Antes de implementarlo, escribir pruebas que describan el comportamiento y sus escenarios de extremo a extremo. Ejecutar las pruebas del slice y las regresiones previas; comprobar formato y análisis cuando exista el proyecto Cargo. Si alguna verificación no es posible, el slice queda pendiente.
- Probar en Windows 11 las rutas reales afectadas; una prueba manual o un contrato aislado complementan, pero no sustituyen, las pruebas de extremo a extremo exigidas por la constitución.
- Mantener los valores fijos de la referencia; consultar cualquier nuevo valor aleatorio o no controlado. Las diferencias inevitables requieren aprobación y actualización de la spec antes de implementarlas.
- Las pruebas de fallo y selección de alternativa deben ser controlables sin modificar el comportamiento de usuario. Los criterios de rendimiento y fidelidad completos se validan con la escena e interacciones completas, no se dan por cumplidos por un prototipo parcial.

## Slices verticales

### S01 — Inicio con recursos inválidos

- **Objetivo:** informar por qué no puede iniciarse la demo cuando faltan o están dañados recursos.
- **RF:** RF-11 parcial.
- **Dependencias:** ninguna.
- **Componentes:** presentación GPUI, recursos y fallos.
- **Entregable:** al iniciar con uno o varios assets originales inválidos, la ventana enumera todos los afectados y no inicia la simulación; tras corregirlos y volver a abrir, vuelve a intentar el inicio. El proyecto ejecutable y la validación de recursos nacen aquí porque este comportamiento los necesita.
- **Pruebas:** primero casos de uno y varios assets ausentes/dañados y reinicio; después, extremo a extremo en Windows desde el proceso real con recursos controlados, verificando mensaje y ausencia de simulación.
- **Finalización:** los casos anteriores pasan y todos los assets afectados se identifican; la ruta de inicio correcto queda cubierta por S02, no se declara completado RF-11.

### S02 — Observar la escena y las ondas iniciales

- **Objetivo:** abrir la demo y observar el agua evolucionar en la escena original.
- **RF:** RF-1 completo; RF-2 parcial; RF-11 parcial.
- **Dependencias:** S01.
- **Componentes:** GPUI, recursos, estado de escena, simulación y presentación de escena.
- **Entregable:** escena inicial con esfera, recipiente, entorno, assets, posiciones, cámara y luz de la referencia; 20 perturbaciones iniciales con distribución y parámetros aprobados, evolución y amortiguación del agua. Se integran las piezas visuales necesarias para mostrarlo; los efectos no incluidos todavía quedan explícitamente pendientes de S08.
- **Pruebas:** valores iniciales, generación controlada, distribución y pasos del agua; extremo a extremo iniciando la aplicación y observando cambios de la escena; regresión de S01. Verificación temprana de que la imagen real se integra en la ventana GPUI en Windows.
- **Finalización:** inicio y evolución observables con los valores del slice; el usuario puede inspeccionar la escena, sin declarar todavía la fidelidad visual completa de RF-2.

### S03 — Crear ondas sobre el agua

- **Objetivo:** permitir pulsar y arrastrar sobre el agua para crear ondas.
- **RF:** RF-3 parcial; RF-9 parcial; RF-2 transversal.
- **Dependencias:** S02.
- **Componentes:** GPUI, interacción, simulación y presentación.
- **Entregable:** pulsación y arrastre con ratón o un contacto táctil compatible generan perturbaciones con posición, radio y fuerza originales; la escena evoluciona visiblemente.
- **Pruebas:** primero conversión de posición y parámetros; extremo a extremo desde entrada real hasta cambio observable, con ratón y tacto disponible; regresión de inicio y evolución.
- **Finalización:** ondas verificadas en pulsación y arrastre sin afectar el inicio. Los casos durante pausa se completan en S05.

### S04 — Orbitar la cámara

- **Objetivo:** explorar la escena arrastrando su fondo.
- **RF:** RF-5 parcial; RF-9 parcial.
- **Dependencias:** S03.
- **Componentes:** GPUI, interacción, estado de escena y presentación.
- **Entregable:** el arrastre del fondo rota la cámara con la sensibilidad y límites aprobados; una acción iniciada sobre el agua sigue creando ondas.
- **Pruebas:** primero dirección, sensibilidad y extremos; extremo a extremo de arrastre y cambio de vista, incluidos límites y regresión del agua.
- **Finalización:** cámara operativa y prioridad agua/fondo verificada. La prioridad de la esfera se completa en S06; la cámara en pausa, en S05.

### S05 — Pausar y seguir explorando

- **Objetivo:** detener y reanudar la evolución sin perder las interacciones ya disponibles.
- **RF:** RF-6 parcial; RF-3 y RF-5 completos respecto a pausa.
- **Dependencias:** S04.
- **Componentes:** GPUI, interacción, simulación, escena y presentación.
- **Entregable:** barra espaciadora alterna pausa; el agua no avanza automáticamente, pero se puede modificar el agua y la cámara con actualización visible.
- **Pruebas:** primero transición de pausa y tiempo; extremo a extremo pausando, esperando, interactuando y reanudando; regresiones S01–S04.
- **Finalización:** se observan y verifican pausa, reanudación y ambas interacciones sin avance automático. La pausa de la esfera se completa en S06–S07 y la luz en S08.

### S06 — Mover la esfera

- **Objetivo:** arrastrar la esfera y observar su desplazamiento del agua.
- **RF:** RF-4; RF-9 parcial; RF-6 transversal.
- **Dependencias:** S05.
- **Componentes:** GPUI, interacción, escena, simulación y presentación.
- **Entregable:** selección prioritaria de esfera, arrastre limitado, liberación y desplazamiento del agua; durante pausa cambia su posición sin avance automático.
- **Pruebas:** primero selección ambigua, límites, velocidad y respuesta del agua; extremo a extremo arrastrando y soltando en ejecución y pausa, con regresión de agua y cámara.
- **Finalización:** comportamiento y límites de RF-4 verificados de extremo a extremo; las tres prioridades de arrastre no interfieren.

### S07 — Esfera con gravedad

- **Objetivo:** activar la física de la esfera y observar su interacción con el agua.
- **RF:** RF-7; RF-4 y RF-6 transversales.
- **Dependencias:** S06.
- **Componentes:** GPUI, interacción, escena, simulación y presentación.
- **Entregable:** G alterna física; gravedad, flotabilidad, resistencia y rebote siguen la referencia y se detienen en pausa.
- **Pruebas:** primero estado inicial, fuerzas y rebote con tiempos controlados; extremo a extremo con G, caída, agua, arrastre y pausa; regresiones previas.
- **Finalización:** física activable y desactivable, respuesta y pausa verificadas. La conformidad visual completa del agua y efectos se cierra en S08.

### S08 — Luz y efectos originales

- **Objetivo:** observar la apariencia y efectos originales y ajustar la luz.
- **RF:** RF-2 restante; RF-8; RF-1, RF-4 y RF-5 transversales.
- **Dependencias:** S07.
- **Componentes:** GPUI, interacción, escena, simulación y presentación.
- **Entregable:** reflejos, refracciones, oclusión, sombras y cáusticas originales; L orienta la luz según la cámara, también en pausa, y la mantiene actualizada según la referencia.
- **Pruebas:** primero valores y efectos ante cambios de agua, esfera, cámara y luz; extremo a extremo con L sostenida, cámara y pausa; comparación visual del usuario frente a la referencia con la misma resolución y controles; regresiones previas.
- **Finalización:** RF-2 y RF-8 verificados, incluida validación visual del usuario. Las diferencias inevitables requieren aprobación previa; no se afirma igualdad píxel por píxel.

### S09 — Ayuda, enlaces y ventana

- **Objetivo:** consultar instrucciones y usar la demo en ventanas de distinto tamaño.
- **RF:** RF-9 y RF-10; RF-11 parcial para error al abrir enlace.
- **Dependencias:** S08.
- **Componentes:** GPUI, interacción, escena, presentación y comunicación de fallos.
- **Entregable:** ayuda con textos, atribuciones y enlaces provisionales aprobados; navegador predeterminado y mensaje si falla; redimensionamiento, minimización y reserva de espacio conforme a la referencia; entradas táctiles y casos de foco/contacto especificados.
- **Pruebas:** primero enlaces y fallos controlados, prioridades de entrada y tamaño; extremo a extremo con ayuda, navegador, redimensionamiento en ejecución y pausa, ratón y tacto compatible; comprobar ausencia de arrastre al usar ayuda y comparar casos límite con la referencia.
- **Finalización:** ayuda, enlaces, entradas y comportamiento de ventana verificados sin regresiones. Si el entorno no permite probar tacto real, la verificación queda pendiente, no se declara pasada.

### S10 — Continuidad y errores de la simulación completa

- **Objetivo:** mantener la experiencia con la alternativa gráfica y comunicar fallos sin presentar una simulación rota.
- **RF:** RF-11 restante; RF-1 a RF-10 transversales; requisito no funcional de rendimiento.
- **Dependencias:** S09.
- **Componentes:** todos los que la ruta y los errores ejerciten, incluida adaptación gráfica y recursos.
- **Entregable:** selección y activación controlada de alternativa; errores gráficos y de assets posteriores al inicio, último cuadro congelado y bloqueo de interacciones cuando corresponda; comunicación en ventana o consola según las reglas aprobadas.
- **Pruebas:** primero selección, precedencia y errores controlados; extremo a extremo con ruta normal y alternativa, fallo de ambas, fallos posteriores, recurso aún utilizable y recurso inutilizable; regresión integral. En el equipo del usuario, a 1920 × 1080, medir durante un minuto con agua, esfera y cámara en movimiento: cada intervalo de un segundo debe alcanzar 30 FPS en cada ruta. Validación visual del usuario en ambas rutas.
- **Finalización:** RF-11 y el resto de criterios de la spec están verificados, incluidas ambas rutas y rendimiento en el equipo del usuario. Si no se alcanza el mínimo o no es viable la alternativa, el slice y la entrega quedan bloqueados; se consulta al usuario sin cambiar la spec silenciosamente.

## Orden, trazabilidad y conflictos

El orden propuesto es S01 → S02 → S03 → S04 → S05 → S06 → S07 → S08 → S09 → S10. Cada flecha refleja una dependencia funcional o de regresión declarada; la integración sucede en cada slice, no al final. S01 evita construir infraestructura gráfica antes de necesitarla; S02 integra la primera vista real.

| RF | Slices | Cobertura |
|---|---|---|
| RF-1 | S02, S08, S10 | Inicio y ondas; fidelidad visual y regresión en ambas rutas. |
| RF-2 | S02, S03, S06–S08, S10 | Evolución, respuesta a interacción y efectos; regresión de ambas rutas. |
| RF-3 | S03, S05, S09–S10 | Agua activa, pausa y entradas completas. |
| RF-4 | S06–S08, S10 | Arrastre, interacción, física relacionada y regresión. |
| RF-5 | S04–S05, S08–S10 | Cámara, pausa, luz y ventana. |
| RF-6 | S05–S08, S10 | Pausa con agua, cámara, esfera, física y luz. |
| RF-7 | S07, S10 | Gravedad, pausa y regresión en ambas rutas. |
| RF-8 | S08, S10 | Luz, cáusticas y regresión en ambas rutas. |
| RF-9 | S03–S04, S06, S09–S10 | Entrada parcial y ayuda, tacto y enlaces completos. |
| RF-10 | S09–S10 | Ventana y regresión. |
| RF-11 | S01–S02, S09–S10 | Assets al inicio, fallos relevantes y alternativa completa. |

**Conflictos a comprobar:** prioridad esfera/agua/fondo frente a ayuda y tacto; actualizaciones de imagen frente a pausa; luz y cámara con cáusticas; redimensionamiento y coordenadas de entrada; pérdida de recursos frente a última imagen; fidelidad frente a rendimiento y selección de ruta. Los comportamientos transversales se añaden a regresión al introducir cada slice afectado.

## Riesgos y decisiones pendientes de validación

- **GPUI y gráficos 3D en Windows:** la vía de composición, selección de versión y soporte de entrada táctil requieren verificación real desde los primeros slices consumidores; no se da por probada una API específica.
- **Alternativa y rendimiento:** reproducir los efectos con mínimo 30 FPS en cada segundo en ambas rutas puede no ser viable en el equipo del usuario. No se garantiza hasta medirlo con la escena completa.
- **Comparación de la referencia:** el tiempo y la aleatoriedad afectan estados dinámicos; se controlan en pruebas y se conserva la validación visual del usuario. Si se identifica otro valor no controlado se consulta antes de implementar lo afectado.
- **Errores y superficie gráfica:** si falla el mecanismo que presenta la imagen, podría no ser posible mostrar el mensaje o conservar el último cuadro como exige la spec; debe verificarse y escalarse si bloquea el contrato.
- **Assets y licencias:** verificar procedencia y condiciones de todos los recursos y atribuciones, sin asumir que la licencia del código cubre automáticamente cada imagen.
- **Entorno de aceptación:** se necesita acceso al equipo Windows 11 del usuario, a 1920 × 1080, y a los medios necesarios para los escenarios reales de entrada y rendimiento. Una ejecución en un solo equipo no demuestra compatibilidad universal.
