import { baseApi } from "@/constants/axios.ts"

export const signIn = async (email: string, password: string) => {
  const response = await baseApi.post<{ token: string; refresh_token: string }>(
    "auth/sign-in",
    {
      email,
      password,
    },
  )

  return response.data
}

export const getTokenByRefreshToken = async (token: string) => {
  const response = await baseApi.get<{ token: string; refresh_token: string }>(
    "/auth/renew",
    {
      params: {
        refresh_token: token,
      },
    },
  )

  return response.data
}
