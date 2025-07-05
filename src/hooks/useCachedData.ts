import { useEffect, useState } from "react";

// Hook para cachear datos por tiempo definido (ms)
export default function useCachedData<T>(fetcher: () => Promise<T>, cacheMs: number): [T | null, boolean] {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let isMounted = true;
    const cacheKey = fetcher.toString();
    const cached = sessionStorage.getItem(cacheKey);
    const cachedTime = sessionStorage.getItem(cacheKey + ":time");
    const now = Date.now();
    if (cached && cachedTime && now - parseInt(cachedTime) < cacheMs) {
      setData(JSON.parse(cached));
      setLoading(false);
    } else {
      setLoading(true);
      fetcher().then(res => {
        if (isMounted) {
          setData(res);
          setLoading(false);
          sessionStorage.setItem(cacheKey, JSON.stringify(res));
          sessionStorage.setItem(cacheKey + ":time", now.toString());
        }
      }).catch(() => {
        if (isMounted) setLoading(false);
      });
    }
    return () => { isMounted = false; };
  }, [fetcher, cacheMs]);

  return [data, loading];
}
