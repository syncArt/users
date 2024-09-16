import EmojiArt from "@/components/EmojiArt";
import { HeroText } from "@/components/HeroText";
import DashedBackground from "../assets/dash-animated-background.svg";
import IILoginImage from "../assets/II-login-button.svg";
import { useAuth } from "@/hooks/useAuthClient";
import { useEffect, useState } from "react";

export const HeroPage = () => {
  const { login, whoamiActor, logout } = useAuth();
  const [result, setResult] = useState("");
  const [result2, setResult2] = useState("");

  const handleLogin = () => {
    login();
  };

  useEffect(() => {
    if (!!whoamiActor) {
      const handleWhoami = async () => {
        const whoami = await whoamiActor?.whoami();
        console.log("whoami", whoami);
        setResult(whoami);
      };
      const handleWhoamiToText = async () => {
        const whoamiToText = await whoamiActor?.id_to_account();
        console.log("whoamiToText", whoamiToText);
        setResult2(whoamiToText);
      };
      handleWhoami();
      handleWhoamiToText();
    }
  }, [whoamiActor]);

  return (
    <div className="flex w-full">
      <div className="hidden w-full min-w-[380px] pl-28 laptop:flex">
        <div className="flex flex-col">
          <div className="mt-20 flex">
            <EmojiArt char={"1"} />
          </div>
          <h1 className="mt-10 flex font-spaceMono text-[48px] font-bold italic text-white">
            USER IDENTITY
          </h1>
          <span className="flex font-spaceMono text-[33px] italic text-white">
            Share and manage your data across multiple applications
          </span>
          <span className="mt-2 flex font-spaceMono text-[14px] font-bold italic text-white">
            (possible only on ICP)
          </span>
          <HeroText />
        </div>
      </div>
      <div className="relative right-0 flex min-h-screen w-full flex-col items-center justify-center bg-white">
        {/*<DashedBackground className="flex object-cover object-center" />*/}
        <span className="flex text-[18px]">Log in with Internet Identity</span>
        <button onClick={() => handleLogin()}>
          <IILoginImage />
        </button>
        <input
          type="text"
          readOnly
          id="whoami"
          value={result}
          placeholder="......"
        />
      </div>
    </div>
  );
};
