import { z } from 'zod';

const hostRegex =
  /^(([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9])\.)*([A-Za-z0-9]|[A-Za-z0-9][A-Za-z0-9-]*[A-Za-z0-9])$|^(([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])$/;

const proxyAuthSchema = z
  .object({
    username: z.string().max(100).optional(),
    password: z.string().max(100).optional(),
  })
  .superRefine((v, ctx) => {
    const u = v.username;
    const p = v.password;
    if (u && !p) {
      ctx.addIssue({
        code: z.ZodIssueCode.custom,
        path: ['password'],
        message: 'Password is required when username is provided',
      });
    }
    if (p && !u) {
      ctx.addIssue({
        code: z.ZodIssueCode.custom,
        path: ['username'],
        message: 'Username is required when password is provided',
      });
    }
  })
  .optional();

export const proxySchema = z.discriminatedUnion('type', [
  z.object({ type: z.literal('none') }),

  z.object({ type: z.literal('system') }),

  z.object({
    type: z.enum(['http', 'https', 'socks5']),
    host: z
      .string()
      .min(1, 'Proxy host is required')
      .max(255, 'Host too long')
      .regex(hostRegex, 'Invalid hostname or IP address'),
    port: z
      .number({ invalid_type_error: 'Port must be a number' })
      .int('Port must be an integer')
      .min(1, 'Port must be 1-65535')
      .max(65535, 'Port must be 1-65535'),
    auth: proxyAuthSchema,
  }),
]);

export type ProxyType = z.infer<typeof proxySchema>;
