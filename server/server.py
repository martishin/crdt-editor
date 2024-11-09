import asyncio
import json
import os
from fastapi import FastAPI, WebSocket, WebSocketDisconnect
from crdt_lww import PyLWWElementDictionary as LWWElementDictionary, PyTimestamp as Timestamp

app = FastAPI()
dictionary = LWWElementDictionary()
connected_clients = set()
DATA_FILE = "data.json"

# Load existing data from file on startup
def load_data():
    if os.path.exists(DATA_FILE):
        with open(DATA_FILE, "r") as file:
            data = json.load(file)
            for key, value in data.items():
                dictionary.add(key, value, Timestamp.now())

# Save current data to file
def save_data():
    data = {}
    for key in dictionary.keys():
        result = dictionary.lookup(key)
        if result is not None:
            data[key] = result
    with open(DATA_FILE, "w") as file:
        json.dump(data, file)

# Load data at startup
load_data()

@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    connected_clients.add(websocket)

    try:
        # Send the current dictionary state to the new client
        initial_data = {key: dictionary.lookup(key) for key in dictionary.keys() if dictionary.lookup(key) is not None}
        await websocket.send_text(json.dumps({"action": "initial_data", "data": initial_data}))

        while True:
            data = await websocket.receive_text()
            message = json.loads(data)
            action = message.get("action")
            key = message.get("key")
            value = message.get("value")
            timestamp = Timestamp.now()

            if action == "add":
                dictionary.add(key, value, timestamp)
                save_data()  # Save data after update
            elif action == "remove":
                dictionary.remove(key)
                save_data()  # Save data after update
            elif action == "lookup":
                result = dictionary.lookup(key)
                await websocket.send_text(json.dumps({"key": key, "value": result}))
                continue  # Skip broadcasting

            # Broadcast the update to all connected clients
            update = {
                "action": action,
                "key": key,
                "value": value if action == "add" else None,
                "timestamp": timestamp.value(),
            }
            await broadcast(json.dumps(update))
    except WebSocketDisconnect:
        print("Client disconnected")
    except Exception as e:
        print(f"Error: {e}")
    finally:
        connected_clients.remove(websocket)

async def broadcast(message: str):
    to_remove = set()
    for client in connected_clients:
        try:
            await client.send_text(message)
        except WebSocketDisconnect:
            to_remove.add(client)

    # Clean up disconnected clients
    connected_clients.difference_update(to_remove)
