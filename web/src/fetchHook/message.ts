import { useAwaited } from "@/hooks/useAwaited.ts"
import { getMessages } from "@/service/message.ts"
import {
  useMessageActions,
  useMessages as useStoreMessages,
} from "@/store/message.ts"
import { useEffect } from "react"

export const useMessages = () => {
  const { addMessage } = useMessageActions()
  const messages = useStoreMessages()
  const data = useAwaited(getMessages)

  useEffect(() => {
    data.data && addMessage(data.data)
  }, [data.loading])

  return { ...data, data: messages }
}
