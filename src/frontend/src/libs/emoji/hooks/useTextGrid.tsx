// @ts-nocheck
import { useCallback, useState } from "react";
import { useLettersLib } from "./useLettersLib";
import { Options, Texture } from "../types/Grid";
import { TextureStyleObjDto } from "../models/dto/textureStyleObj-dto";

const parseTextToRows = (text: string[][], callback) => {
  return text.map((word) => callback(word));
};

const wordToDisplayParser = (word: string[][], palette) => {
  const findHighestLetter = word?.reduce((prevVal, currentLetter) => {
    return prevVal > currentLetter.resolution.y
      ? prevVal
      : currentLetter.resolution.y;
  }, 0);

  const rowsAmountArr = Array.from({ length: findHighestLetter }, (v, k) => k);

  return rowsAmountArr.map((index) => {
    return word.reduce((prevVal, currentLetter, letterIndex) => {
      //check if current row for letter exist
      //else fill with background emoji
      if (!!currentLetter.data[index]) {
        //check if last letter of the text. if so, dont add brake between chars
        if (word.length - 1 === letterIndex) {
          return `${prevVal}${currentLetter.data[index]}`;
        } else {
          return `${prevVal}${currentLetter.data[index]},${palette.background},`;
        }
      } else {
        const letterWidth = currentLetter.resolution.x;
        const filledWithBlank = Array.from(
          { length: letterWidth },
          (v, k) => palette.background
        );
        return `${prevVal}${filledWithBlank.flat()}`;
      }
    }, "");
  });
};

export const useTextGrid = (
  template,
  libType,
  initTexture?: {
    options?: Partial<Options>;
    initialVal?: Partial<TextureStyleObjDto>;
  }
) => {
  const [text, setText] = useState("");
  const [resolution, setResolution] = useState({ x: 0, y: 0 });
  const [lib, { palette, letterOptions, changeEmoji }] = useLettersLib(
    libType,
    template,
    initTexture
  );

  const getTextGrid = useCallback(() => {
    if (!!lib?.a && !!text) {
      const separateWords = text.split("\n").filter((el) => !!el);

      if (initTexture?.initialVal?.outline?.x || palette?.outline?.x) {
        return separateWords.map((word) => [
          lib[" "],
          ...word.split("").map((char) => lib[char]),
          lib[" "]
        ]);
      }

      return separateWords.map((word) =>
        word.split("").map((char) => lib[char])
      );
    }
    return [];
  }, [lib, text]);

  const textGrid = getTextGrid();

  const fullText = parseTextToRows(textGrid, (word) => {
    const lettersWidthSum = word.reduce(
      (prev, current) => prev + current.resolution.x,
      0
    );

    //count one column of break added after each letter
    const lettersBrakesSum = word.length - 1;
    const fullWidth = lettersWidthSum + lettersBrakesSum;

    return wordToDisplayParser(word, palette);
  });

  return [
    { textGrid: fullText, inputText: text, palette, lib, letterOptions },
    { setText, changeEmoji }
  ];
};
