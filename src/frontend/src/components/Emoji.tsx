// @ts-nocheck
"use client";

import emojiMartData from "@emoji-mart/data";
import { init as emojiMartInit } from "emoji-mart";

emojiMartInit({ data: emojiMartData }).then((e) => console.log(e)); // 🍸

export function RenderEmoji({ id, options, className }: { id: string, options?: {skin?: string, size?: string; set?: string}, className?: string }) {

  return (
    !!id && (
      <div
        className={className}
        style={{
          display: "inline-flex",
          position: "relative",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <em-emoji
          id={id}
          skin={options?.skin || "1"}
          size={options?.size || "11px"}
          set={options?.set || "twitter"}
        />
      </div>
    )
  );
}
