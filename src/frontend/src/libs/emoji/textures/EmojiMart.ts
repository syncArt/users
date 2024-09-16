import { TextureStyleObjDto } from "../models/dto/textureStyleObj-dto";
import { Letter } from "../types";

const data: Letter.Texture = {
  options: {
    skin: "1",
    size: "2em",
    set: "native",
    fallback: ":shrug:"
  },
  initialVal: new TextureStyleObjDto({
    main: "🌕",
    background: "🌑",
    border: "🚀",
    outline: {
      x: true,
      y: true
    }
  })
};

export default data;
