#!/usr/bin/env node
const { spawnSync, execSync } = require("node:child_process");
const { platform, arch, env } = process;

function isMusl() {
  let stderr;
  try {
    stderr = execSync("ldd --version", { stdio: ["pipe", "pipe", "pipe"] });
  } catch (err) {
    stderr = err.stderr;
  }
  return stderr.indexOf("musl") > -1;
}

const PLATFORMS = {
  win32: {
    x64: "@prebuilt-down/win32-x64/prebuilt-down.exe",
    arm64: "@prebuilt-down/win32-arm64/prebuilt-down.exe",
  },
  darwin: {
    x64: "@prebuilt-down/darwin-x64/prebuilt-down",
    arm64: "@prebuilt-down/darwin-arm64/prebuilt-down",
  },
  linux: {
    x64: "@prebuilt-down/linux-x64/prebuilt-down",
    arm64: "@prebuilt-down/linux-arm64/prebuilt-down",
  },
  "linux-musl": {
    x64: "@prebuilt-down/linux-x64-musl/prebuilt-down",
    arm64: "@prebuilt-down/linux-arm64-musl/prebuilt-down",
  },
};

const binPath =
  env.MY_CLI_BINARY ??
  (platform === "linux" && isMusl()
    ? PLATFORMS["linux-musl"]?.[arch]
    : PLATFORMS[platform]?.[arch]);

if (!binPath) {
  console.error(`Unsupported platform: ${platform}-${arch}`);
  process.exit(1);
}

const result = spawnSync(require.resolve(binPath), process.argv.slice(2), {
  shell: false,
  stdio: "inherit",
});

if (result.error) throw result.error;
process.exitCode = result.status;
