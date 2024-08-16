import axios, { AxiosRequestConfig, AxiosResponse } from "axios";
import { ENV } from "@/constants/env.ts";
import {
  getRefreshToken,
  getToken,
  removeRefreshToken,
  removeToken,
  setRefreshToken,
  setToken,
} from "@/helper/token.ts";

const redirectToSignIn = () => (document.location.href = "/sign-in");

export interface Config extends AxiosRequestConfig {
  baseUrl?: string;
}

const createAxiosInstant = (config?: Config) => {
  const instance = axios.create({
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
    // timeout: 15000, // 15s
    ...config,
    baseURL: config?.baseUrl,
  });

  instance.interceptors.request.use(
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-expect-error
    (config) => {
      const token = getToken();

      return {
        ...config,
        headers: {
          "Content-Type": "application/json",
          ...config.headers,
          Authorization: `Bearer ${token}`,
        },
      };
    },
    (error) => {
      console.log(error);
      return Promise.reject(error);
    },
  );

  instance.interceptors.response.use(
    (res) => res,
    async (err) => {
      const originalConfig = err.config!;

      if (err.response) {
        // Access Token was expired
        if (err.response.status === 401) {
          try {
            const refreshToken = getRefreshToken();
            if (!refreshToken) {
              throw new Error("no token");
            }

            const result = await instance.get<{
              token: string;
              refresh_token: string;
            }>("/auth/renew", {
              params: {
                refresh_token: refreshToken,
              },
            });

            if (!result.data) {
              throw new Error("no token");
            }

            setToken(result.data.token);
            setRefreshToken(result.data.refresh_token);

            return instance(originalConfig);
          } catch (e) {
            removeRefreshToken();
            removeToken();
            redirectToSignIn();
          }
        }
      }

      throw err;
    },
  );

  return instance;
};

export const requestInstant =
  (config?: Config) =>
  <T>(options: AxiosRequestConfig): Promise<AxiosResponse<T>> => {
    const client = createAxiosInstant(config);

    try {
      return client(options);
    } catch (error) {
      return Promise.reject(error);
    }
  };

const request = requestInstant({ baseUrl: ENV.baseUrl });

const get = <T = any>(url: string, options?: AxiosRequestConfig) => {
  return request<T>({
    url: url,
    method: "GET",
    ...options,
  });
};

const post = <T = any>(
  url: string,
  data?: any,
  options?: AxiosRequestConfig,
) => {
  return request<T>({
    ...options,
    url,
    data,
    method: "POST",
  });
};

const put = <T = any>(url: string, data: any, options?: AxiosRequestConfig) => {
  return request<T>({
    ...options,
    url,
    data,
    method: "PUT",
  });
};

const patch = <T = any>(
  url: string,
  data: any,
  options?: AxiosRequestConfig,
) => {
  return request<T>({
    ...options,
    url,
    data,
    method: "PATCH",
  });
};

const remove = <T = any>(
  url: string,
  data?: any,
  options?: AxiosRequestConfig,
) => {
  return request<T>({
    ...options,
    url,
    data,
    method: "DELETE",
  });
};

export const baseApi = {
  get,
  put,
  post,
  remove,
  patch,
};
