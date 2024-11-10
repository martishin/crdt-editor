import React, { useState, useEffect } from "react"
import { useWebSocket } from "../hooks/useWebSocket"

const DataEditor: React.FC = () => {
  const { messages, sendMessage, isConnected, initialData } = useWebSocket()
  const [key, setKey] = useState("")
  const [value, setValue] = useState("")
  const [data, setData] = useState<Record<string, string>>(initialData || {})

  // Apply initial data only on first render or when it updates
  useEffect(() => {
    if (initialData) setData(initialData)
  }, [initialData])

  // Update the local data state whenever a broadcast message is received
  useEffect(() => {
    messages.forEach((message) => {
      const { action, key, value } = message

      // Ensure `key` is defined and a string before using it
      if (typeof key === "string") {
        if (action === "add" || action === "update") {
          setData((prevData) => ({ ...prevData, [key]: value || "" }))
        } else if (action === "remove") {
          setData((prevData) => {
            const updatedData = { ...prevData }
            delete updatedData[key]
            return updatedData
          })
        }
      }
    })
  }, [messages])

  const handleAdd = () => sendMessage("add", key, value)
  const handleRemove = () => sendMessage("remove", key)

  return (
    <>
      <div className="app-container">
        <h2>CRDT Editor</h2>
        <div className="status">
          <span className="status-label">Status:</span>{" "}
          <span className={isConnected ? "connected" : "disconnected"}>
            {isConnected ? "Connected" : "Disconnected"}
          </span>
        </div>

        <div className="form">
          <input
            type="text"
            placeholder="Key"
            value={key}
            onChange={(e) => setKey(e.target.value)}
          />
          <input
            type="text"
            placeholder="Value"
            value={value}
            onChange={(e) => setValue(e.target.value)}
          />
        </div>
        <div className="buttons">
          <button onClick={handleRemove}>Remove</button>
          <button onClick={handleAdd}>Set</button>
        </div>

        <h3>Data</h3>
        <div className="messages">
          {Object.entries(data).map(([key, value]) => (
            <div key={key} className="message-item">
              <strong>{key}</strong>: {value}
            </div>
          ))}
        </div>
      </div>
      <footer className="footer">
        <p>
          Check out the CRDT algorithm code on{" "}
          <a
            href="https://github.com/martishin/react-python-rust-crdt-editor/blob/main/crdt/src/crdt.rs"
            target="_blank"
            style={{ color: "#888" }}
          >
            GitHub
          </a>
        </p>
      </footer>
    </>
  )
}

export default DataEditor
