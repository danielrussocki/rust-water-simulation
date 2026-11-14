# Water Simulation

Los seis JPEG originales están en `assets/`. Ejecute `cargo run` desde la raíz
del proyecto: la aplicación busca los recursos en `assets/`, relativo al
directorio de trabajo. La escena todavía no está implementada.

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
