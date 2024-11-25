import { useEffect, useState } from "react";
import { FormField } from "@/components/FormField";
import { useNavigate } from "react-router-dom";
import {
  optionalCandidValueToValue,
  valueToOptionalCandidValue
} from "@/utils";
import { useApi } from "@/hooks/useAuthClient";

export type UserDataType = {
  nickname: string;
  description?: string;
};

export type FormFieldData = {
  name: keyof UserDataType;
  value: string | number;
};

export const Dashboard = () => {
  const [userData, setUserData] = useState<UserDataType>({
    description: "",
    nickname: ""
  });
  const { usersInterRsActor, logout, principal } = useApi();
  const navigate = useNavigate();

  const handleUserGeneralData = async () => {
    await usersInterRsActor.get_general_info_from_user().then((res) => {
      if ("Ok" in res) {
        const data = res.Ok;

        const parsedData = {
          nickname: data.nickname,
          description: optionalCandidValueToValue(data.description)
        };

        setUserData(parsedData);
      } else {
        console.error("Error:", res.Err);
      }
    });
  };

  useEffect(() => {
    handleUserGeneralData();
  }, []);

  const handleSave = async () => {
    console.log(userData);
    await usersInterRsActor
      .update({
        app_type: { General: null },
        general_info: [
          {
            ...userData,
            description: valueToOptionalCandidValue(userData.description)
          }
        ],
        apps_data: []
      })
      .then(() => {
        console.log("saved");
        handleUserGeneralData();
      });
  };

  const handleFieldChange = (data: FormFieldData) => {
    setUserData((prev) => ({ ...prev, [data.name]: data.value }));
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
          value={principal?.toString() || ""}
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
            name="nickname"
            fieldName="nickname"
            initVal={userData.nickname}
            onChange={handleFieldChange}
          />
          <FormField
            name="description"
            fieldName="description"
            initVal={userData.description}
            onChange={handleFieldChange}
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
