import { baseApi } from "@/constants/axios.ts"

export const getMessages = async () => {
  const response = await baseApi.get("/api/v1/messages")

  return response.data
}

export const sendMessages = async (content: string) => {
  const response = await baseApi.post("/api/v1/messages", { content })

  return response.data
}
