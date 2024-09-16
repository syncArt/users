import {createBrowserRouter} from "react-router-dom";
import {HeroPage} from "@/pages/HeroPage";
import {Layout} from "@/modules/Layout";

export const router = createBrowserRouter([
    {
        element: <Layout/>,
        children: [
            {
                path: "/",
                element: <HeroPage/>,
            }
        ],
    },
]);
