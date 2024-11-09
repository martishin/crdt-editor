import asyncio
from fastapi import FastAPI, WebSocket
from crdt_lww import PyLWWElementDictionary as LWWElementDictionary, PyTimestamp as Timestamp
import json

app = FastAPI()
dictionary = LWWElementDictionary()

connected_clients = set()


@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    connected_clients.add(websocket)
    try:
        while True:
            data = await websocket.receive_text()
            message = json.loads(data)
            action = message.get("action")
            key = message.get("key")
            value = message.get("value")
            timestamp = Timestamp.now()

            if action == "add":
                dictionary.add(key, value, timestamp)
            elif action == "remove":
                dictionary.remove(key)
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
    except Exception as e:
        print(f"Error: {e}")
    finally:
        connected_clients.remove(websocket)


async def broadcast(message: str):
    await asyncio.gather(
        *[client.send_text(message) for client in connected_clients]
    )
