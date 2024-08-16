import { baseApi } from "@/constants/axios.ts";

export const signIn = async (email: string, password: string) => {
  const response = await baseApi.post<{
    accessToken: string;
    refreshToken: string;
  }>("/api/v1/auth/sign-in", {
    email,
    password,
  });

  return response.data;
};

export const getTokenByRefreshToken = async (token: string) => {
  const response = await baseApi.get<{ token: string; refresh_token: string }>(
    "/api/v1/auth/renew",
    {
      params: {
        refresh_token: token,
      },
    },
  );

  return response.data;
};
