# Game of Life

Laboratorio de renderizado en tiempo real implementando Conway's Game of Life con un framebuffer propio y la funcion `point`.

## Como ejecutar

```bash
cargo run
```

## Caracteristicas

- Framebuffer logico de `100x100` escalado a una ventana de `800x800`.
- Cada celda se pinta usando `point`.
- Lectura del estado de cada celda con `get_color`.
- Reglas completas de Conway: underpopulation, survival, overpopulation y reproduction.
- Bordes toroidales: las celulas que salen por una orilla interactuan con el lado opuesto.
- Patron inicial creativo con varios organismos clasicos.

## Organismos implementados

- Glider
- Blinker
- Toad
- Block
- Beehive
- Boat
- Tub
- Beacon
- Loaf
- Lightweight spaceship
- Middleweight spaceship
- Heavyweight spaceship
- Pulsar
- Pentadecathlon
- Gosper glider gun

## Demo

![Game of Life demo](demo.gif)


