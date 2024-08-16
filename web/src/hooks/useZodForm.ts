import { zodResolver } from "@hookform/resolvers/zod"
import {
  FieldValues,
  useForm,
  UseFormProps,
  UseFormReturn,
} from "react-hook-form"
import { z } from "zod"

interface Props {
  schema: z.Schema
}

export const useZodForm = <
  TFieldValues extends FieldValues = FieldValues,
  TContext = any,
  TTransformedValues extends FieldValues | undefined = undefined,
>({
  schema,
  ...props
}: UseFormProps<TFieldValues, TContext> & Props): UseFormReturn<
  TFieldValues,
  TContext,
  TTransformedValues
> => useForm({ resolver: zodResolver(schema), ...props })
