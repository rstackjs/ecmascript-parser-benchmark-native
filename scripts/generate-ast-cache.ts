import { spawnSync } from "node:child_process";
const proc = spawnSync("./bin/rust", ["--generate-ast-cache", "files/typescript.js", "files/checker.ts", "files/lib.dom.d.ts", "files/react.js"], { stdio: "inherit" });
if (proc.error) throw proc.error;
if (proc.status !== 0) throw new Error("Oxc AST cache generation failed");
