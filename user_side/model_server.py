import asyncio
import websockets
import json

stop_server = asyncio.Event()

async def send_model(websocket, path):
    global model_data
    await websocket.send(json.dumps(model_data))

    ack = None
    while ack != 'ACK':
        try:
            ack = await asyncio.wait_for(websocket.recv(), timeout=1.0)
        except asyncio.TimeoutError:
            continue

    await websocket.close()
    stop_server.set()

async def start_server(model):
    global model_data
    model_data = model
    server = await websockets.serve(send_model, 'localhost', 8765)

    await stop_server.wait()

    server.close()
    await server.wait_closed()

def run_server(model):
    asyncio.run(start_server(model))
