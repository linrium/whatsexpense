import { createFileRoute, useNavigate } from "@tanstack/react-router"
import { z } from "zod"

import { Button } from "@/components/ui/button"
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form"
import { Input } from "@/components/ui/input"
import { toast } from "sonner"
import { useZodForm } from "@/hooks/useZodForm.ts"
import { signIn } from "@/service"
import { getToken, setToken } from "@/helper/token.ts"
import { useLayoutEffect } from "react"

export const Route = createFileRoute("/sign-in")({
  component: () => <InputForm />,
})

const schema = z.object({
  email: z.string().email({
    message: "Email must be valid.",
  }),
  password: z.string().min(6, {
    message: "Password must be at least 6 characters.",
  }),
})

type SignInFormProps = z.infer<typeof schema>

export function InputForm() {
  const navigate = useNavigate({ from: "/sign-in" })
  const form = useZodForm({
    schema: schema,
    defaultValues: {
      email: "",
      password: "",
    },
  })

  useLayoutEffect(() => {
    const token = getToken()
    if (token) {
      navigate({ to: "/chat" })
    }
  }, [])

  async function onSubmit(data: SignInFormProps) {
    try {
      const response = await signIn(data.email, data.password)
      setToken(response.accessToken)
      toast("You signed in successfully.")
      await navigate({ to: "/chat" })
    } catch (error) {
      console.log(error)
      toast("Something went wrong.")
    }
  }

  return (
    <div className="flex items-center justify-center h-screen w-full">
      <div className="flex-1 mx-4 p-4 border space-y-4 rounded-lg">
        <h2 className="text-2xl font-bold">Login</h2>
        <Form {...form} onSubmit={onSubmit}>
          <FormField
            control={form.control}
            name="email"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Email</FormLabel>
                <FormControl>
                  <Input autoCapitalize="none" placeholder="email" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="password"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Password</FormLabel>
                <FormControl>
                  <Input placeholder="password" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <Button className="w-full mt-4" type="submit">
            Submit
          </Button>
        </Form>
      </div>
    </div>
  )
}
