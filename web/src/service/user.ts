import { baseApi } from "@/constants/axios.ts"
import { User } from "@/type/user.ts"

export const getUser = async () => {
  const response = await baseApi.get<User>("/api/v1/users/me")

  return response.data
}
