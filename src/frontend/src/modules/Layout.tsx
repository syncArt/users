import { Outlet, Navigate } from "react-router-dom";
import BackgroundShape from "@/assets/background.svg";
import { useAuth } from "@/hooks/useAuthClient";
import { useNavigate } from "react-router-dom";
import { useEffect } from "react";

export const Layout = ({
                         protectedRoutes = false
                       }: {
  protectedRoutes?: boolean;
}) => {
  const { isAuthenticated } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    if (protectedRoutes && !isAuthenticated) {
      navigate("/login");
    }
  }, [protectedRoutes, isAuthenticated, navigate]);

  if (!isAuthenticated && protectedRoutes) {
    return <Navigate to="/login" />;
  }

  return (
    <main className="flex min-h-screen w-full justify-center">
      <span className="fixed bottom-0 left-0 right-0 top-0 -z-10 flex">
        <BackgroundShape className="left-0 h-full min-h-[1880px] w-full min-w-[2000px] scale-150" />
      </span>
      <Outlet />
    </main>
  );
};
