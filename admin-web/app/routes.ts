import { type RouteConfig, index, route } from "@react-router/dev/routes";

export default [
  index("routes/login.tsx"),
  route("/admin", "components/AdminLayout.tsx", [
    route("dashboard", "routes/admin/dashboard.tsx"),
    route("lessions", "routes/admin/lessions.tsx"),
    route("lessons-list", "routes/admin/lessons-list.tsx"),
    route("lesson/create", "routes/admin/lesson/create.tsx"),
    route("actions", "routes/admin/actions.tsx"),
    route("teachers", "routes/admin/teachers.tsx"),
    route("posters", "routes/admin/posters.tsx"),
    route("notices", "routes/admin/notices.tsx"),
    route("admin-users", "routes/admin/admin-users.tsx"),
    route("locations", "routes/admin/locations.tsx"),
    route("users", "routes/admin/users.tsx"),
  ]),
  // Catch-all route for unmatched paths
  route("*", "routes/not-found.tsx"),
] satisfies RouteConfig;
