// Re-export the public interface
pub use api::{send_mesh, send_mesh_stl};
pub use mesh::InputMesh;

mod api;
mod mesh;
mod websocket;

// The rest of the code related to tests can remain here or be moved to separate test modules.


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::OpenOptions;

    fn load_test(path: String) -> Result<stl_io::IndexedMesh, std::io::Error> {
        let mut file = OpenOptions::new().read(true).open(path).unwrap();
        stl_io::read_stl(&mut file)
    }

    #[test]
    fn it_works() {
        let susan = load_test("./test_files/susan.stl".to_string());
        let result = send_mesh_stl(&susan.unwrap());
        assert!(result.is_ok(), "Failed to send mesh: {:?}", result.err().unwrap());
    }

    #[test]
    fn speed_test() {

        // Currently this test shows that the updating speed is very slow. 
        // This is due to the fact that the connection is created and destroyed every time.
        
        let mut mutable_mesh = InputMesh::new();
        mutable_mesh.triangles = vec![[0, 1, 2]];
        mutable_mesh.vertices = vec![[0., 0., 0.], [0., 1., 0.], [1., 1., 1.]];
        for i in 1..5 
        {
            println!("testing iteration {}...", i);
            for coord  in mutable_mesh.vertices[1].iter_mut() {
                *coord *= 1.1;
            }

            let result = send_mesh(&mutable_mesh);
            assert!(result.is_ok(), "Failed to send mesh: {:?}", result.err().unwrap());
        }
    }
}
