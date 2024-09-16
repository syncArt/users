import { createBrowserRouter } from "react-router-dom";
import { LoginPage, AppsData, Dashboard } from "@/pages";
import { Layout } from "@/modules/Layout";

export const router = createBrowserRouter([
  {
    element: <Layout />,
    children: [
      {
        path: "/",
        element: <LoginPage />
      },
      {
        path: "/login",
        element: <LoginPage />
      }
    ]
  },
  {
    element: <Layout protectedRoutes />,
    children: [
      {
        path: "/dashboard",
        element: <Dashboard />
      },
      {
        path: "/dashboard/:app_name",
        element: <AppsData />
      }
    ]
  }
]);
