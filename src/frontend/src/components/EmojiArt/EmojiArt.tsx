//@ts-nocheck

import { RenderEmoji } from "../Emoji";
import styles from "./emojiArt.module.css";
import { TEXTURE_TYPES, LIB_TYPES } from "@/libs/emoji/types";
import { useTextGrid } from "@/libs/emoji/hooks/useTextGrid";
import { TextureStyleObjDto } from "@/libs/emoji/models/dto/textureStyleObj-dto";
import { Grid } from "@/libs/emoji/types";

export const EmojiArt = ({
                           char,
                           initValues
                         }: {
  char: string;
  initValues?: {
    options?: Partial<Grid.Options>;
    initialVal?: Partial<TextureStyleObjDto>;
  };
}) => {
  const smileyBallTemplate = TEXTURE_TYPES.SMILEY_BALL;
  const PIXELMOJIS = LIB_TYPES.PIXELMOJIS;

  const [{ lib, letterOptions, palette }] = useTextGrid(
    smileyBallTemplate,
    PIXELMOJIS,
    initValues
  );

  if (!lib) {
    return <></>;
  }

  return (
    <section className={styles.emojiArtWrapper}>
      <div className={styles.emojiArt}>
        {!!lib && Object.values(lib).length > 0
          ? lib[char].data?.map((el, index) => {
            return (
              <div key={`row-${index}`} className={styles.emojiArtBoxRow}>
                {el?.map((el2) => {
                  return el2?.map((el3, renderEmojiIndex) => {
                    if (!el3) {
                      return (
                        <span
                          key={`renderEmoji-${renderEmojiIndex}`}
                          style={{
                            display: "flex",
                            position: "relative",
                            height: "16px",
                            width: "16px"
                          }}
                        />
                      );
                    }
                    return (
                      <RenderEmoji
                        key={`renderEmoji-${renderEmojiIndex}`}
                        id={el3}
                        options={letterOptions}
                      />
                    );
                  });
                })}
              </div>
            );
          })
          : null}
      </div>
    </section>
  );
};
