/* eslint-disable */

import fs from "fs";
import path from "path";
import JSON5 from "json5";

const assetsFilePath = path.join(process.cwd(), "dist/.ic-assets.json5");
if (fs.existsSync(assetsFilePath)) {
  let fileData = fs.readFileSync(assetsFilePath, "utf8");

  // Parsuj plik JSON5
  let assetsConfig = JSON5.parse(fileData);

  // Modyfikuj nagłówek Content-Security-Policy
  const newCSP = "default-src 'self'; script-src 'self'; connect-src *; img-src 'self' data:; style-src * 'unsafe-inline'; style-src-elem * 'unsafe-inline'; font-src *; object-src 'none'; base-uri 'self'; frame-ancestors 'none'; form-action 'self'; upgrade-insecure-requests;";
  assetsConfig.forEach((item) => {
    if (item.headers && item.headers["Content-Security-Policy"]) {
      item.headers["Content-Security-Policy"] = newCSP;
    }
  });

  // Dodaj nową konfigurację dla .well-known i ii-alternative-origins
  assetsConfig.push(
    {
      match: ".well-known",
      ignore: false
    },
    {
      match: ".well-known/ii-alternative-origins",
      headers: {
        "Access-Control-Allow-Origin": "*",
        "Content-Type": "application/json"
      },
      ignore: false
    }
  );

  // Zapisz zaktualizowany plik w formacie JSON5
  const updatedFileData = JSON5.stringify(assetsConfig, null, 2);
  fs.writeFileSync(assetsFilePath, updatedFileData);
  console.log("Plik .ic-assets.json5 został zaktualizowany.");
} else {
  console.error("Plik .ic-assets.json5 nie został znaleziony.");
}
