// @ts-nocheck
// options inherit from single element renderer of picked lib. For emoji mart would be: skin, size, set
// builder is about builder function and resolution of given letter
// letter is representation of chosen letter from keyboard

import {BuilderFuncType, Resolution, LetterStyles} from "@/types/Letter";

export class LetterObjDto {
  letter: string;
  builder: BuilderFuncType;
  resolution: Resolution;
  defaultOptions: LetterStyles;
  data: any[][];

  constructor(data: LetterObjDto) {
    this.letter = data.letter;
    this.builder = data.builder;
    this.defaultOptions = data.defaultOptions;
    this.resolution = data.resolution;
    this.data = data.data;

    Object.preventExtensions(this as Object);
  }
}
