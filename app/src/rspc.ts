import { createClient } from "@rspc/client";
import { TauriTransport } from "@rspc/tauri";
import type { Procedures } from "./bindings"; // These were the bindings exported from your Rust code!
import { QueryClient } from "@tanstack/react-query";
import { createReactQueryHooks } from "@rspc/react-query";


export const client = createClient<Procedures>({
    transport: new TauriTransport(),
});

export const queryClient = new QueryClient();
export const rspc = createReactQueryHooks<Procedures>();