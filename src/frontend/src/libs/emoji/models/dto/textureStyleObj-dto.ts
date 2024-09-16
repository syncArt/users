// @ts-nocheck
export class TextureStyleObjDto {
  main: string;
  background: string;
  border: string;
  outline: {
    x: boolean;
    y: boolean;
  } | null;

  constructor(data: TextureStyleObjDto) {
    this.main = data.main;
    this.background = data.background;
    this.border = data.border;
    this.outline = data.outline;

    Object.preventExtensions(this as Object);
  }
}
