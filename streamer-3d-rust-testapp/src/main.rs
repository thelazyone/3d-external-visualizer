extern crate streamer_3d_rust;

fn main() {
    // Check if an argument is passed (file path)
    if let Some(path) = std::env::args().nth(1) {
        let input_mesh = streamer_3d_rust::load_test(path).unwrap();
        let result = streamer_3d_rust::send_mesh_stl(&input_mesh);
        match result {
            Ok(_) => println!("Mesh sent successfully!"),
            Err(err) => println!("Failed to send mesh: {}", err),
        }
    } else {
        println!("Please provide the path to the mesh file.");
    }
}