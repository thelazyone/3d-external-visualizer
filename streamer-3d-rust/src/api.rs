use crate::mesh::InputMesh;
use crate::websocket::start_server_ws;
use serde_json::json;
use tokio::runtime::Runtime;


pub fn send_mesh(input_mesh : &InputMesh) -> Result<(), std::io::Error> {

    // Create a JSON object from the vertices and triangles
    let model_data = json!({
        "vertices": input_mesh.vertices,
        "faces": input_mesh.triangles,
    });

    // Convert the JSON object to a string
    let message = serde_json::to_string(&model_data)?;

    // Sending the websocket here
    let rt = Runtime::new().unwrap();
    rt.block_on(start_server_ws(message)).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("Error sending Message: {}", e))
    })?;

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