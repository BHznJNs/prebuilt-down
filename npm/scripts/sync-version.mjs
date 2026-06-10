import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function readCargoVersion() {
  const toml = fs.readFileSync(path.resolve(__dirname, "./Cargo.toml"), "utf8");
  const match = toml.match(/^version\s*=\s*"([^"]+)"/m);
  if (!match) throw new Error("Can not find 'version' field in Cargo.toml");
  return match[1];
}

const version = readCargoVersion();
if (!/^\d+\.\d+\.\d+/.test(version)) {
  console.error(`Invalid version code: ${version}`);
  process.exit(1);
}

const NPM_DIR = path.resolve(__dirname, "./npm");

const PACKAGES = [
  "prebuilt-down",
  "@prebuilt-down/win32-x64",
  // "@prebuilt-down/win32-arm64",
  // "@prebuilt-down/darwin-x64",
  // "@prebuilt-down/darwin-arm64",
  // "@prebuilt-down/linux-x64",
  // "@prebuilt-down/linux-arm64",
  // "@prebuilt-down/linux-x64-musl",
  // "@prebuilt-down/linux-arm64-musl",
];

console.log(`syncing version: ${version}\n`);

for (const pkgName of PACKAGES) {
  const pkgPath = path.join(NPM_DIR, pkgName, "package.json");
  const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf8"));

  const prev = pkg.version;
  pkg.version = version;

  if (pkg.optionalDependencies) {
    for (const dep of Object.keys(pkg.optionalDependencies)) {
      pkg.optionalDependencies[dep] = version;
    }
  }

  fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + "\n");
  console.log(`  ${pkgName.padEnd(48)} ${prev} → ${version}`);
}
