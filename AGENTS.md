# Guía permanente para agentes

## Proyecto y arquitectura

- El proyecto reescribe en Rust con GPUI el simulador de [webgl-water](https://github.com/evanw/webgl-water), referencia del comportamiento existente.
- `docs/constitution.md` contiene los principios obligatorios del proyecto y prevalece sobre esta guía ante cualquier discrepancia.
- La arquitectura debe separar interfaz y entradas, simulación y renderizado mediante límites explícitos. La simulación no debe depender de los controles de la interfaz.
- No se debe presumir que existan módulos o una arquitectura implementada: inspeccionar el repositorio antes de trabajar y respetar la estructura efectivamente existente.

## Tecnologías y convenciones

- Rust y GPUI son las tecnologías objetivo. Seguir las convenciones estándar de formato y nomenclatura de Rust.
- Elegir la solución más simple que cumpla los requisitos aprobados. Justificar cada dependencia nueva mediante un requisito aprobado.
- Mantener responsabilidades diferenciadas, evitar duplicar lógica y preservar los comportamientos aprobados.
- Propagar o comunicar los errores recuperables con contexto; no ocultarlos. No usar `unwrap` ni `expect` en rutas recuperables; explicar la causa de los fallos irrecuperables.

## Ejecución y verificaciones

- Inspeccionar los manifiestos y scripts disponibles antes de ejecutar comandos. Sin un manifiesto Cargo y una aplicación configurada, los siguientes comandos de Cargo son previstos, no comandos ya operativos del proyecto:
  - Ejecución: `cargo run`.
  - Pruebas y regresiones: `cargo test`, además de los comandos específicos de extremo a extremo que se establezcan.
  - Formato: `cargo fmt --check`.
  - Análisis: `cargo clippy`.
- Ejecutar las pruebas automatizables y las comprobaciones de formato y análisis antes de dar por terminado un cambio. Si alguna verificación no puede ejecutarse, comunicarlo y no afirmar que ha pasado.

## Specs, planes y tareas

- Antes de implementar, leer la constitución y la spec, el plan y las tareas activos. Si faltan, no están aprobados cuando corresponde, o son ambiguos, detener la implementación y solicitar aclaración o aprobación.
- La spec aprobada define el alcance y los criterios de aceptación; el código y las pruebas deben ajustarse a ella. El plan organiza su implementación y `tasks.md` define los slices funcionales y sus dependencias.
- No ampliar el alcance ni cambiar comportamientos sin actualizar y aprobar primero la spec. No modificar specs aprobadas silenciosamente: comunicar y someter a aprobación cualquier cambio propuesto antes de implementarlo.
- Resolver en la spec las ambigüedades y discrepancias antes de implementar el comportamiento afectado. Documentar y aprobar en ella las diferencias necesarias respecto a la referencia.
- Vincular cada requisito implementado con pruebas de sus criterios de aceptación. Añadir pruebas de regresión para los defectos corregidos cuando sean automatizables.

## Proceso obligatorio de implementación vertical

1. Seleccionar en `tasks.md` un slice funcional con comportamiento observable y verificable. No iniciarlo si tiene dependencias pendientes.
2. Identificar sus requisitos y criterios de aceptación aprobados. No organizar la ejecución por capas técnicas aisladas.
3. Escribir primero las pruebas que describen el comportamiento esperado, incluidas las de extremo a extremo necesarias para verificar el slice.
4. Implementar únicamente los componentes y capas necesarios para completar el slice activo, sin introducir comportamientos fuera de la spec.
5. Ejecutar las pruebas del slice, incluidas las de extremo a extremo, y las pruebas de regresión de la funcionalidad existente; ejecutar también las demás verificaciones aplicables.
6. Marcar la tarea como completada solo después de verificar sus criterios de aceptación y superar las comprobaciones requeridas. Si una verificación falla o no puede realizarse, mantenerla pendiente y comunicar el bloqueo.
