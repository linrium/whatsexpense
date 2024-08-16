import { useMessages } from "@/fetchHook/message.ts";
import { useUserInfo } from "@/fetchHook/user.ts";
import { Message, Transaction } from "@/type/message.ts";
import { cn } from "@/lib/utils.ts";
import { buttonVariants } from "@/components/ui/button.tsx";
import { Badge } from "@/components/ui/badge.tsx";
import { useEffect, useRef } from "react";
import { ScrollArea } from "@/components/ui/scroll-area.tsx";
import { Separator } from "@/components/ui/separator.tsx";

export const ChatSection = () => {
  const { data: messages, loading } = useMessages();
  const { data: user } = useUserInfo();
  const listRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (listRef.current) {
      listRef.current.scrollIntoView(false);
    }
  }, [messages?.length]);

  if (loading) return null;

  return (
    <div className="flex flex-col size-full bg-gray-50">
      <ScrollArea ref={listRef}>
        <div className="flex flex-col h-full w-full gap-4 pb-20 px-4">
          {[...messages]
            .reverse()
            .map((message) =>
              user ? (
                <MyMessage {...message} key={message.id} userId={user.id} />
              ) : null,
            )}
        </div>
      </ScrollArea>
    </div>
  );
};

const MyMessage = ({ userId, ...props }: Message & { userId: string }) => {
  return props.fromId === userId ? (
    <div className="w-full max-w-[75%] self-end flex flex-col items-end gap-1">
      <div
        className={cn(
          buttonVariants({
            variant: "default",
            size: "default",
          }),
        )}
      >
        {props.content}
      </div>
      <div className="text-xs font-light text-neutral-500">
        {props.createdAt}
      </div>
    </div>
  ) : (
    <div
      className={cn("w-full max-w-[75%] self-start p-2 rounded-lg bg-white")}
    >
      <div>
        {props.transactions.map((transaction) => (
          <BotMessage {...transaction} />
        ))}
      </div>
    </div>
  );
};

const BotMessage = ({
  type,
  amount,
  currency,
  categoryId,
  title,
  quantity,
  updatedAt,
}: Transaction) => {
  return (
    <div>
      <div className="flex items-center gap-2 flex-wrap">
        <Badge>{updatedAt}</Badge>
        <Badge>{currency}</Badge>
        <Badge variant={type === "outcome" ? "destructive" : "default"}>
          {type}
        </Badge>
        <Badge>{categoryId}</Badge>
      </div>
      <Separator className="my-1" />
      <div className="flex items-center justify-between">
        <div>
          <div>{title}</div>
          <div className="text-neutral-500">
            {quantity} <span>{quantity <= 1 ? "item" : "items"}</span>
          </div>
        </div>

        <div>{toCurrency(amount, "VND")}</div>
      </div>
    </div>
  );
};

const toCurrency = (amount: string | number, currency = "USD") => {
  let number = amount;

  if (typeof number === "string") {
    number = parseFloat(number);
    if (isNaN(number)) return amount;
  }

  const formatter = new Intl.NumberFormat("en-US", {
    style: "currency",
    currency,
  });

  return formatter.format(number);
};
