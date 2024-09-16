// @ts-nocheck
import { useState } from "react";
import { TextureStyleObjDto } from "../models/dto/textureStyleObj-dto";

export const usePalette = () => {
  const [palette, setPalette] = useState<TextureStyleObjDto>(null);

  const changeEmoji = (name, val) => {
    switch (name) {
      case "main":
        setPalette((prev) => ({ ...prev, main: val }));
        break;
      case "background":
        setPalette((prev) => ({ ...prev, background: val }));
        break;
      case "border":
        setPalette((prev) => ({ ...prev, border: val }));
        break;
      case "outline":
        setPalette((prev) => ({ ...prev, outline: val }));
        break;
      default:
        throw Error("changeEmoji is fucked up");
    }
  };

  const initializePalette = (data: TextureStyleObjDto) => {
    setPalette(data);
  };

  return [
    {
      ...palette,
      initializePalette
    },
    changeEmoji
  ];
};
