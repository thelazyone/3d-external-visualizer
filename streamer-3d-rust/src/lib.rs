use tokio::runtime::Runtime;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use std::borrow::Cow;
use serde_json::json;
use std::sync::mpsc;
use futures::{SinkExt, StreamExt};



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


// Websocket functions:

async fn send_mesh_ws(
    mut websocket: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    model_data: String,
    tx: mpsc::Sender<()>
) -> Result<(), Box<dyn std::error::Error>> {

    websocket.send(Message::Text(model_data)).await?; // Use send method here

    let mut ack = None;
    while ack.is_none() {
        if let Some(Ok(msg)) = websocket.next().await { // Use the next method here
            if msg.is_text() && msg.to_text().unwrap() == "ACK" {
                ack = Some(());
            }
        }
    }

    let close_frame = CloseFrame {
        code: CloseCode::Normal,
        reason: Cow::Borrowed(""),
    };
    websocket.close(Some(close_frame)).await?; // Use close method with a CloseFrame

    tx.send(()).unwrap(); 
    Ok(())
}


async fn start_server_ws(model_data: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stop_server = false;

    // Create a TCP listener
    let listener = TcpListener::bind("localhost:8765").await?;
    while !stop_server {
        // Accept an incoming TCP connection
        if let Ok((stream, _)) = listener.accept().await {
            // Upgrade the TCP connection to a WebSocket connection
            let websocket = accept_async(stream).await?;

            // Create a channel for sending messages
            let (tx, rx) = mpsc::channel();

            // Send the model data to the client
            send_mesh_ws(websocket, model_data.clone(), tx).await?;

            // Receive the signal (whether "ACK" received or not)
            if rx.recv().is_ok() {
                stop_server = true;
            }
        }
    }

    Ok(())
}


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
