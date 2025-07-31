import { z } from "zod";

interface positiveNumberSchemaProps {
  message: string;
}
export const positiveNumberSchema = ({ message }: positiveNumberSchemaProps) =>
  z.coerce
    .number()
    .refine((val) => val > 0, {
      message,
    })
    .optional();
