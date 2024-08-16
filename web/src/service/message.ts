import { baseApi } from "@/constants/axios.ts"

export const getMessages = async () => {
  const response = await baseApi.get("/messages")

  return response.data
}

export const sendMessages = async (content: string) => {
  const response = await baseApi.post("/messages", { content })

  return response.data
}
