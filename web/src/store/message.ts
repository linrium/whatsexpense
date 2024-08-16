import { create } from "zustand"
import { Message } from "@/type/message.ts"

interface MessageState {
  message: Message[]
  addMessage: (message: Message[]) => void
}

const useMessageStore = create<MessageState>((set) => ({
  message: [] as Message[],
  addMessage: (message) =>
    set((state) => ({ message: [...message, ...state.message] })),
}))

export const useMessages = () => useMessageStore((state) => state.message)

export const useMessageActions = () => ({
  addMessage: useMessageStore((state) => state.addMessage),
})
