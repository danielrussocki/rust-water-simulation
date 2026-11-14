# Tareas: reescritura del simulador de agua

Referencia: [spec aprobada](spec.md) y [plan aprobado](plan.md).

## Reglas de ejecución

- Seguir estrictamente S01 → S02 → S03 → S04 → S05 → S06 → S07 → S08 → S09 → S10. No iniciar un slice con dependencias pendientes.
- Escribir primero las pruebas de comportamiento y de extremo a extremo del slice. Una tarea técnica terminada no implica que el slice esté terminado.
- Cada slice requiere pruebas de extremo a extremo sobre la aplicación real en Windows 11; los dobles y las comprobaciones manuales complementan esas pruebas, no las sustituyen.
- Para cada cierre ejecutar la regresión disponible, `cargo fmt --check`, `cargo clippy` y las pruebas automatizables aplicables. S01 no tiene slices previos: su regresión consiste en volver a ejecutar sus pruebas de comportamiento tras integrar el cambio.
- Marcar una casilla solo cuando se cumpla su «Hecho cuando». Si falla o no puede ejecutarse una verificación, dejar la casilla y el slice pendientes y comunicar el bloqueo. Las estimaciones son orientativas: la mayoría busca 20–30 minutos, pero algunas verificaciones reales pueden requerir más.
- Consultar valores no controlados nuevos o diferencias inevitables antes de implementar lo afectado. No ampliar el alcance ni modificar la spec aprobada silenciosamente.

## S01 — Inicio con recursos inválidos

**Objetivo:** informar por qué no inicia la demo ante recursos inválidos.  
**RF:** RF-11 parcial. **Dependencias:** ninguna.  
**Resultado observable:** una ventana enumera todos los assets ausentes o dañados, impide iniciar la simulación y permite reintentar cerrando y abriendo tras corregirlos.

- [x] **S01-T01** Escribir pruebas de comportamiento para uno y varios assets ausentes o dañados. **RF:** RF-11. **Hecho cuando:** las pruebas exigen la lista completa y el bloqueo del inicio.
- [x] **S01-T02** Escribir la prueba de extremo a extremo de inicio fallido y reapertura tras corrección. **RF:** RF-11. **Hecho cuando:** el escenario lanza el proceso real y verifica mensaje, ausencia de simulación y nuevo intento.
- [x] **S01-T03** Verificar la procedencia y condiciones de uso de los assets originales necesarios. **RF:** RF-11; atribución de la spec. **Hecho cuando:** cada asset requerido y sus condiciones están identificados y los bloqueos de licencia comunicados.
- [x] **S01-T04** Preparar el ejecutable Rust con GPUI mínimo que necesita el mensaje de inicio. **RF:** RF-11. **Hecho cuando:** la aplicación abre una ventana real en Windows 11 y las dependencias utilizadas están justificadas.
- [x] **S01-T05** Validar todos los assets requeridos y reunir sus errores. **RF:** RF-11. **Hecho cuando:** los casos de ausencia y daño producen una lista completa con identidad de cada recurso.
- [x] **S01-T06** Conectar el resultado de validación con el mensaje y el bloqueo de inicio. **RF:** RF-11. **Hecho cuando:** la ventana muestra todos los afectados y no presenta la simulación como iniciada.
- [x] **S01-T07** Ejecutar aceptación de extremo a extremo y reapertura con assets corregidos. **RF:** RF-11. **Hecho cuando:** las pruebas del proceso real pasan en Windows 11; el inicio visual correcto se verifica en S02.
- [x] **S01-T08** Ejecutar regresión y comprobaciones de formato y análisis. **RF:** RF-11. **Hecho cuando:** las pruebas de S01 pasan tras integrar y las comprobaciones aplicables no presentan fallos.
- [x] **S01-T09** Verificar criterios e integración recursos → ventana. **RF:** RF-11. **Hecho cuando:** el mensaje completo, el bloqueo y el nuevo intento están comprobados; RF-11 permanece parcial.

## S02 — Observar la escena y las ondas iniciales

**Objetivo:** iniciar la demo y observar el agua en movimiento.  
**RF:** RF-1; RF-2 y RF-11 parciales. **Dependencias:** S01.  
**Resultado observable:** escena con assets, esfera, recipiente, entorno y ondas iniciales que evolucionan; los efectos visuales completos se cierran en S08.

- [ ] **S02-T01** Escribir pruebas de valores iniciales y de las 20 perturbaciones. **RF:** RF-1. **Hecho cuando:** comprueban posiciones iniciales fijas, estados, radio, fuerzas alternadas y selección uniforme independiente con entradas controladas.
- [ ] **S02-T02** Escribir pruebas de evolución y del intervalo superior a un segundo. **RF:** RF-2. **Hecho cuando:** comprueban dos pasos por actualización, amortiguación `0.995` y omisión del intervalo indicado.
- [ ] **S02-T03** Escribir una prueba de extremo a extremo de inicio y movimiento visible. **RF:** RF-1, RF-2. **Hecho cuando:** el proceso real exige escena con recursos originales y cuadros que reflejan la evolución del agua.
- [ ] **S02-T04** Seleccionar y justificar la vía gráfica necesaria para mostrar la escena con GPUI en Windows. **RF:** RF-1, RF-2. **Hecho cuando:** versión, dependencias y vía de presentación tienen justificación y se comprueba su viabilidad en la ventana real; no se supone que GPUI comparta texturas.
- [ ] **S02-T05** Inicializar estado de esfera, cámara, luz y perturbaciones originales. **RF:** RF-1. **Hecho cuando:** las pruebas de valores y generación pasan sin introducir controles nuevos.
- [ ] **S02-T06** Integrar evolución del agua con el estado y los intervalos de actualización. **RF:** RF-2. **Hecho cuando:** pasan las pruebas de pasos, amortiguación y omisión.
- [ ] **S02-T07** Presentar recipiente, esfera, entorno y agua con los assets originales. **RF:** RF-1, RF-2. **Hecho cuando:** la ventana real muestra la escena y actualiza los cuadros; los efectos pendientes se identifican como parciales.
- [ ] **S02-T08** Ejecutar aceptación de extremo a extremo y verificar la integración de la imagen en GPUI. **RF:** RF-1, RF-2. **Hecho cuando:** inicio, escena y evolución pasan en Windows 11; se comprueba la presentación y redimensionamiento pertinente de la vista.
- [ ] **S02-T09** Ejecutar regresión S01 y comprobaciones aplicables. **RF:** RF-1, RF-2, RF-11. **Hecho cuando:** la ruta de error de recursos y las verificaciones de formato, análisis y pruebas pasan.
- [ ] **S02-T10** Verificar criterios e integración recursos → simulación → escena → ventana. **RF:** RF-1, RF-2. **Hecho cuando:** el estado y la evolución del slice se observan desde la aplicación; la fidelidad completa de RF-2 queda pendiente de S08.

## S03 — Crear ondas sobre el agua

**Objetivo:** crear ondas mediante pulsación y arrastre.  
**RF:** RF-3 y RF-9 parciales; RF-2 transversal. **Dependencias:** S02.  
**Resultado observable:** ratón o un contacto táctil compatible producen ondas con parámetros originales.

- [ ] **S03-T01** Escribir pruebas de posición, radio y fuerza de las perturbaciones del usuario. **RF:** RF-3. **Hecho cuando:** las pruebas exigen posición correspondiente, radio `0.03` y fuerza `0.01`.
- [ ] **S03-T02** Escribir pruebas de extremo a extremo para pulsar y arrastrar sobre el agua. **RF:** RF-3, RF-9. **Hecho cuando:** ejercitan la aplicación real con ratón y tacto compatible y exigen cambio observable.
- [ ] **S03-T03** Traducir posiciones de entrada a puntos de la superficie. **RF:** RF-3. **Hecho cuando:** los puntos de prueba corresponden a la referencia y no dependen de controles de interfaz dentro de la simulación.
- [ ] **S03-T04** Conectar pulsación y arrastre con perturbaciones y actualización de escena. **RF:** RF-2, RF-3, RF-9. **Hecho cuando:** cada interacción aplicable crea ondas visibles con los parámetros probados.
- [ ] **S03-T05** Ejecutar aceptación de extremo a extremo con ambas entradas disponibles. **RF:** RF-3, RF-9. **Hecho cuando:** pulsación y arrastre reales producen ondas; si no puede probarse tacto real, queda pendiente.
- [ ] **S03-T06** Ejecutar regresión S01–S02 y comprobaciones aplicables. **RF:** RF-1, RF-2, RF-11. **Hecho cuando:** inicio, errores de assets y evolución siguen pasando.
- [ ] **S03-T07** Verificar criterios e integración entrada → simulación → imagen. **RF:** RF-2, RF-3, RF-9. **Hecho cuando:** el efecto observable corresponde a la interacción; el comportamiento en pausa queda pendiente de S05.

## S04 — Orbitar la cámara

**Objetivo:** explorar la escena arrastrando el fondo.  
**RF:** RF-5 y RF-9 parciales; RF-3 transversal. **Dependencias:** S03.  
**Resultado observable:** el fondo rota la cámara y el agua continúa generando ondas.

- [ ] **S04-T01** Escribir pruebas de dirección, sensibilidad y límites de cámara. **RF:** RF-5. **Hecho cuando:** comprueban el desplazamiento y la inclinación entre `-89.999°` y `89.999°`.
- [ ] **S04-T02** Escribir prueba de extremo a extremo de arrastre del fondo y prioridad agua/fondo. **RF:** RF-3, RF-5, RF-9. **Hecho cuando:** exige cambio de vista en el fondo y ondas al iniciar sobre el agua.
- [ ] **S04-T03** Clasificar el inicio del arrastre entre agua y fondo. **RF:** RF-3, RF-5. **Hecho cuando:** se conserva el modo elegido durante el arrastre y pasan los casos de selección del slice.
- [ ] **S04-T04** Conectar el movimiento del fondo con los ángulos y la imagen. **RF:** RF-5, RF-9. **Hecho cuando:** dirección, sensibilidad y límites pasan las pruebas.
- [ ] **S04-T05** Ejecutar aceptación de extremo a extremo de órbita y límites. **RF:** RF-5, RF-9. **Hecho cuando:** la vista cambia en la aplicación y no supera los límites.
- [ ] **S04-T06** Ejecutar regresión S01–S03 y comprobaciones aplicables. **RF:** RF-1, RF-2, RF-3, RF-11. **Hecho cuando:** siguen pasando inicio, errores y ondas.
- [ ] **S04-T07** Verificar criterios e integración entrada → cámara → imagen. **RF:** RF-3, RF-5. **Hecho cuando:** agua y fondo no interfieren; esfera y pausa quedan para S06 y S05 respectivamente.

## S05 — Pausar y seguir explorando

**Objetivo:** pausar y reanudar sin perder las interacciones disponibles.  
**RF:** RF-6 parcial; RF-3 y RF-5 en pausa. **Dependencias:** S04.  
**Resultado observable:** espacio alterna la ejecución; agua y cámara pueden modificarse en pausa sin avance automático.

- [ ] **S05-T01** Escribir pruebas de transición, tiempo y ausencia de avance en pausa. **RF:** RF-6. **Hecho cuando:** fallan si el agua avanza automáticamente o si espacio no alterna el estado.
- [ ] **S05-T02** Escribir prueba de extremo a extremo de pausa, agua, cámara y reanudación. **RF:** RF-3, RF-5, RF-6. **Hecho cuando:** exige imagen actualizada por interacción sin avance automático.
- [ ] **S05-T03** Conectar espacio con el estado de pausa y el avance de simulación. **RF:** RF-6. **Hecho cuando:** la evolución se detiene y reanuda según las pruebas.
- [ ] **S05-T04** Actualizar la imagen por interacción de agua o cámara sin avanzar el tiempo. **RF:** RF-3, RF-5, RF-6. **Hecho cuando:** ambas acciones producen el resultado visual previsto en pausa.
- [ ] **S05-T05** Ejecutar aceptación de extremo a extremo del ciclo de pausa. **RF:** RF-3, RF-5, RF-6. **Hecho cuando:** pausa, espera, interacciones y reanudación pasan en la aplicación real.
- [ ] **S05-T06** Ejecutar regresión S01–S04 y comprobaciones aplicables. **RF:** RF-1, RF-2, RF-3, RF-5, RF-9, RF-11. **Hecho cuando:** las funcionalidades anteriores siguen pasando.
- [ ] **S05-T07** Verificar criterios e integración teclado/entrada → pausa → imagen. **RF:** RF-3, RF-5, RF-6. **Hecho cuando:** las dos interacciones no avanzan la simulación; esfera y luz se completan en slices posteriores.

## S06 — Mover la esfera

**Objetivo:** arrastrar la esfera y desplazar el agua.  
**RF:** RF-4; RF-9 parcial y RF-6 transversal. **Dependencias:** S05.  
**Resultado observable:** esfera arrastrable con límites originales, prioridad sobre agua y respuesta del agua, también en pausa.

- [ ] **S06-T01** Escribir pruebas de prioridad esfera/agua/fondo y límites de arrastre. **RF:** RF-4, RF-9. **Hecho cuando:** comprueban prioridad y los rangos X/Z `[-0.75, 0.75]` y Y `[-0.75, 10]`.
- [ ] **S06-T02** Escribir pruebas de extremo a extremo de arrastre, liberación y pausa. **RF:** RF-4, RF-6. **Hecho cuando:** exigen posición visible, respuesta del agua al avanzar y comportamiento de velocidad de la referencia.
- [ ] **S06-T03** Integrar la selección prioritaria y movimiento limitado de la esfera. **RF:** RF-4, RF-9. **Hecho cuando:** la esfera responde al arrastre sin activar agua o cámara por error.
- [ ] **S06-T04** Conectar desplazamiento de esfera y agua, incluida velocidad durante arrastre. **RF:** RF-4. **Hecho cuando:** se cumplen los límites y el desplazamiento del agua al avanzar.
- [ ] **S06-T05** Actualizar la posición visible durante pausa sin avanzar física ni agua. **RF:** RF-4, RF-6. **Hecho cuando:** la escena refleja el arrastre y la simulación permanece pausada.
- [ ] **S06-T06** Ejecutar aceptación de extremo a extremo en ejecución y pausa. **RF:** RF-4, RF-6, RF-9. **Hecho cuando:** selección, movimiento, liberación y respuesta del agua pasan.
- [ ] **S06-T07** Ejecutar regresión S01–S05 y comprobaciones aplicables. **RF:** RF-1–RF-6, RF-9, RF-11. **Hecho cuando:** agua, cámara, pausa e inicio siguen pasando.
- [ ] **S06-T08** Verificar criterios e integración entrada → esfera → agua → imagen. **RF:** RF-4, RF-6, RF-9. **Hecho cuando:** el comportamiento completo de RF-4 está demostrado sin interferir con las otras prioridades.

## S07 — Esfera con gravedad

**Objetivo:** activar y desactivar la física de la esfera.  
**RF:** RF-7; RF-4 y RF-6 transversales. **Dependencias:** S06.  
**Resultado observable:** G alterna gravedad, flotabilidad, resistencia y rebote; la física no avanza en pausa.

- [ ] **S07-T01** Escribir pruebas de estado inicial, gravedad y fuerzas con tiempo controlado. **RF:** RF-7. **Hecho cuando:** comprueban física inicialmente desactivada y valores y reglas de la referencia.
- [ ] **S07-T02** Escribir pruebas de extremo a extremo para G, rebote, arrastre y pausa. **RF:** RF-4, RF-6, RF-7. **Hecho cuando:** exigen respuesta observable y ausencia de avance físico en pausa.
- [ ] **S07-T03** Conectar G con la activación y desactivación de física. **RF:** RF-7. **Hecho cuando:** el estado cambia con cada pulsación y empieza desactivado.
- [ ] **S07-T04** Aplicar gravedad, flotabilidad y resistencia según la referencia. **RF:** RF-7. **Hecho cuando:** los casos controlados de movimiento pasan.
- [ ] **S07-T05** Aplicar rebote del fondo y coordinación con arrastre y pausa. **RF:** RF-4, RF-6, RF-7. **Hecho cuando:** el rebote usa factor `0.7` y la pausa detiene la física.
- [ ] **S07-T06** Ejecutar aceptación de extremo a extremo de la física. **RF:** RF-4, RF-6, RF-7. **Hecho cuando:** alternancia, caída, rebote y pausa pasan en la aplicación.
- [ ] **S07-T07** Ejecutar regresión S01–S06 y comprobaciones aplicables. **RF:** RF-1–RF-6, RF-9, RF-11. **Hecho cuando:** los comportamientos anteriores siguen pasando.
- [ ] **S07-T08** Verificar criterios e integración teclado → física → agua → imagen. **RF:** RF-4, RF-6, RF-7. **Hecho cuando:** las fuerzas y su efecto observable cumplen la referencia.

## S08 — Luz y efectos originales

**Objetivo:** reproducir la apariencia original y permitir ajustar la luz.  
**RF:** RF-2 restante y RF-8; RF-1, RF-4 y RF-5 transversales. **Dependencias:** S07.  
**Resultado observable:** efectos originales y dirección de luz controlada con L, incluso en pausa.

- [ ] **S08-T01** Escribir pruebas de comportamiento de L con cámara y pausa. **RF:** RF-8. **Hecho cuando:** exigen orientación según vista, actualización con L sostenida y cáusticas en pausa.
- [ ] **S08-T02** Escribir pruebas de extremo a extremo de luz y cambios en la escena. **RF:** RF-2, RF-8. **Hecho cuando:** cubren agua, esfera, cámara y luz en la aplicación real.
- [ ] **S08-T03** Integrar reflejos y refracciones con valores y assets originales. **RF:** RF-2. **Hecho cuando:** los efectos responden al agua y a la cámara según los escenarios de prueba.
- [ ] **S08-T04** Integrar oclusión ambiental y sombras suaves. **RF:** RF-2. **Hecho cuando:** responden a escena y esfera conforme a la referencia.
- [ ] **S08-T05** Integrar cáusticas y su actualización ante cambios de agua, esfera y luz. **RF:** RF-2, RF-8. **Hecho cuando:** los escenarios previstos muestran sus cambios, incluida pausa.
- [ ] **S08-T06** Conectar L con la dirección de vista y la actualización de la escena. **RF:** RF-5, RF-8. **Hecho cuando:** las pruebas de pulsación, sostenimiento y pausa pasan.
- [ ] **S08-T07** Ejecutar aceptación de extremo a extremo y comparación visual del usuario. **RF:** RF-1, RF-2, RF-8. **Hecho cuando:** las pruebas pasan y el usuario valida apariencia y comportamiento frente a la referencia con resolución y controles equivalentes.
- [ ] **S08-T08** Ejecutar regresión S01–S07 y comprobaciones aplicables. **RF:** RF-1–RF-7, RF-9, RF-11. **Hecho cuando:** no se pierde comportamiento anterior.
- [ ] **S08-T09** Verificar criterios e integración estado → efectos → ventana. **RF:** RF-2, RF-8. **Hecho cuando:** todos los efectos y estados de L se verifican; cualquier diferencia inevitable cuenta con aprobación previa.

## S09 — Ayuda, enlaces y ventana

**Objetivo:** consultar la ayuda y usar la demo con distintos tamaños de ventana.  
**RF:** RF-9 y RF-10; RF-11 parcial. **Dependencias:** S08.  
**Resultado observable:** ayuda original con enlaces provisionales, errores de apertura y vista adaptable; entrada y casos de foco según la referencia.

- [ ] **S09-T01** Escribir pruebas de ayuda, enlaces y fallo de apertura. **RF:** RF-9, RF-11. **Hecho cuando:** exigen textos y atribuciones originales, `https://example.com`, navegador predeterminado y mensaje con causa disponible.
- [ ] **S09-T02** Escribir pruebas de extremo a extremo de ayuda y redimensionamiento. **RF:** RF-9, RF-10. **Hecho cuando:** cubren interacción con ayuda sin arrastre, cambio de tamaño en pausa y ejecución, minimización y ventana pequeña.
- [ ] **S09-T03** Escribir pruebas de entrada táctil y casos de foco/contacto frente a la referencia. **RF:** RF-9. **Hecho cuando:** cubren un contacto, contactos múltiples, cancelación, pérdida de foco y puntero fuera de ventana.
- [ ] **S09-T04** Mostrar ayuda con textos, atribuciones y espacio correspondientes a la referencia. **RF:** RF-9, RF-10. **Hecho cuando:** se ven los contenidos y la ayuda no inicia arrastres de escena.
- [ ] **S09-T05** Conectar enlaces provisionales y comunicar fallos de apertura. **RF:** RF-9, RF-11. **Hecho cuando:** cada enlace solicita abrir el destino aprobado y un fallo produce mensaje con contexto.
- [ ] **S09-T06** Integrar cambio de tamaño, minimización y actualización en pausa. **RF:** RF-10. **Hecho cuando:** vista y ayuda siguen el comportamiento equivalente de referencia sin imponer tamaño mínimo nuevo.
- [ ] **S09-T07** Integrar los casos de entrada táctil, foco y contactos especificados. **RF:** RF-9. **Hecho cuando:** su comportamiento coincide con la referencia y la pérdida de foco por sí sola no cancela el arrastre.
- [ ] **S09-T08** Ejecutar aceptación de extremo a extremo en Windows, incluidos enlaces y tacto compatible. **RF:** RF-9, RF-10, RF-11. **Hecho cuando:** pasan los escenarios reales; si no puede verificarse tacto real, la tarea queda pendiente.
- [ ] **S09-T09** Ejecutar regresión S01–S08 y comprobaciones aplicables. **RF:** RF-1–RF-9, RF-11. **Hecho cuando:** los controles, efectos, errores y pausa siguen funcionando.
- [ ] **S09-T10** Verificar criterios e integración ayuda/entrada → escena → ventana y navegador. **RF:** RF-9, RF-10. **Hecho cuando:** no hay arrastres desde la ayuda y la escena se adapta también en pausa.

## S10 — Continuidad y errores de la simulación completa

**Objetivo:** ofrecer la alternativa gráfica y comunicar errores sin presentar una simulación rota.  
**RF:** RF-11 restante; RF-1–RF-10 transversales. **Dependencias:** S09.  
**Resultado observable:** alternativa con comportamiento equivalente, errores claros y última imagen congelada cuando corresponde; ambas rutas cumplen la aceptación de rendimiento.

- [ ] **S10-T01** Escribir pruebas de selección de alternativa y fallo de ambas rutas. **RF:** RF-11. **Hecho cuando:** exigen alternativa ante capacidad insuficiente y mensaje con causa si tampoco funciona.
- [ ] **S10-T02** Escribir pruebas de pérdida de assets y fallos gráficos posteriores. **RF:** RF-11. **Hecho cuando:** distinguen recurso aún utilizable de inutilizable y exigen imagen congelada y bloqueo cuando corresponde.
- [ ] **S10-T03** Escribir pruebas de precedencia de errores, ventana y consola. **RF:** RF-11. **Hecho cuando:** cubren las reglas específicas de assets y los fallos recuperables y de la referencia con contexto.
- [ ] **S10-T04** Escribir pruebas de extremo a extremo de las dos rutas y fallos controlados. **RF:** RF-1–RF-11. **Hecho cuando:** ejecutan la aplicación real sin añadir un control de usuario no previsto.
- [ ] **S10-T05** Integrar selección de la alternativa gráfica y activación controlada para pruebas. **RF:** RF-11. **Hecho cuando:** la aplicación usa la alternativa bajo la condición prevista y puede verificarse en el equipo del usuario.
- [ ] **S10-T06** Conectar la alternativa con la escena completa y sus controles. **RF:** RF-1–RF-10, RF-11. **Hecho cuando:** los escenarios funcionales y visuales aplicables funcionan también en esa ruta.
- [ ] **S10-T07** Gestionar fallo de ambas rutas y errores gráficos posteriores al inicio. **RF:** RF-11. **Hecho cuando:** se muestra la causa y, tras iniciar, se conserva la última imagen y se bloquea la escena.
- [ ] **S10-T08** Gestionar pérdida de assets durante la ejecución. **RF:** RF-11. **Hecho cuando:** se continúa mientras el asset sea utilizable y, si no, se identifica y se congela la escena con interacciones bloqueadas.
- [ ] **S10-T09** Integrar comunicación de los demás errores en ventana o consola. **RF:** RF-11. **Hecho cuando:** se aplican la precedencia aprobada y el contexto exigido sin ocultar fallos recuperables.
- [ ] **S10-T10** Ejecutar aceptación de extremo a extremo de ambas rutas y errores. **RF:** RF-1–RF-11. **Hecho cuando:** pasan las pruebas reales de selección, fallos, recursos y continuidad en Windows.
- [ ] **S10-T11** Medir rendimiento de la ruta normal en el equipo del usuario. **RF:** RF-1–RF-8; rendimiento. **Hecho cuando:** a 1920 × 1080, durante un minuto con agua, esfera y cámara en movimiento, cada intervalo consecutivo de un segundo alcanza al menos 30 FPS.
- [ ] **S10-T12** Medir rendimiento de la alternativa en el equipo del usuario. **RF:** RF-11; rendimiento. **Hecho cuando:** la alternativa cumple la misma prueba de 30 FPS en cada intervalo de un segundo.
- [ ] **S10-T13** Obtener validación visual del usuario para ambas rutas. **RF:** RF-1–RF-8, RF-11. **Hecho cuando:** el usuario compara con la referencia a la misma resolución y controles y acepta apariencia y comportamiento; no se exige igualdad píxel por píxel.
- [ ] **S10-T14** Ejecutar regresión integral S01–S09 y comprobaciones aplicables. **RF:** RF-1–RF-11. **Hecho cuando:** todas las pruebas automatizables, formato y análisis pasan en las rutas aplicables.
- [ ] **S10-T15** Verificar criterios e integración completa, incluida la alternativa. **RF:** RF-1–RF-11. **Hecho cuando:** todos los criterios de la spec y casos límite están verificados en el equipo del usuario; si falla la alternativa o el rendimiento, la entrega permanece bloqueada.

## Cobertura de requisitos

| RF | Slices que lo cubren |
|---|---|
| RF-1 | S02, S08, S10 |
| RF-2 | S02, S03, S06–S08, S10 |
| RF-3 | S03, S05, S09–S10 |
| RF-4 | S06–S08, S10 |
| RF-5 | S04–S05, S08–S10 |
| RF-6 | S05–S08, S10 |
| RF-7 | S07, S10 |
| RF-8 | S08, S10 |
| RF-9 | S03–S04, S06, S09–S10 |
| RF-10 | S09–S10 |
| RF-11 | S01–S02, S09–S10 |

La matriz cubre RF-1 a RF-11. Las menciones transversales indican regresión e integración, no finalización anticipada del requisito completo.
