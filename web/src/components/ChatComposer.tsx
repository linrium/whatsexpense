import { Input } from "@/components/ui/input.tsx"
import { useMutation } from "@/hooks/useAwaited.ts"
import { sendMessages } from "@/service/message.ts"
import { KeyboardEvent, useRef } from "react"
import { useMessageActions } from "@/store/message.ts"
import { SendIcon } from "@/assets/svg/SendIcon.tsx"

export const ChatComposer = () => {
  const { addMessage } = useMessageActions()
  const { mutate } = useMutation(sendMessages, {
    onSuccess: (data) => {
      addMessage(data)
    },
  })
  const inputRef = useRef<HTMLInputElement>(null)

  const onSend = async (text: string) => {
    const v = text.trim()
    if (inputRef.current && v) {
      mutate(v)
      inputRef.current.focus()
      inputRef.current.value = ""
    }
  }

  const handleKeyDown = async (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key === "Enter" && inputRef?.current) {
      event.preventDefault()
      await onSend((event.target as HTMLInputElement).value)
    }
  }

  const handleSend = async () => {
    if (inputRef.current) {
      await onSend(inputRef.current.value)
    }
  }

  return (
    <div className="fixed bottom-0 left-0 right-0 flex items-center justify-center gap-2 p-4 bg-white">
      <Input ref={inputRef} onKeyDown={handleKeyDown} />
      <span className="p-1 hover:bg-blue-50 rounded-lg cursor-pointer">
        <SendIcon className="size-8" onClick={handleSend} />
      </span>
    </div>
  )
}
