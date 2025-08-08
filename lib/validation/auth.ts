import { z } from "zod";

export const authSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("Basic"),
    username: z.string().min(1, "Username is required"),
    password: z.string().min(1, "Password is required"),
  }),

  z.object({
    type: z.literal("Bearer"),
    token: z.string().min(1, "Token is required"),
  }),

  z.object({
    type: z.literal("CustomToken"),
    scheme: z.string().min(1, "Scheme is required"),
    token: z.string().min(1, "Token is required"),
  }),

  z.object({
    type: z.literal("ApiKeyHeader"),
    header_name: z.string().min(1, "Header name is required"),
    key: z.string().min(1, "API key is required"),
  }),

  z.object({
    type: z.literal("ApiKeyQuery"),
    key_name: z.string().min(1, "Query parameter name is required"),
    key: z.string().min(1, "API key is required"),
  }),

  z.object({
    type: z.literal("Cookie"),
    cookie: z.string().min(1, "Cookie value is required"),
  }),
]);

export type AuthType = z.infer<typeof authSchema>;
