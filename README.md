# Escuela
Un programa para tener una base de datos de alumnos de una escuela 

El struct Escuela guarda el nombre, dueño y un vector de Alumno.

Alumno tiene campos para nombre, matrícula única (como ID), edad y si está activo (inscrito) o no.

Las instrucciones:

1. crear_escuela: Inicializa la cuenta con nombre y dueño.
2. agregar_alumno: Añade un alumno nuevo validando que no pases el límite máximo.
3. eliminar_alumno: Remueve alumno por matrícula buscando posición en el vector.
4. editar_alumno: Modifica los campos que recibas, con Option para actualizar sólo los necesarios.

Usamos PDA con seeds en todas las cuentas para mayor seguridad y manejo de direcciones programáticas.

La validación has_one = owner garantiza que solo el dueño pueda modificar la escuela y sus alumnos.

Este patrón es ideal para manejar listas dinámicas con control completo y seguridad on-chain en Solana con Ancho
