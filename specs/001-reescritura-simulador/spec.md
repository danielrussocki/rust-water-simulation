# Especificación: reescritura del simulador de agua

**Estado:** aprobada.

## Contexto y objetivo

Reproducir en Windows 11 la experiencia de [webgl-water](https://github.com/evanw/webgl-water) como aplicación Rust con GPUI, conservando sus interacciones, comportamiento, apariencia y recursos originales. La referencia para esta especificación es el repositorio original en el commit [`73eda8be832b649367b25ea5690c1f0181bb56ad`](https://github.com/evanw/webgl-water/tree/73eda8be832b649367b25ea5690c1f0181bb56ad).

Se requiere fidelidad estricta: los valores controlados, colores y demás parámetros deben corresponder a los de la referencia y deben utilizarse los mismos assets. La apariencia debe ser casi idéntica y será validada visualmente por el usuario; no se exige igualdad píxel por píxel.

## Usuarios

- Personas que ejecutan en Windows 11 la demo y exploran la simulación de agua mediante sus controles.

## Historias de usuario

- Como usuario, quiero observar la escena y sus efectos originales para experimentar la demo en Windows 11.
- Como usuario, quiero interactuar con el agua, la esfera y la cámara para explorar cómo responde la escena.
- Como usuario, quiero pausar la simulación y controlar la gravedad y la dirección de la luz como en la demo original.
- Como usuario, quiero consultar las instrucciones y recibir los errores equivalentes a los originales.

## Requisitos funcionales y criterios de aceptación

### RF-1. Escena inicial

El sistema debe presentar la escena inicial de la referencia, con el agua, el recipiente, la esfera, el entorno y sus valores y apariencia originales.

- **Al iniciar** en un equipo Windows 11, el sistema deberá mostrar la escena con la posición inicial de la esfera, la cámara, la luz y el estado de controles de la referencia, salvo que se produzca un fallo contemplado en RF-11.
- **Al iniciar**, el sistema deberá generar las 20 perturbaciones iniciales con los radios y fuerzas originales; sus posiciones se elegirán independientemente y con distribución uniforme dentro de la superficie del agua, sin exigir coincidencia entre ejecuciones.

### RF-2. Simulación y apariencia del agua

El sistema debe reproducir la evolución del agua y los efectos visuales de la referencia, incluidos reflejos, refracciones, oclusión ambiental, sombras suaves y cáusticas.

- **Mientras** la simulación esté activa, el sistema deberá hacer evolucionar y amortiguar las ondas conforme a la referencia.
- **Cuando** cambien el agua, la esfera, la cámara o la luz, el sistema deberá mostrar los efectos visuales resultantes conforme a la referencia.
- **Al presentar** la escena, el sistema deberá utilizar los mismos assets, colores y valores que la referencia.

### RF-3. Interacción con el agua

El sistema debe permitir crear ondas mediante las interacciones de la referencia.

- **Cuando** el usuario pulse o arrastre sobre el agua, el sistema deberá generar perturbaciones en las posiciones correspondientes, con el radio y la fuerza de la referencia.
- **Mientras** la simulación esté pausada, **cuando** el usuario interactúe con el agua, el sistema deberá actualizar la imagen sin reanudar el transcurso de la simulación.

### RF-4. Movimiento de la esfera

El sistema debe permitir arrastrar la esfera y reproducir su interacción con el agua.

- **Cuando** el usuario arrastre la esfera, el sistema deberá desplazarla conforme al movimiento del puntero y a los límites de la referencia.
- **Cuando** la esfera se desplace y la simulación avance, el sistema deberá desplazar el agua a su alrededor como en la referencia.
- **Mientras** la simulación esté pausada, **cuando** el usuario arrastre la esfera, el sistema deberá mostrar su nueva posición sin reanudar la simulación.
- **Cuando** el usuario suelte la esfera después de arrastrarla, el sistema deberá aplicar el comportamiento de velocidad de la referencia.

### RF-5. Cámara

El sistema debe permitir orbitar la cámara alrededor de la escena como en la referencia.

- **Cuando** el usuario arrastre el fondo de la escena, el sistema deberá rotar la cámara en función del desplazamiento del puntero.
- **Mientras** se orbite la cámara, el sistema deberá respetar los límites de inclinación de la referencia.
- **Mientras** la simulación esté pausada, **cuando** se mueva la cámara, el sistema deberá actualizar la imagen sin reanudarla.

### RF-6. Pausa

El sistema debe permitir pausar y reanudar la simulación.

- **Cuando** el usuario pulse la barra espaciadora, el sistema deberá alternar entre pausa y ejecución.
- **Mientras** la simulación esté pausada, el sistema no deberá avanzar automáticamente el agua ni la física de la esfera; las interacciones permitidas deberán seguir funcionando como en la referencia.

### RF-7. Gravedad de la esfera

El sistema debe permitir activar y desactivar la física de la esfera mediante el control original.

- **Al iniciar**, el sistema deberá mantener desactivada la gravedad de la esfera.
- **Cuando** el usuario pulse G, el sistema deberá alternar la física de la esfera.
- **Mientras** la física esté activada y la simulación avance, el sistema deberá reproducir la gravedad, flotabilidad, resistencia del agua y rebote en el fondo de la referencia.

### RF-8. Dirección de la luz

El sistema debe permitir ajustar la dirección de la luz mediante el control original.

- **Cuando** el usuario accione L, el sistema deberá orientar la luz según la dirección de vista de la cámara, como en la referencia.
- **Mientras** L permanezca accionada y la vista cambie, el sistema deberá actualizar la dirección de la luz conforme a la referencia.
- **Mientras** la simulación esté pausada, **cuando** se accione L, el sistema deberá actualizar la imagen y las cáusticas sin reanudarla.

### RF-9. Ayuda y controles de entrada

El sistema debe ofrecer la información y los controles de la demo original.

- **Al mostrar** la demo, el sistema deberá presentar las instrucciones, descripción de características, atribuciones y textos de los enlaces de la referencia.
- **Cuando** el usuario interactúe con la ayuda o sus enlaces, el sistema no deberá interpretar esa interacción como un arrastre de la escena.
- **Cuando** el usuario active un enlace de la ayuda, el sistema deberá abrir `https://example.com` en el navegador predeterminado de Windows. Este destino es provisional para todos los enlaces.
- **Si** Windows no puede abrir un enlace de la ayuda, el sistema deberá mostrar un mensaje de error con la causa disponible.
- **Cuando** el usuario utilice ratón o una entrada táctil de un solo contacto compatible, el sistema deberá permitir las interacciones correspondientes de la referencia.
- **Al presentar** la ayuda en Windows, el sistema deberá conservar sus textos originales. Cualquier diferencia visual que se considere inevitable deberá consultarse con el usuario y aprobarse antes de implementar la parte afectada.

### RF-10. Tamaño de la ventana

El sistema debe adaptar la presentación al tamaño disponible conservando el comportamiento de la referencia.

- **Cuando** cambie el tamaño de la ventana, el sistema deberá reajustar la vista y actualizar la escena, incluso si la simulación está pausada.
- **Mientras** se muestre la ayuda, el sistema deberá reservarle espacio según el comportamiento visual de la referencia.
- **Cuando** la ventana se minimice o sea demasiado pequeña para mostrar cómodamente la escena y la ayuda, el sistema deberá conservar el comportamiento equivalente de la referencia, sin imponer un tamaño mínimo nuevo.

### RF-11. Compatibilidad gráfica y errores

El sistema debe mantener la simulación en dispositivos Windows 11 sin las capacidades gráficas habituales mediante una alternativa equivalente, y comunicar los demás fallos de manera equivalente a la referencia.

- **Si** el dispositivo no puede ejecutar los efectos originales con sus capacidades gráficas habituales, el sistema deberá utilizar una alternativa que conserve la apariencia y el comportamiento, aunque sea más lenta.
- **Si** tampoco es posible ejecutar esa alternativa, el sistema deberá mostrar en su ventana un mensaje con la causa y no presentar la simulación como operativa.
- **Si** faltan o están dañados assets originales, el sistema deberá mostrar un mensaje claro que enumere todos los assets afectados y detener el inicio de la simulación. Esta regla prevalece sobre el modo de comunicar ese fallo en la referencia.
- **Cuando** el usuario corrija los assets afectados y vuelva a abrir la aplicación, el sistema deberá intentar iniciar de nuevo; no se requiere un control de reintento en el mensaje de error.
- **Si** un asset deja de estar disponible después del inicio, el sistema deberá continuar la simulación mientras pueda utilizarlo.
- **Si** la simulación no puede continuar porque ya no puede utilizar un asset, el sistema deberá mostrar un error que lo identifique, mantener la última imagen congelada detrás del mensaje y bloquear las interacciones con la escena.
- **Si** se produce otro error gráfico no recuperable, el sistema deberá mostrar en su ventana un mensaje con la causa y no presentar la simulación como operativa.
- **Si** ocurre un error gráfico no recuperable después de iniciar la simulación, el sistema deberá mantener la última imagen congelada detrás del mensaje y bloquear las interacciones con la escena.
- **Si** ocurre un error que la referencia muestra en la demo, el sistema deberá mostrar un mensaje equivalente en la aplicación.
- **Si** un fallo solo se comunica en la consola en la referencia, el sistema deberá comunicarlo por la consola equivalente.
- **Si** un fallo recuperable solo se comunica en la consola de la referencia y no está sujeto a una regla específica de esta spec, bastará con comunicarlo en la consola equivalente con contexto, sin mostrar un mensaje en la ventana.
- **Si** se produce un fallo recuperable, el sistema deberá comunicarlo con contexto; no deberá ignorarlo ni ocultarlo.

## Valores fijos de referencia

Los siguientes valores utilizan las coordenadas y unidades de la referencia fijada:

- **Escena inicial (RF-1):** centro de la esfera `(-0.4, -0.75, 0.2)`, radio `0.25` y velocidad `(0, 0, 0)`; ángulos de cámara X `-25°` e Y `-200.5°`; dirección de luz inicial `(2, 2, -1)` normalizada. La simulación empieza activa y la física de la esfera desactivada.
- **Perturbaciones (RF-1 y RF-3):** las 20 perturbaciones iniciales tienen radio `0.03` y fuerza alternada `-0.01`, `0.01`, empezando por `-0.01`. Las generadas por el usuario tienen radio `0.03` y fuerza `0.01`.
- **Esfera (RF-4 y RF-7):** durante el arrastre, el centro se limita a `[-0.75, 0.75]` en X y Z y a `[-0.75, 10]` en Y. La gravedad es `(0, -4, 0)`; al rebotar en el fondo, la velocidad vertical resultante es su valor absoluto multiplicado por `0.7`. Durante las actualizaciones realizadas mientras se arrastra la esfera, su velocidad se establece en cero.
- **Cámara (RF-5):** la inclinación X está limitada a `[-89.999°, 89.999°]`; cada unidad de desplazamiento del puntero resta una unidad al ángulo correspondiente, como en la referencia.
- **Agua (RF-2):** cada actualización activa aplica dos pasos de evolución; en cada paso la velocidad se amortigua con el factor `0.995`. Una actualización cuyo intervalo sea superior a un segundo se omite.

Los demás valores fijos y reglas de la referencia siguen siendo exigibles aunque no estén enumerados aquí.

## Requisitos no funcionales

- **Plataforma:** para esta entrega, el entorno compatible es cualquier equipo Windows 11, sin restringir la tarjeta gráfica. Para aceptar esta entrega basta comprobar la aplicación en el equipo Windows 11 del usuario; la comprobación manual en otros dispositivos queda fuera de esta entrega. Si no es posible ejecutar la simulación ni siquiera mediante la alternativa gráfica, se aplica el error de RF-11.
- **Rendimiento:** en el equipo Windows 11 del usuario y a 1920 × 1080, después de iniciar la simulación, durante un minuto con movimientos de agua, esfera y cámara, cada intervalo consecutivo de un segundo debe alcanzar al menos 30 FPS. El mismo mínimo se aplica a la alternativa gráfica. Los 60 FPS son el objetivo ideal y los 120 FPS el objetivo de alto rendimiento; ninguno de estos dos objetivos es obligatorio para la aceptación.
- **Fidelidad:** deben conservarse valores controlados, colores, assets, interacciones y comportamiento de la referencia. El usuario validará visualmente que el resultado sea casi idéntico, usando la misma resolución y los mismos controles; no se exige igualdad píxel por píxel.
- **Comparación de la simulación:** los valores fijos de la referencia deberán ser iguales en la reescritura. No se exige igualdad de trayectorias o estados numéricos que dependan de valores no fijos; el usuario validará visualmente que las ondas y la esfera se comporten de forma casi idéntica mediante las mismas interacciones.
- **Valores no controlados:** si se identifica otro valor aleatorio o no controlado, se debe consultar al usuario y acordar su tratamiento antes de implementar el comportamiento afectado.
- **Estabilidad:** los comportamientos aceptados no deben sufrir regresiones.
- **Verificabilidad:** cada requisito debe contar con pruebas vinculadas a sus criterios de aceptación; cada incremento funcional debe verificarse de extremo a extremo.
- **Atribución:** deben conservarse las atribuciones de la demo y respetarse las condiciones aplicables a los assets originales.

## Casos límite

- **Arrastre ambiguo:** al iniciar sobre la esfera tiene prioridad moverla; si no se alcanza la esfera pero se apunta al agua, se crean ondas; en otro caso se orbita la cámara, como en la referencia.
- **Pérdida de foco durante un arrastre:** debe conservarse el comportamiento de la referencia; no se cancelará el arrastre automáticamente solo por perder el foco.
- **Puntero fuera de la ventana o cambios de contacto táctil:** al salir el puntero de la ventana, cancelarse un contacto o añadirse un segundo contacto, debe conservarse el comportamiento de la referencia.
- **Límites de movimiento:** la esfera no debe atravesar los límites establecidos en la referencia y la inclinación de la cámara debe permanecer dentro de los suyos.
- **Interacciones durante la pausa:** agua, esfera, cámara y luz pueden modificar la imagen sin hacer avanzar automáticamente la simulación.
- **Contactos táctiles múltiples:** deben conservarse las condiciones de inicio, movimiento y finalización de la interacción táctil de la referencia.
- **Intervalo de actualización superior a un segundo:** la actualización de simulación correspondiente debe omitirse como en la referencia.
- **Ondas iniciales aleatorias:** las posiciones pueden variar entre ejecuciones y entre la referencia y la reescritura; deben elegirse independientemente y con distribución uniforme dentro de la superficie del agua, respetando la cantidad y los parámetros originales.
- **Capacidad gráfica no disponible:** debe utilizarse la alternativa equivalente indicada en RF-11; si tampoco puede funcionar, debe comunicarse el error sin presentar la simulación como operativa.
- **Assets ausentes o dañados:** deben enumerarse todos los assets afectados en un mensaje claro y detenerse el inicio de la simulación.

## Fuera de alcance

- Funciones que no existan en el repositorio de referencia, incluidos guardar simulaciones, exportar imágenes y añadir ajustes nuevos.
- Compatibilidad con sistemas operativos y versiones de Windows distintos de Windows 11 en esta entrega.
- Verificación de compatibilidad en otros dispositivos Windows 11; el usuario podrá realizarla manualmente después de esta entrega.
- Cambios intencionales de aspecto, física, controles, recursos o comportamiento respecto a la referencia.

La alternativa de compatibilidad gráfica y el mensaje claro para assets ausentes o dañados de RF-11 son excepciones aprobadas respecto al manejo de fallos de la referencia; no añaden funciones de interacción ni alteran intencionalmente la apariencia o el comportamiento de la simulación. Los destinos provisionales de los enlaces de RF-9 también son una excepción aprobada respecto a la referencia. Toda diferencia visual que se considere inevitable en Windows deberá consultarse y aprobarse antes de implementar la parte afectada.

## Criterios de finalización

- Todos los requisitos funcionales y sus criterios de aceptación se verifican frente a la referencia fijada; el usuario valida visualmente la apariencia con la misma resolución y los mismos controles.
- Las pruebas de extremo a extremo y regresión aplicables pasan, sin romper funcionalidades previamente aceptadas.
- Se verifican los casos límite y el comportamiento de error incluido en el alcance.
- El usuario comprueba la aplicación en su equipo Windows 11; esta comprobación no demuestra compatibilidad universal.
- La alternativa gráfica de RF-11 se verifica en el equipo del usuario activándola bajo condiciones controladas, incluso si el equipo no la necesita normalmente.
- Las dudas abiertas que impiden verificar la equivalencia o determinar el comportamiento esperado se resuelven y aprueban antes de implementar lo afectado.

## Dudas abiertas

Ninguna identificada. Si aparece una ambigüedad o diferencia no prevista, deberá consultarse y resolverse antes de implementar el comportamiento afectado.
