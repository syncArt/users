import { useEffect, useState } from "react";
import { useAuth } from "@/hooks/useAuthClient";
import { FormField } from "@/components/FormField";
import { useNavigate } from "react-router-dom";

export const Dashboard = () => {
  const [result, setResult] = useState("");
  const [updatedData, setNewData] = useState({});

  const { whoamiActor, logout } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    if (!!whoamiActor) {
      const handleWhoami = async () => {
        const whoami = await whoamiActor?.whoami();
        setResult(whoami);
      };
      handleWhoami();
    }
  }, [whoamiActor]);

  const handleSave = () => {
    console.log(updatedData);
  };

  const handleAppDataClick = (appName: string) => {
    console.log("test");
    navigate(`/dashboard/${appName}`);
  };

  return (
    <div className="m-20 flex w-full flex-col items-start justify-start">
      <div className="flex flex-nowrap font-spaceMono text-[14px] text-white">
        <button className="mr-5 flex" onClick={() => logout()}>
          Logout
        </button>
        <p className="flex">Internet Identity:</p>
        <input
          type="text"
          readOnly
          id="whoami"
          value={result}
          placeholder="......"
          className="ml-2 flex border-none bg-transparent text-white outline-0"
        />
      </div>
      <div>
        <span className="mt-10 flex font-sequel100Black font-95 italic text-white">
          General data:
        </span>
        <div className="ml-8 flex flex-col">
          <FormField
            fieldName="nickname"
            initVal="testtt"
            onChange={(value) => setNewData((prev) => ({ ...prev, value }))}
          />
          <FormField
            fieldName="connected apps"
            initVal="test222"
            onChange={(value) => setNewData((prev) => ({ ...prev, value }))}
          />
        </div>
      </div>
      <button
        className="mt-5 flex font-sequel100Black text-white"
        onClick={handleSave}
      >
        SAVE
      </button>

      <span className="mt-20 flex font-sequel100Black font-95 italic text-white">
        Apps data:
      </span>

      <button
        onClick={() => handleAppDataClick("smileyball")}
        className="ml-5 mt-2 flex h-[100px] w-[100px] cursor-pointer items-center rounded-medium border-2 border-black bg-[#959494] text-[4px] hover:bg-grey"
      >
        <span
          className="align-center logo-wrapper pointer-events-none relative flex w-full flex-col flex-wrap justify-center font-[5px]">
          <p className="flex w-full justify-center">🌕🌕🌕🌕🌕</p>
          <p className="flex w-full justify-center">🌕🌕🌕🌕🌕🌕🌕🌕🌕</p>
          <p className="flex w-full justify-center">🌕🌕🌕🌕🌕🌕🌕🌕🌕🌕🌕</p>
          <p className="flex w-full justify-center">🌕🌕🌕🌑🌕🌕🌕🌑🌕🌕🌕</p>
          <p className="flex w-full justify-center">
            🌕🌕🌕🌕🌑🌕🌕🌕🌑🌕🌕🌕🌕
          </p>
          <p className="flex w-full justify-center">
            🌕🌕🌕🌕🌑🌕🌕🌕🌑🌕🌕🌕🌕
          </p>
          <p className="flex w-full justify-center">
            🌕🌕🌕🌕🌕🌕🌕🌕🌕🌕🌕🌕🌕
          </p>
          <p className="flex w-full justify-center">
            🌕🌕🌑🌕🌕🌕🌕🌕🌕🌕🌑🌕🌕
          </p>
          <p className="flex w-full justify-center">
            🌕🌕🌑🌕🌕🌕🌕🌕🌕🌕🌑🌕🌕
          </p>
          <p className="flex w-full justify-center">🌕🌕🌑🌕🌕🌕🌕🌕🌑🌕🌕</p>
          <p className="flex w-full justify-center">🌕🌕🌕🌑🌑🌑🌑🌑🌕🌕🌕</p>
          <p className="flex w-full justify-center">🌕🌕🌕🌕🌕🌕🌕🌕🌕</p>
          <p className="flex w-full justify-center">🌕🌕🌕🌕🌕</p>
        </span>
      </button>
    </div>
  );
};
