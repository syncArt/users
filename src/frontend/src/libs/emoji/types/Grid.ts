import { TextureStyleObjDto } from "../models/dto/textureStyleObj-dto";

export type Resolution = {
  x: number;
  y: number;
}

export type Options = {
  isVertical: boolean;
  hasBorder: boolean;
  isSquarish: boolean;
};

export type Texture = {
  options: Options;
  initialVal: TextureStyleObjDto;
};