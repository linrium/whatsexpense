import { AxiosError } from "axios";
import { useCallback, useEffect, useState } from "react";

export const useAwaited = <T, P = any>(
  func: (props?: P) => Promise<T>,
  dependencies?: P,
) => {
  const [data, setData] = useState<T | null>(null);
  const [error, setError] = useState<AxiosError | unknown>();
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async (params?: P) => {
      try {
        const result = await func(params);
        setData(result);
      } catch (err: AxiosError | unknown) {
        setError(err);
      } finally {
        setLoading(false);
      }
    };

    fetchData(dependencies);
  }, [dependencies]);

  return { data, error, loading };
};

export const useMutation = <T, P = any>(
  func: (props: P) => Promise<T>,
  options?: {
    onSuccess?: (data: T) => void;
    onError?: (err: AxiosError | unknown) => void;
  },
) => {
  const [error, setError] = useState<AxiosError | unknown>();
  const [loading, setLoading] = useState(false);

  const asyncMutate = useCallback(async (params: P) => {
    setLoading(true);
    try {
      const data = await func(params);
      options?.onSuccess?.(data);
    } catch (err: AxiosError | unknown) {
      options?.onError?.(err);
      setError(err);
    } finally {
      setLoading(false);
    }
  }, []);

  const mutate = useCallback(
    (params: P) => {
      asyncMutate(params);
    },
    [asyncMutate],
  );

  return { mutate, asyncMutate, error, loading };
};
