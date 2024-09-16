import { EmojiMartOptions } from "./custom/EmojiMart";
import { TextureStyleObjDto } from "../models/dto/textureStyleObj-dto";

export type BuilderFuncType = (x: any, y: any) => any[][];

export type Resolution = {
  x: number;
  y: number;
};

//EXTENDABLE IF ANY OTHER STYLES FOR OTHER LIB
export type LetterStyles = EmojiMartOptions;
export type Texture = {
  options: LetterStyles;
  initialVal: TextureStyleObjDto;
};
