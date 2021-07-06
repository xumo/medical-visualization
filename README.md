# Technical Excercise - Rodrigo Torres

This project contains the submission for techinal excersise by Rodrigo Torres.

It consists of two parts: 
1. Native Language Task
2. Computer Graphics Task

## Native Language Task
To run this task:
```shell
cd native_language_task
cargo run -- -i assets/sinus.mhd -r assets/sinus.raw
```
To run the test

```shell
cargo run test
```
This will run a simple test for the byte conversion function.


## Computer Graphics Task
For this task all the files were extracted from the [Treejs repository](https://github.com/mrdoob/three.js/blob/dev/examples/webgl2_materials_texture3d.html) and to run a simple HTTP Server is needed.
```shel
cd computer_graphics_task
python3 -m http.server
```
And then go to (http://localhost:8000/)





