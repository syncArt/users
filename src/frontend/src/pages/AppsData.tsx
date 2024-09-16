import { useParams } from "react-router-dom";

export const AppsData = () => {
  const { app_name } = useParams();

  return (
    <div>
      <h1>App Name: {app_name}</h1>
    </div>
  );
};