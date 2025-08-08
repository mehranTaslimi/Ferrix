import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useFormContext } from "react-hook-form";
import FormMessage from "./form-message";

export default function AuthField() {
  const { control, watch } = useFormContext();
  const authType = watch("auth.type") || "None";

  return (
    <div className="space-y-4 rounded-lg border p-4">
      <FormField
        control={control}
        name="auth.type"
        render={({ field }) => (
          <FormItem className="flex items-center justify-between">
            <FormLabel>Authentication Type</FormLabel>
            <Select onValueChange={field.onChange} value={field.value}>
              <FormControl>
                <SelectTrigger>
                  <SelectValue placeholder="Select authentication type" />
                </SelectTrigger>
              </FormControl>
              <SelectContent>
                <SelectItem value="None">None</SelectItem>
                <SelectItem value="Basic">Basic Auth</SelectItem>
                <SelectItem value="Bearer">Bearer Token</SelectItem>
                <SelectItem value="CustomToken">Custom Token</SelectItem>
                <SelectItem value="ApiKeyHeader">API Key (Header)</SelectItem>
                <SelectItem value="ApiKeyQuery">API Key (Query)</SelectItem>
                <SelectItem value="Cookie">Cookie Auth</SelectItem>
              </SelectContent>
            </Select>
          </FormItem>
        )}
      />

      {authType === "Basic" && (
        <div className="space-y-4 p-4 border rounded-lg">
          <FormField
            control={control}
            name="auth.username"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Username</FormLabel>
                <FormControl>
                  <Input placeholder="username" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={control}
            name="auth.password"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Password</FormLabel>
                <FormControl>
                  <Input type="password" placeholder="••••••" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>
      )}

      {authType === "Bearer" && (
        <div className="p-4 border rounded-lg">
          <FormField
            control={control}
            name="auth.token"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Bearer Token</FormLabel>
                <FormControl>
                  <Input placeholder="Enter token" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>
      )}

      {authType === "CustomToken" && (
        <div className="space-y-4 p-4 border rounded-lg">
          <FormField
            control={control}
            name="auth.scheme"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Token Scheme</FormLabel>
                <FormControl>
                  <Input placeholder="Token" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={control}
            name="auth.token"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Token Value</FormLabel>
                <FormControl>
                  <Input placeholder="Enter token" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>
      )}

      {authType === "ApiKeyHeader" && (
        <div className="space-y-4 p-4 border rounded-lg">
          <FormField
            control={control}
            name="auth.header_name"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Header Name</FormLabel>
                <FormControl>
                  <Input placeholder="X-API-Key" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={control}
            name="auth.key"
            render={({ field }) => (
              <FormItem>
                <FormLabel>API Key</FormLabel>
                <FormControl>
                  <Input placeholder="Enter API key" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>
      )}

      {authType === "ApiKeyQuery" && (
        <div className="space-y-4 p-4 border rounded-lg">
          <FormField
            control={control}
            name="auth.key_name"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Query Parameter</FormLabel>
                <FormControl>
                  <Input placeholder="api_key" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={control}
            name="auth.key"
            render={({ field }) => (
              <FormItem>
                <FormLabel>API Key</FormLabel>
                <FormControl>
                  <Input placeholder="Enter API key" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>
      )}

      {authType === "Cookie" && (
        <div className="p-4 border rounded-lg">
          <FormField
            control={control}
            name="auth.cookie"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Cookie Value</FormLabel>
                <FormControl>
                  <Input placeholder="session=abc123" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>
      )}
    </div>
  );
}
