# Constitución del proyecto

## Propósito y alcance

Reescribir en Rust con GPUI el simulador de agua de [webgl-water](https://github.com/evanw/webgl-water). El repositorio original es la referencia de comportamiento; la especificación aprobada define el alcance y los criterios verificables de la reescritura.

## Simplicidad y convenciones

- Se elegirá la solución más simple que satisfaga la especificación aprobada, sin anticipar necesidades no especificadas.
- El código Rust seguirá las convenciones estándar de formato y nomenclatura; los cambios deberán superar `cargo fmt` y `cargo clippy`.
- Toda dependencia nueva deberá tener una justificación vinculada a un requisito aprobado.

## Especificaciones y código

- Toda funcionalidad deberá estar contemplada en una especificación aprobada antes de implementarse.
- El código y las pruebas deberán ajustarse a los criterios de aceptación de esa especificación.
- Las ambigüedades o discrepancias se resolverán en la especificación antes de implementar el comportamiento afectado.
- Cualquier cambio de comportamiento requerirá actualizar y aprobar primero la especificación y después adaptar el código y las pruebas.

## Separación de responsabilidades

- La interfaz y las entradas, la simulación y el renderizado tendrán responsabilidades diferenciadas y límites explícitos.
- Las reglas de simulación no dependerán de los controles de la interfaz; la interfaz comunicará las acciones mediante esos límites.
- Cada módulo tendrá una responsabilidad identificable y no duplicará lógica de otro módulo.

## Calidad, pruebas y errores

- Cada requisito implementado tendrá pruebas vinculadas a sus criterios de aceptación.
- Las pruebas automatizables se ejecutarán junto con las comprobaciones de formato y análisis antes de dar por terminado un cambio.
- Los fallos recuperables se propagarán o comunicarán con contexto; no se ignorarán ni se ocultarán.
- No se usarán `unwrap` o `expect` en rutas de ejecución recuperables; los fallos irrecuperables deberán expresar su causa.
- Los defectos corregidos incorporarán una prueba de regresión que reproduzca el comportamiento incorrecto cuando sea automatizable.

## Desarrollo mediante slices verticales

- Cada incremento funcional ofrecerá un comportamiento observable y verificable.
- Se avanzará por funcionalidades completas, no por capas técnicas aisladas.
- Cada slice implementará únicamente las capas necesarias para satisfacer sus requisitos aprobados.
- Ningún slice se considerará terminado sin pruebas de extremo a extremo que verifiquen su comportamiento.
- Ningún slice podrá romper funcionalidades ya implementadas; deberá superar sus pruebas de regresión.
- No se implementarán comportamientos que no estén contemplados por la especificación aprobada.

## Compatibilidad y estabilidad

- Para cada comportamiento incluido en el alcance aprobado, la referencia será el simulador original y la equivalencia se verificará según los criterios de la especificación.
- Las diferencias necesarias por el cambio de tecnología deberán documentarse y aprobarse en la especificación antes de implementarse.
- Los comportamientos ya implementados y aprobados se preservarán mediante pruebas de regresión; cualquier cambio requerirá una modificación aprobada de la especificación.
