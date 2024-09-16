import { Outlet } from "react-router-dom";
import BackgroundShape from "../assets/background.svg";

export const Layout = () => {
  return (
    <main className="flex min-h-screen w-full justify-center">
      <span className="fixed bottom-0 left-0 right-0 top-0 -z-10 flex">
        <BackgroundShape className="left-0 h-full min-h-[1880px] w-full min-w-[2000px] scale-150" />
      </span>
      <Outlet />
    </main>
  );
};
