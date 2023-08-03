extern crate streamer_3d_rust;
use std::fs::OpenOptions;
use stl_io;


fn main() {
    // Check if an argument is passed (file path)
    if let Some(path) = std::env::args().nth(1) {

        // reading a file, if a file is passed:
        let mut file = OpenOptions::new().read(true).open(path).unwrap();
        let input_mesh = stl_io::read_stl(&mut file);

        // Sending the mesh to the visualizer.
        let result = streamer_3d_rust::send_mesh_stl(&input_mesh.unwrap());
        match result {
            Ok(_) => println!("Mesh sent successfully!"),
            Err(err) => println!("Failed to send mesh: {}", err),
        }
    } else {
        println!("Please provide the path to the mesh file.");
    }
}