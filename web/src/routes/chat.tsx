import { createFileRoute } from "@tanstack/react-router"
import { ChatComposer } from "@/components/ChatComposer.tsx"
import { ChatSection } from "@/components/ChatSection.tsx"

export const Route = createFileRoute("/chat")({
  component: () => <Page />,
})

const Page = () => {
  return (
    <div className="relative flex flex-col w-full items-center justify-center">
      <ChatSection />
      <ChatComposer />
    </div>
  )
}
