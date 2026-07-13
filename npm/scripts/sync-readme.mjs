import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const PROJECT_ROOT = path.resolve(__dirname, "../../");
const NPM_DIR = path.resolve(PROJECT_ROOT, "./npm");

const readme = fs.readFileSync(path.resolve(PROJECT_ROOT, "./README.md"), "utf8")
fs.writeFileSync(path.resolve(NPM_DIR, "./prebuilt-down/README.md"), readme)
