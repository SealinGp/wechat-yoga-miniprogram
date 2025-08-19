import type { Config } from "@react-router/dev/config";

export default {
  // Config options...
  // Server-side render by default, to enable SPA mode set this to `false`
  ssr: true,
  // Handle static assets and system requests
  ignoredRouteFiles: ["**/*.css", "**/*.json"],
  // Prerender static routes
  prerender: ["/"],
} satisfies Config;
