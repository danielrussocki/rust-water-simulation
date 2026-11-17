# Water Simulation

Los seis JPEG originales están en `assets/`. Ejecute `cargo run` desde la raíz
del proyecto: la aplicación busca los recursos en `assets/`, relativo al
directorio de trabajo.

## Presentación de S02

La ventana usa GPUI 0.2.2 tanto para cargar los JPEG locales originales como
para presentar una capa `canvas` nativa con el recipiente, la esfera y la malla
de agua. La malla recibe el estado de simulación independiente de GPUI y se
actualiza por cuadro; no se comparten texturas entre GPUI y otra API gráfica.
Esta vía no añade dependencias: GPUI ya era necesaria para la ventana de
RF-11, y `image` ya valida los JPEG. Los reflejos, refracciones, oclusión,
sombras y cáusticas siguen pendientes de S08.

## Procedencia de los recursos

La referencia es [webgl-water, commit 73eda8be](https://github.com/evanw/webgl-water/tree/73eda8be832b649367b25ea5690c1f0181bb56ad).
La referencia indica que WebGL Water es de Evan Wallace y está publicado bajo
la licencia MIT; el aviso correspondiente se conserva en
`assets/WEBGL-WATER-MIT.txt`.

La textura `tiles.jpg` procede de [078 - Bathroom Tile Texture](https://www.flickr.com/photos/zooboing/3682834083/),
de Patrick Hoesly (zooboing), bajo [Creative Commons Attribution 2.0](https://creativecommons.org/licenses/by/2.0/).
`assets/tiles.jpg` es la versión del repositorio de referencia, sin modificaciones
por este proyecto. El archivo `assets/tile-texture.jpg` fue aportado por el
usuario y no ha sido modificado por este proyecto. No se afirma que el autor
respalde este proyecto ni se imponen restricciones adicionales sobre las imágenes.

Los cinco JPEG del entorno proceden del mismo commit del repositorio de referencia,
bajo MIT según la información facilitada para este proyecto. Los archivos se han
incorporado sin modificaciones.

La prueba de ventana en Windows requiere una sesión de escritorio visible y un
idioma instalado para el OCR de Windows.
