export const setToken = (token: string) => localStorage.setItem("token", token);

export const getToken = () => localStorage.getItem("token");

export const removeToken = () => localStorage.removeItem("token");

export const setRefreshToken = (token: string) =>
  localStorage.setItem("refreshToken", token);

export const getRefreshToken = () => localStorage.getItem("refreshToken");

export const removeRefreshToken = () => localStorage.removeItem("refreshToken");
