# 3d-external-visualizer
A 3D external visualizer for general purpose 3D projects.

The main goal of the project is to provide a basic visualizer for 3D files, which runs independently from any specific 3D project.

Currently the visualizer is implemented in js and the user-side (technically a server) utility is in python. However, since everything is handled through WebSockets, one of the first future steps would be to reimplement the visualizer in RUST-WASM and to implement another user-side library in RUST as well.

A better description of the interface will follow, for now it can be inferred from the model_server.py.

## Running locally
Aside from building the cargo projects, you can set up the viewer-3d-js with a `python -m http.server -b 127.0.0.1 8080` command.

## Simple usage: 
* The visualizer is accessible through this live demo: https://test.thelazyforger.com/3d-visualizer/ or by using "parcel" to package the js code to run in localhost. Follow the readme.txt inside /js_viewer. Expect the visualizer to be a pitch black screen, it's waiting for any input from one of the examples (see below).
* Sample "senders" of 3D meshes are load_cube.py and load_stl.py (which requires "susan.stl"). Both depend on model_server.py, which is the utility that handles both the connection and the serialization of the files. 

## notes: 
Currently the sending is done through a VERY basic json format. It's not optimized and it could be serialized in binary, but as a PoC it's already something.

## Next Steps:
* Move from Websocket to Grpc. Binary forever!
* Implementing a RUST library that does the same of the python one.
* Add a cfg file with address and port for the senders.
* Find a faster way to have a real time update. Simple solution: not closing the server, but having it running async instead. :D 
* Test some heavier loads
* Connect the RUST library with https://github.com/dima634/baby_shark and see the update in mesh real-time without the need of an intermediate file being written.
* Similarly, after re-writing Lazy-Tree (https://github.com/thelazyone/lazy-tree) using the visualizer for it.
