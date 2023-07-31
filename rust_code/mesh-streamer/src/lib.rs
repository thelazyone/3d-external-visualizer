use std::fs::OpenOptions;
use stl_io;
use tungstenite::{connect, Message};
use serde_json::json;

pub fn load_test(path: String) -> Result<stl_io::IndexedMesh, std::io::Error> {
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    stl_io::read_stl(&mut file)
}

pub struct InputMesh {
    pub vertices: Vec<[f32; 3]>,
    pub triangles: Vec<[usize; 3]>,
    // Normals will be added one day.
}
impl InputMesh {
    fn new () -> InputMesh {
        InputMesh {
            vertices : Vec::<[f32; 3]>::new(),
            triangles : Vec::<[usize; 3]>::new(),
        }
    }
}

pub fn send_mesh(input_mesh : &InputMesh) -> Result<(), std::io::Error> {

    let (mut socket, response) = connect("ws://localhost:8765").map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("WebSocket connection failed: {}", e))
    })?;

    // Create a JSON object from the vertices and triangles
    let model_data = json!({
        "vertices": input_mesh.vertices,
        "faces": input_mesh.triangles,
    });

    // Convert the JSON object to a string
    let message = serde_json::to_string(&model_data)?;

    // Send the message through the WebSocket connection
    socket.write_message(Message::Text(message))
        .map_err(|e| std::io::Error::new(
            std::io::ErrorKind::Other, format!("Failed to send message: {}", e)))?;

    Ok(())
}

pub fn send_mesh_stl(indexed_mesh : &stl_io::IndexedMesh) -> Result<(), std::io::Error> {

    // Converting the object in a serializable structure
    let mut input_mesh = InputMesh::new();
    input_mesh.vertices = Vec::<[f32; 3]>::with_capacity(indexed_mesh.vertices.len());
    input_mesh.triangles = Vec::<[usize; 3]>::with_capacity(indexed_mesh.faces.len());

    println!("Indexed mesh has {} vertexes ", indexed_mesh.vertices.len());

    for vertex in &indexed_mesh.vertices {
        input_mesh.vertices.push([
            vertex[0], 
            vertex[1],
            vertex[2]]);
    }
    for triangle in &indexed_mesh.faces {
        input_mesh.triangles.push([
            triangle.vertices[0],
            triangle.vertices[1],
            triangle.vertices[2]]);    
    }  

    send_mesh(&input_mesh)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let susan = load_test("./test/susan.stl".to_string());
        let result = send_mesh_stl(&susan.unwrap());
        assert!(result.is_ok(), "Failed to send mesh: {:?}", result.err().unwrap());
    }
}
