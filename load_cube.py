import json
import asyncio
import websockets
import json
import time

def create_large_cube(n):
    # Create an array to store the vertices
    vertices = []
    # Create an array to store the faces
    faces = []

    # Calculate the size of each grid cell
    cell_size = 1.0 / n

    # Create the faces and vertices for each face of the cube
    for i in range(6):
        for j in range(n):
            for k in range(n):
                # Calculate the base index for this grid cell
                base_index = len(vertices)

                if i == 0:  # top face
                    vertices.extend([
                        [(j + 0) * cell_size - 0.5, 0.5, (k + 0) * cell_size - 0.5],
                        [(j + 1) * cell_size - 0.5, 0.5, (k + 0) * cell_size - 0.5],
                        [(j + 1) * cell_size - 0.5, 0.5, (k + 1) * cell_size - 0.5],
                        [(j + 0) * cell_size - 0.5, 0.5, (k + 1) * cell_size - 0.5]
                    ])
                elif i == 1:  # bottom face
                    vertices.extend([
                        [(j + 0) * cell_size - 0.5, -0.5, (k + 0) * cell_size - 0.5],
                        [(j + 1) * cell_size - 0.5, -0.5, (k + 0) * cell_size - 0.5],
                        [(j + 1) * cell_size - 0.5, -0.5, (k + 1) * cell_size - 0.5],
                        [(j + 0) * cell_size - 0.5, -0.5, (k + 1) * cell_size - 0.5]
                    ])
                elif i == 2:  # front face
                    vertices.extend([
                        [(j + 0) * cell_size - 0.5, (k + 0) * cell_size - 0.5, 0.5],
                        [(j + 1) * cell_size - 0.5, (k + 0) * cell_size - 0.5, 0.5],
                        [(j + 1) * cell_size - 0.5, (k + 1) * cell_size - 0.5, 0.5],
                        [(j + 0) * cell_size - 0.5, (k + 1) * cell_size - 0.5, 0.5]
                    ])
                elif i == 3:  # back face
                    vertices.extend([
                        [(j + 0) * cell_size - 0.5, (k + 0) * cell_size - 0.5, -0.5],
                        [(j + 1) * cell_size - 0.5, (k + 0) * cell_size - 0.5, -0.5],
                        [(j + 1) * cell_size - 0.5, (k + 1) * cell_size - 0.5, -0.5],
                        [(j + 0) * cell_size - 0.5, (k + 1) * cell_size - 0.5, -0.5]
                    ])
                elif i == 4:  # right face
                    vertices.extend([
                        [0.5, (j + 0) * cell_size - 0.5, (k + 0) * cell_size - 0.5],
                        [0.5, (j + 1) * cell_size - 0.5, (k + 0) * cell_size - 0.5],
                        [0.5, (j + 1) * cell_size - 0.5, (k + 1) * cell_size - 0.5],
                        [0.5, (j + 0) * cell_size - 0.5, (k + 1) * cell_size - 0.5]
                    ])
                elif i == 5:  # left face
                    vertices.extend([
                        [-0.5, (j + 0) * cell_size - 0.5, (k + 0) * cell_size - 0.5],
                        [-0.5, (j + 1) * cell_size - 0.5, (k + 0) * cell_size - 0.5],
                        [-0.5, (j + 1) * cell_size - 0.5, (k + 1) * cell_size - 0.5],
                        [-0.5, (j + 0) * cell_size - 0.5, (k + 1) * cell_size - 0.5]
                    ])

                # Add two faces for this grid cell
                faces.append([base_index, base_index + 1, base_index + 2])
                faces.append([base_index, base_index + 2, base_index + 3])

    # Return the vertices and faces as a dictionary
    return {'vertices': vertices, 'faces': faces}


async def send_model(websocket, path):
    global model_data
    await websocket.send(json.dumps(model_data))

    ack = None
    while ack != 'ACK':
        try:
            ack = await asyncio.wait_for(websocket.recv(), timeout=1.0)
        except asyncio.TimeoutError:
            # If no message is received within the timeout period, just continue waiting
            continue

    await websocket.close()  # Close the websocket after receiving the ack
    stop_server.set()  # Notify that the server can be stopped

stop_server = asyncio.Event()

async def start_server(model):
    global model_data
    model_data = model
    server = await websockets.serve(send_model, 'localhost', 8765)

    # Wait until the server is told to stop
    await stop_server.wait()

    server.close()  # Stop the server
    await server.wait_closed()  # Wait until the server has indeed stopped

loop = asyncio.get_event_loop()
cube_side = 100
large_cube = create_large_cube(cube_side)
t_before = time.time()
print("sending cube with side " + str(cube_side))
loop.run_until_complete(start_server(large_cube))
print("sending required " + str(time.time() - t_before) + " s.")