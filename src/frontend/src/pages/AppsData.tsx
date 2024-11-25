import { useParams } from "react-router-dom";
import { useEffect, useState } from "react";
import { useApi } from "@/hooks/useAuthClient";

export const AppsData = () => {
  const [smileyballData, setSmileyballData] = useState("");
  const { app_name } = useParams();
  const { usersInterRsActor } = useApi();

  useEffect(() => {
    const handleUserSmileyballData = async () => {
      await usersInterRsActor
        .get_app_data_from_user({ Smileyball: null })
        .then((res) => {
          if ("Ok" in res) {
            const data = res.Ok;

            console.log(data);
            setSmileyballData(JSON.stringify(data));
          } else {
            console.error("Error:", res.Err);
          }
        });
    };

    handleUserSmileyballData();
  }, []);

  return (
    <div>
      <h1>App Name: {app_name}</h1>
      {smileyballData}
    </div>
  );
};
