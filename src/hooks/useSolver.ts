import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export function useSolver(input: any) {
  const [data, setData] = useState<any>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const handler = setTimeout(async () => {
      setLoading(true);
      try {
        const result = await invoke("solve_linear_program", { input });
        setData(result);
        setError(null);
      } catch (err) {
        setError(err as string);
      } finally {
        setLoading(false);
      }
    }, 250); // Debounce de 250ms conforme au blueprint

    return () => clearTimeout(handler);
  }, [input]);

  return { data, loading, error };
}
