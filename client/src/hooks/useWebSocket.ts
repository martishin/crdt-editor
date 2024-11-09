import { useEffect, useRef, useState } from "react"

const WS_URL = "ws://localhost:8000/ws"

type Message = {
  action: string
  key?: string
  value?: string
  timestamp?: number
  data?: Record<string, string>
}

export const useWebSocket = () => {
  const socket = useRef<WebSocket | null>(null)
  const [messages, setMessages] = useState<Message[]>([])
  const [isConnected, setIsConnected] = useState(false)
  const [initialData, setInitialData] = useState<Record<string, string>>({})

  useEffect(() => {
    socket.current = new WebSocket(WS_URL)

    socket.current.onopen = () => setIsConnected(true)
    socket.current.onclose = () => setIsConnected(false)

    socket.current.onmessage = (event) => {
      const data: Message = JSON.parse(event.data)
      if (data.action === "initial_data" && data.data) {
        setInitialData(data.data) // Set initial data when received
      } else {
        setMessages((prevMessages) => [...prevMessages, data])
      }
    }

    return () => {
      socket.current?.close()
    }
  }, [])

  const sendMessage = (action: string, key: string, value?: string) => {
    if (socket.current && socket.current.readyState === WebSocket.OPEN) {
      const message = JSON.stringify({ action, key, value })
      socket.current.send(message)
    }
  }

  return { messages, sendMessage, isConnected, initialData }
}
