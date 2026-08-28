# Proyecto 1: Raycasting

Proyecto de Raycasting desarrollado en Rust utilizando la librería Raylib.

El objetivo del proyecto es implementar un nivel completamente jugable utilizando la técnica de ray casting vista en clase. El jugador puede recorrer diferentes laberintos en primera persona, sin atravesar paredes, hasta encontrar la salida.

## Características implementadas

- Renderizado 3D utilizando Ray Casting
- Dos niveles seleccionables
- Diferentes tipos de paredes
- Texturas para las paredes
- Colisiones con las paredes
- Movimiento del jugador
- Rotación horizontal utilizando el mouse
- Vista 2D para visualizar el funcionamiento de los rayos
- Cambio entre vista 2D y 3D
- Minimapa ubicado en una esquina de la pantalla
- Posición y dirección del jugador visibles en el minimapa
- Salida identificada dentro del nivel
- Sprite animado
- Z-buffer para ocultar correctamente el sprite detrás de las paredes
- Sistema de disparo
- Mira en el centro de la pantalla
- Detección de impacto sobre el enemigo
- Música de fondo
- Efectos de sonido para disparo, impacto y victoria
- Pantalla de bienvenida
- Selección entre múltiples niveles
- Pantalla de éxito al completar el nivel
- Contador de FPS
- Soporte para control/gamepad

## Controles

### Teclado y mouse

- `W` - avanzar
- `S` - retroceder
- `A` - moverse hacia la izquierda
- `D` - moverse hacia la derecha
- `Mouse` - rotar horizontalmente
- `Flecha izquierda / derecha` - rotación alternativa
- `Click izquierdo` - disparar
- `SPACE` - disparar
- `M` - cambiar entre vista 2D y vista 3D
- `1` - seleccionar Nivel 1
- `2` - seleccionar Nivel 2

## Ray Casting

Para generar la vista 3D se lanzan múltiples rayos desde la posición del jugador dentro de su campo de visión.

Cada rayo determina la distancia hasta la pared con la que colisiona. Esta distancia se utiliza para calcular la altura de la columna que se dibuja en pantalla.

También se realiza una corrección de distancia para evitar el efecto de ojo de pez.

## Texturas

Las paredes utilizan mapeo de texturas.

La posición del impacto del rayo se utiliza para calcular la coordenada horizontal de la textura (`tx`).

Para cada píxel vertical de la pared se calcula la coordenada correspondiente (`ty`) y se obtiene el color de la textura para dibujarlo en pantalla.

## Sprites

El enemigo se representa utilizando un sprite animado que siempre se muestra orientado hacia el jugador.

Para determinar su posición en pantalla se utiliza:

- Posición relativa entre el jugador y el sprite
- `atan2` para obtener el ángulo
- Normalización del ángulo
- Campo de visión del jugador
- Distancia euclidiana
- Escalamiento dependiendo de la distancia

Se utiliza un Z-buffer para comparar la distancia del sprite con la distancia de las paredes y evitar que el sprite pueda verse a través de ellas.

## Niveles

El proyecto incluye dos niveles:

- `maze1.txt`
- `maze2.txt`

El jugador debe recorrer el laberinto hasta encontrar la salida.

## Audio

El proyecto incluye música de fondo y diferentes efectos de sonido:

- `background.wav`
- `shot.wav`
- `hit.wav`
- `success.wav`

Los archivos de audio se encuentran dentro de la carpeta `assets`.

## Ejecución

Para ejecutar el proyecto es necesario tener Rust instalado.

Desde la carpeta principal del proyecto ejecutar:

```bash
cargo run
