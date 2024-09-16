// @ts-nocheck
import { useState } from "react";
import { Texture } from "../types/Letter";

export const useLetterOptions = () => {
  const [letterOptions, setLetterOptions] = useState<Pick<"options", Texture>>();

  return [letterOptions, setLetterOptions];
};
