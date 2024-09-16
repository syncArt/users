import { TextureStyleObjDto } from "../models/dto/textureStyleObj-dto";
import { Letter } from "../types";

const data: Letter.Texture = {
  options: {
    skin: "1",
    size: "16px",
    set: "twitter",
    fallback: ":shrug:"
  },
  initialVal: new TextureStyleObjDto({
    main: "🙂",
    background: "🌑",
    border: "🚀",
    outline: null
  })
};

export default data;
