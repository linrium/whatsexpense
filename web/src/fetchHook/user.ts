import { useAwaited } from "@/hooks/useAwaited.ts";
import { useEffect } from "react";
import { getUser } from "@/service/user.ts";
import {
  useUserActions,
  useUserInfo as useUserStoreInfo,
} from "@/store/user.ts";

export const useUserInfo = () => {
  const { setUser } = useUserActions();
  const user = useUserStoreInfo();
  const data = useAwaited(getUser);

  useEffect(() => {
    if (data.data) {
      setUser(data.data);
    }
  }, [data.data, data.loading, setUser]);

  return { ...data, data: user };
};
