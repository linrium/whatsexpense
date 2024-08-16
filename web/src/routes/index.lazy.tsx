import { createLazyFileRoute, Link } from "@tanstack/react-router"
import { Button } from "@/components/ui/button.tsx"

export const Route = createLazyFileRoute("/")({
  component: Index,
})

function Index() {
  return (
    <div className="p-2">
      <h3>Welcome Home!</h3>
      <Link to="/sign-in">
        <Button>Let's start</Button>
      </Link>
    </div>
  )
}
