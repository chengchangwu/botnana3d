# botnana3d: Three.js vs Kiss3d by Examples

To understancd the performance differences between rust and javascript for 3D graphics applications.
If the performance of native kiss3d examples is far better than threejs ones running with nodejs,
asmjs/wasm versions of kiss3d examples will be written to further compare against threejs ones
running by a browser.

## Benchmarks

ASUS X401A

|benchmark|three.js  |kiss3d  |threejs(chrome)|kiss3d(chrome)|
|---------|----------|--------|---------------|--------------|
|vector3  |2,465,392 |237     |               |              |

Amazon EC2 c4.xlarge

|benchmark|three.js  |kiss3d  |threejs(chrome)|kiss3d(chrome)|
|---------|----------|--------|---------------|--------------|
|vector3  |1,718,691 |384     |               |              |

Botnana Control (AM3357 800MHz)

|benchmark|three.js   |kiss3d   |threejs(chrome)|kiss3d(chrome)|
|---------|-----------|---------|---------------|--------------|
|vector3  |44,944,346 |5,541    |               |              |

