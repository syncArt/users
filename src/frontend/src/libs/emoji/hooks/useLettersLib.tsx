// @ts-nocheck
import { useCallback, useEffect, useState } from "react";
import { LIB_TYPES, TEXTURE_TYPES } from "../types";
import { LettersLibObjDto } from "../models/dto/lettersLibObj-dto";
import { LetterObjDto } from "../models/dto/letterObj-dto";
import { BuilderFuncType, Texture } from "../types/Letter";
import { useLetterOptions, usePalette } from "../hooks";
import { fr } from "../data/ASCII_3x4_letters_builder_function";
import { Options } from "../types/Grid";
import { TextureStyleObjDto } from "../models/dto/textureStyleObj-dto";

const buildLib = (builderFuncs, textureData, callback) => {
  let lib = {};
  const availableLetters = Object.keys(builderFuncs);
  availableLetters.forEach((el: string) => {
    const letterData: any = builderFuncs[el](
      textureData.initialVal.main,
      textureData.initialVal.background
    );

    const resolution = {
      y: textureData.initialVal.outline?.x
        ? letterData.length + 2
        : letterData.length,
      x: letterData[0].length
    };

    const dataWithOutline = [
      [fr(letterData[0].length, textureData.initialVal.background)],
      ...letterData,
      [fr(letterData[0].length, textureData.initialVal.background)]
    ];

    let letter = new LetterObjDto({
      letter: el,
      builder: builderFuncs[el],
      defaultOptions: (textureData as Texture).options,
      data: textureData.initialVal.outline ? dataWithOutline : letterData,
      resolution: resolution
    });
    lib = { ...lib, [el]: letter };
  });
  callback({ lib, texture: textureData });
};

const buildLibInit = async (libName, texture, callback) => {
  const loadLib = import(`../data`).then((mod) => mod[libName]);
  const loadTextureData = import(`../textures`).then((mod) => mod[texture]);
  try {
    await Promise.all([loadLib, loadTextureData]).then(
      ([builderFuncs, textureData]: [BuilderFuncType, Texture]) => {
        buildLib(builderFuncs, textureData, callback);
      }
    );
  } catch (err) {
    console.error("THERE WAS AN ERROR BUILDING LIB. check LettersLib");
  }
};

const rebuildLibWithNewPalette = (
  lib,
  oldPalette,
  modifier
): LettersLibObjDto => {
  let newLib = {};

  const newPalette = { ...oldPalette, ...modifier };
  Object.keys(lib).forEach((el) => {
    const newData = lib[el]?.builder(newPalette.main, newPalette.background);

    const dataWithOutline = [
      [fr(newData[0].length, newPalette.background)],
      ...newData,
      [fr(newData[0].length, newPalette.background)]
    ];

    newLib[el] = {
      ...lib[el],
      data: newPalette.outline ? dataWithOutline : newData
    };
  });

  return newLib as LettersLibObjDto;
};

export const useLettersLib = (
  libName: LIB_TYPES,
  texture: TEXTURE_TYPES,
  initTexture?: {
    options?: Partial<Options>;
    initialVal?: Partial<TextureStyleObjDto>;
  }
) => {
  const [lib, setLib] = useState<LettersLibObjDto | null>(null);
  const [letterOptions, setLetterOptions] = useLetterOptions();
  const [{ initializePalette, ...palette }, changeEmoji] = usePalette();

  useEffect(() => {
    buildLibInit(libName, texture, ({ lib, texture }) => {
      setLib(lib);
      setLetterOptions(texture.options);
      initializePalette(texture.initialVal);
    });
  }, []);

  const handleChangeEmoji = useCallback(
    (name, val) => {
      if (!!palette && Object.values(palette).length > 0) {
        setLib((prev) => {
          return rebuildLibWithNewPalette(prev, palette, { [name]: val });
        });
        changeEmoji(name, val);
      } else {
        console.error("[custom] palette is empty!!!!");
      }
    },
    [palette]
  );

  return [
    lib,
    {
      palette,
      letterOptions,
      changeEmoji: handleChangeEmoji,
      setLetterOptions
    }
  ];
};
