import { create } from "zustand"
import { User } from "@/type/user.ts"

interface userState {
  user: User | null
  setUser: (user: User) => void
}

const useUserStore = create<userState>((set) => ({
  user: null,
  setUser: (user) => set(() => ({ user })),
}))

export const useUserInfo = () => useUserStore((state) => state.user)

export const useUserActions = () => ({
  setUser: useUserStore((state) => state.setUser),
})
