import { useQuery } from "@tanstack/react-query";

import { ping } from "./commands";

/** Estado assíncrono do command `ping` via TanStack Query (ADR-015). */
export function usePing() {
  return useQuery({ queryKey: ["ping"], queryFn: ping });
}
