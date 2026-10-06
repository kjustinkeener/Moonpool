#!/usr/bin/env node
// End-to-end test of a built Moonpool against throwaway PORTABLE copies in a temp folder.
// See README.md beside this file. Exits 0 when every scenario passes, 1 otherwise.
//
// Safety: every copy gets its `moonpool.portable` flag BEFORE it is first run (an unflagged
// exe would act as the installed copy and use the owner's bare channel), the harness refuses
// to talk to the installed copy's endpoint, and it only ever kills PIDs whose exe lives
// inside its own temp folder.

import { spawn } from "node:child_process";
import fs from "node:fs";
import net from "node:net";
import os from "node:os";
import path from "node:path";
import readline from "node:readline";
import { fileURLToPath } from "node:url";

import * as plat from "./platform.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(here, "..", "..");

// --------------------------------------------------------------------------------------------
// Options
// --------------------------------------------------------------------------------------------

const argv = process.argv.slice(2);
const opt = (name, dflt) => {
  const i = argv.indexOf(name);
  return i >= 0 && argv[i + 1] ? argv[i + 1] : dflt;
};
const EXE = path.resolve(opt("--exe", plat.defaultExe(repoRoot)));
const KEEP = argv.includes("--keep");
const ONLY = opt("--only", "");
const TMP_BASE = path.resolve(opt("--tmp", os.tmpdir()));

if (!fs.existsSync(EXE)) {
  console.error(`e2e: no exe at ${EXE} (build one first, see tools/e2e/README.md)`);
  process.exit(2);
}

// --------------------------------------------------------------------------------------------
// Small helpers
// --------------------------------------------------------------------------------------------

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function waitFor(what, fn, timeoutMs = 30000, everyMs = 250) {
  const deadline = Date.now() + timeoutMs;
  let last;
  while (Date.now() < deadline) {
    last = await fn();
    if (last) return last;
    await sleep(everyMs);
  }
  throw new Error(`timed out after ${timeoutMs / 1000}s waiting for ${what}`);
}

function assert(cond, msg) {
  if (!cond) throw new Error(msg);
}

const ROOT = fs.mkdtempSync(path.join(TMP_BASE, "moonpool-e2e-"));
const ROOT_KEY = plat.pathKey(ROOT);
const spawned = new Set(); // every PID we started directly

/** Every live process whose exe is inside our temp root: ours by construction. */
function ourProcesses(procs = plat.listProcesses()) {
  return procs.filter((p) => p.exe && plat.pathKey(p.exe).startsWith(ROOT_KEY + path.sep));
}

// --------------------------------------------------------------------------------------------
// Copies
// --------------------------------------------------------------------------------------------

/** Lay out `<ROOT>/<name>/.moonpool/<exe>` with the portable flag beside it. */
function makeCopy(name) {
  const appDir = path.join(ROOT, name, ".moonpool");
  fs.mkdirSync(appDir, { recursive: true });
  // The flag first: the exe must never run unflagged.
  fs.writeFileSync(path.join(appDir, "moonpool.portable"), "e2e test copy\n");
  const exe = path.join(appDir, plat.exeName);
  fs.copyFileSync(EXE, exe);
  if (!plat.isWindows) fs.chmodSync(exe, 0o755);
  const endpoint = plat.endpointFor(appDir);
  assert(
    !plat.forbiddenEndpoints().includes(endpoint),
    `refusing: ${name} would use the installed copy's endpoint ${endpoint}`,
  );
  return { name, appDir, exe, endpoint, label: `Moonpool (${name})` };
}

function run(copy, args = [], { stdio = "ignore" } = {}) {
  assert(fs.existsSync(path.join(copy.appDir, "moonpool.portable")), "portable flag missing");
  const child = spawn(copy.exe, args, { cwd: copy.appDir, stdio, windowsHide: false });
  spawned.add(child.pid);
  child.exitInfo = new Promise((resolve) =>
    child.on("exit", (code, signal) => resolve({ code, signal })),
  );
  child.on("error", () => {});
  return child;
}

// --------------------------------------------------------------------------------------------
// Control channel client (newline-delimited JSON over a named pipe / Unix socket)
// --------------------------------------------------------------------------------------------

function call(endpoint, cmd, args = [], timeoutMs = 5000) {
  if (plat.forbiddenEndpoints().includes(endpoint)) {
    throw new Error(`refusing to talk to the installed copy's endpoint ${endpoint}`);
  }
  return new Promise((resolve) => {
    let buf = "";
    let done = false;
    const finish = (v) => {
      if (done) return;
      done = true;
      clearTimeout(timer);
      sock.destroy();
      resolve(v);
    };
    const sock = net.connect(endpoint);
    const timer = setTimeout(() => finish({ error: "timeout" }), timeoutMs);
    sock.on("connect", () => sock.write(JSON.stringify({ cmd, args }) + "\n"));
    sock.on("data", (d) => {
      buf += d.toString("utf8");
      const nl = buf.indexOf("\n");
      if (nl >= 0) {
        try {
          finish({ reply: JSON.parse(buf.slice(0, nl)) });
        } catch (e) {
          finish({ error: `bad reply: ${e.message}` });
        }
      }
    });
    sock.on("error", (e) =>
      finish({ error: ["ENOENT", "ECONNREFUSED"].includes(e.code) ? "down" : e.code || e.message }),
    );
    sock.on("close", () => finish({ error: "closed" }));
  });
}

async function ok(endpoint, cmd, args = [], timeoutMs) {
  const r = await call(endpoint, cmd, args, timeoutMs);
  assert(r.reply, `${cmd}: no reply (${r.error})`);
  assert(r.reply.ok === true, `${cmd}: ${JSON.stringify(r.reply)}`);
  return r.reply.result;
}

const pingUp = async (copy) => (await call(copy.endpoint, "ping", [], 2000)).reply?.ok === true;
const pingDown = async (copy) => (await call(copy.endpoint, "ping", [], 2000)).error === "down";

/** The live hub process of a copy: a process of its exe with no arguments. */
function hubPids(copy, procs = plat.listProcesses()) {
  const key = plat.pathKey(copy.exe);
  return procs.filter((p) => p.exe && plat.pathKey(p.exe) === key).map((p) => p.pid);
}

async function startHub(copy, args = []) {
  const child = run(copy, args);
  await waitFor(`${copy.name} to answer ping`, () => pingUp(copy), 45000);
  return child;
}

async function quitHub(copy, pid) {
  await call(copy.endpoint, "quit", [], 3000); // usually no reply: the hub exits first
  await waitFor(`${copy.name} channel to go down`, () => pingDown(copy), 20000);
  if (pid) await waitFor(`${copy.name} hub pid ${pid} to exit`, () => !plat.isAlive(pid), 20000);
}

/** The frontend reports ready a moment after the channel binds; UI-owned verbs need it. */
async function waitFrontend(copy) {
  await waitFor(`${copy.name} frontend`, async () => {
    const r = await call(copy.endpoint, "reload", [], 50000);
    return r.reply?.ok === true;
  }, 60000, 1000);
}

// --------------------------------------------------------------------------------------------
// MCP stdio client
// --------------------------------------------------------------------------------------------

function mcpClient(copy) {
  const child = run(copy, ["mcp"], { stdio: ["pipe", "pipe", "ignore"] });
  const pending = new Map();
  let next = 1;
  readline.createInterface({ input: child.stdout }).on("line", (line) => {
    let msg;
    try {
      msg = JSON.parse(line);
    } catch {
      return;
    }
    const p = pending.get(msg.id);
    if (p) {
      pending.delete(msg.id);
      p(msg);
    }
  });
  const request = (method, params = {}, timeoutMs = 60000) =>
    new Promise((resolve, reject) => {
      const id = next++;
      const t = setTimeout(() => reject(new Error(`mcp ${method}: no answer`)), timeoutMs);
      pending.set(id, (m) => {
        clearTimeout(t);
        resolve(m);
      });
      child.stdin.write(JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n");
    });
  const tool = async (name, args = {}) => {
    const m = await request("tools/call", { name, arguments: args });
    const text = (m.result?.content || []).map((c) => c.text || "").join("\n");
    return { text, isError: m.result?.isError === true };
  };
  const close = async () => {
    child.stdin.end();
    await Promise.race([child.exitInfo, sleep(5000)]);
    plat.killTree(child.pid);
  };
  return { child, request, tool, close };
}

// --------------------------------------------------------------------------------------------
// Scenarios
// --------------------------------------------------------------------------------------------

const A = makeCopy("CopyA");
const B = makeCopy("CopyB");
let hubA = null;
let hubB = null;

const scenarios = [];
const scenario = (name, fn) => scenarios.push({ name, fn });

scenario("copies get distinct per-copy channels", async () => {
  assert(A.endpoint !== B.endpoint, "A and B derived the same endpoint");
  for (const c of [A, B]) assert(await pingDown(c), `${c.name} channel already in use`);
});

scenario("two copies run side by side, each on its own channel", async () => {
  hubA = await startHub(A);
  hubB = await startHub(B);
  for (const [c, other] of [
    [A, B],
    [B, A],
  ]) {
    assert((await ok(c.endpoint, "ping")) === "pong", `${c.name} ping`);
    const paths = await ok(c.endpoint, "paths");
    assert(paths.includes(`hub channel:    ${c.endpoint}`), `${c.name} reports another channel:\n${paths}`);
    assert(paths.includes(`hub copy:       ${c.label}`), `${c.name} label wrong:\n${paths}`);
    assert(paths.includes("hub portable:   true"), `${c.name} not portable:\n${paths}`);
    assert(paths.includes(c.appDir), `${c.name} paths not inside its folder`);
    assert(!paths.includes(other.appDir), `${c.name} paths mention ${other.name}`);
  }
});

scenario("list verb returns the seeded manifest", async () => {
  const snap = JSON.parse(await ok(A.endpoint, "list"));
  const ids = (snap.apps || []).map((a) => a.id);
  assert(ids.includes(plat.cliApp.id), `no apps: ${JSON.stringify(snap)}`);
});

scenario("second launch of the same copy forwards and exits", async () => {
  const before = hubPids(A);
  assert(before.length === 1, `expected one A hub, found ${before}`);
  for (const args of [[], ["show"]]) {
    const second = run(A, args);
    const exit = await Promise.race([second.exitInfo, sleep(25000).then(() => null)]);
    assert(exit, `second launch (${args.join(" ") || "bare"}) did not exit`);
    assert(exit.code === 0, `second launch exited ${exit.code}`);
  }
  const after = hubPids(A);
  assert(after.length === 1 && after[0] === before[0], `hub set changed: ${before} -> ${after}`);
  assert(await pingUp(B), "B disturbed by A's second launch");
});

scenario("a forwarded argv action reaches the hub (launch / stop)", async () => {
  await waitFrontend(A);
  const appRunning = async () => {
    const snap = JSON.parse(await ok(A.endpoint, "list"));
    return (snap.statuses || []).find((s) => s.id === plat.cliApp.id)?.running === true;
  };
  for (const [verb, want] of [
    ["launch", true],
    ["stop", false],
  ]) {
    const fwd = run(A, [verb, plat.cliApp.id]);
    const exit = await Promise.race([fwd.exitInfo, sleep(25000).then(() => null)]);
    assert(exit && exit.code === 0, `forwarded ${verb} exit: ${JSON.stringify(exit)}`);
    await waitFor(`${plat.cliApp.id} running=${want}`, async () => (await appRunning()) === want, 20000, 500);
  }
  assert(hubPids(A).length === 1, "forwarding left an extra A process");
});

scenario("two simultaneous launches of a stopped copy end with one hub", async () => {
  await quitHub(B, hubPids(B)[0]);
  hubB = null;
  const one = run(B);
  const two = run(B);
  await waitFor("B to answer ping", () => pingUp(B), 45000);
  const loser = await Promise.race([
    one.exitInfo.then((e) => ({ who: one, e })),
    two.exitInfo.then((e) => ({ who: two, e })),
    sleep(30000).then(() => null),
  ]);
  assert(loser, "neither racing launch exited");
  assert(loser.e.code === 0, `the forwarding launch exited ${loser.e.code}`);
  const winner = loser.who === one ? two : one;
  const hubs = hubPids(B);
  assert(hubs.length === 1 && hubs[0] === winner.pid, `B hubs ${hubs}, expected [${winner.pid}]`);
  hubB = winner;
});

scenario("mcp from each copy talks only to its own hub", async () => {
  for (const [c, other] of [
    [A, B],
    [B, A],
  ]) {
    const m = mcpClient(c);
    try {
      const init = await m.request("initialize", {
        protocolVersion: "2024-11-05",
        capabilities: {},
        clientInfo: { name: "e2e", version: "0" },
      });
      assert(init.result?.serverInfo?.name === `moonpool (${c.name})`, `serverInfo: ${JSON.stringify(init.result?.serverInfo)}`);
      m.child.stdin.write(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }) + "\n");
      const tools = await m.request("tools/list");
      const names = (tools.result?.tools || []).map((t) => t.name);
      for (const t of ["moonpool_list_apps", "moonpool_launcher_paths", "moonpool_bootup_launcher"]) {
        assert(names.includes(t), `tools/list missing ${t}`);
      }
      const paths = await m.tool("moonpool_launcher_paths");
      assert(!paths.isError, paths.text);
      assert(paths.text.includes(`hub channel:    ${c.endpoint}`), `${c.name} mcp reached another hub:\n${paths.text}`);
      assert(!paths.text.includes(other.appDir), `${c.name} mcp mentions ${other.name}`);
      const list = await m.tool("moonpool_list_apps");
      assert(!list.isError && list.text.includes(plat.cliApp.id), `list_apps: ${list.text}`);
      const raise = await m.tool("moonpool_raise_launcher");
      assert(!raise.isError, `raise: ${raise.text}`);
    } finally {
      await m.close();
    }
  }
});

scenario("example dashboards and help are seeded", async () => {
  const ex = path.join(A.appDir, "dashboards", "examples");
  assert(fs.existsSync(path.join(ex, "README.md")), "dashboards/examples/README.md missing");
  assert(fs.readFileSync(path.join(ex, ".moonpool-version"), "utf8").trim().length > 0, "no stamp");
  const help = path.join(A.appDir, "help");
  assert(fs.existsSync(path.join(help, "index.html")), "help/index.html missing");
  assert(fs.existsSync(path.join(help, "version.txt")), "help/version.txt missing");
  assert(fs.existsSync(path.join(A.appDir, "moonpool-config", "apps.json")), "apps.json not seeded");
});

scenario("launched app child dies when the hub quits", async () => {
  await waitFrontend(A);
  await ok(A.endpoint, "launch", [plat.cliApp.id], 55000);
  const hub = hubPids(A)[0];
  const kids = await waitFor("cli app child under A", () => {
    const procs = plat.listProcesses();
    const ds = plat.descendants(hub, procs);
    const ps = procs.filter((p) => ds.includes(p.pid) && plat.cliApp.process.test(p.name));
    return ps.length ? ps.map((p) => p.pid) : null;
  }, 20000, 500);
  const snap = JSON.parse(await ok(A.endpoint, "list"));
  const st = (snap.statuses || []).find((s) => s.id === plat.cliApp.id);
  assert(st?.running === true, `list does not show it running: ${JSON.stringify(st)}`);
  await quitHub(A, hub);
  hubA = null;
  try {
    await waitFor("launched child to exit", () => kids.every((k) => !plat.isAlive(k)), 15000);
  } finally {
    kids.forEach((k) => plat.killTree(k)); // ours (started by our hub); no-op when gone
  }
});

scenario("quit frees the lock: relaunch becomes the hub", async () => {
  if (await pingUp(A)) await quitHub(A, hubPids(A)[0]);
  hubA = await startHub(A);
  assert(hubPids(A).includes(hubA.pid), "relaunched process is not the hub");
});

scenario("--wait-pid relaunch takes over after the old hub exits", async () => {
  const old = hubPids(A)[0];
  const relaunch = run(A, ["--wait-pid", String(old)]);
  await sleep(1500);
  assert(plat.isAlive(relaunch.pid), "--wait-pid relaunch exited while the old hub was alive");
  await quitHub(A, old);
  await waitFor("relaunch to answer ping", () => pingUp(A), 45000);
  const now = hubPids(A);
  assert(now.length === 1 && now[0] === relaunch.pid, `hub is ${now}, expected ${relaunch.pid}`);
  hubA = relaunch;
});

scenario("seeds are replaced on a stamp change; user dashboards survive", async () => {
  const dash = path.join(A.appDir, "dashboards");
  const ex = path.join(dash, "examples");
  const help = path.join(A.appDir, "help");
  const stamp = fs.readFileSync(path.join(ex, ".moonpool-version"), "utf8").trim();
  const helpStamp = fs.readFileSync(path.join(help, "version.txt"), "utf8").trim();
  await quitHub(A, hubPids(A)[0]);
  fs.writeFileSync(path.join(dash, "mine.html"), "user file");
  fs.mkdirSync(path.join(dash, "my-board"), { recursive: true });
  fs.writeFileSync(path.join(dash, "my-board", "index.html"), "user board");
  fs.writeFileSync(path.join(ex, "stale.txt"), "left by an older build");
  fs.writeFileSync(path.join(ex, ".moonpool-version"), "0.0.0-old");
  fs.writeFileSync(path.join(help, "stale.html"), "old page");
  fs.writeFileSync(path.join(help, "version.txt"), "0.0.0-old");
  hubA = await startHub(A);
  await waitFor("dashboards re-seed", () =>
    fs.readFileSync(path.join(ex, ".moonpool-version"), "utf8").trim() === stamp, 15000);
  assert(!fs.existsSync(path.join(ex, "stale.txt")), "stale example file survived the re-seed");
  assert(fs.existsSync(path.join(ex, "README.md")), "examples README missing after re-seed");
  assert(fs.readFileSync(path.join(dash, "mine.html"), "utf8") === "user file", "user file touched");
  assert(fs.existsSync(path.join(dash, "my-board", "index.html")), "user board touched");
  assert(fs.readFileSync(path.join(help, "version.txt"), "utf8").trim() === helpStamp, "help stamp not restored");
  assert(!fs.existsSync(path.join(help, "stale.html")), "stale help page survived");
  assert(fs.existsSync(path.join(help, "index.html")), "help index missing after re-seed");
});

/** `moonpool_list_apps` through a one-shot `moonpool.exe mcp` of this copy. */
async function mcpListApps(copy) {
  const m = mcpClient(copy);
  try {
    await m.request("initialize", { protocolVersion: "2024-11-05", capabilities: {}, clientInfo: { name: "e2e", version: "0" } });
    return await m.tool("moonpool_list_apps");
  } finally {
    await m.close();
  }
}

scenario("a broken apps.json keeps the last list and is reported", async () => {
  if (!(await pingUp(B))) hubB = await startHub(B);
  const file = path.join(B.appDir, "moonpool-config", "apps.json");
  const good = fs.readFileSync(file, "utf8");
  await waitFrontend(B); // a reload of the good file: also proves the UI is up
  try {
    // Reload with a broken file: refused, the last list stays, list + mcp say so.
    fs.writeFileSync(file, "[ { broken");
    const r = await call(B.endpoint, "reload", [], 50000);
    assert(r.reply?.ok === false, `reload of a broken file: ${JSON.stringify(r)}`);
    assert(/apps\.json has an error/.test(r.reply.error), `reload error: ${r.reply.error}`);
    let snap = JSON.parse(await ok(B.endpoint, "list"));
    assert(typeof snap.manifestError === "string" && snap.manifestError, `no manifestError: ${JSON.stringify(snap)}`);
    assert(snap.manifestLoaded === undefined, `manifestLoaded set after a good load: ${snap.manifestLoaded}`);
    assert((snap.apps || []).some((a) => a.id === plat.cliApp.id), "last good list was dropped");
    let list = await mcpListApps(B);
    assert(!list.isError && list.text.startsWith("apps.json has an error:"), `list_apps: ${list.text}`);
    assert(list.text.includes("last one that loaded") && list.text.includes(plat.cliApp.id), list.text);

    // Restart with the file still broken: no list at all, flagged as never loaded.
    await quitHub(B, hubPids(B)[0]);
    hubB = await startHub(B);
    snap = JSON.parse(await ok(B.endpoint, "list"));
    assert(snap.manifestError && snap.manifestLoaded === false, `startup: ${JSON.stringify(snap)}`);
    assert((snap.apps || []).length === 0, "apps listed from a broken file");
    list = await mcpListApps(B);
    assert(list.text.includes("no apps are loaded"), `list_apps at startup: ${list.text}`);
  } finally {
    fs.writeFileSync(file, good);
  }
  // Fixed and reloaded: the error clears.
  await waitFrontend(B);
  const snap = JSON.parse(await ok(B.endpoint, "list"));
  assert(snap.manifestError === undefined, `error survived a good reload: ${snap.manifestError}`);
  assert((snap.apps || []).some((a) => a.id === plat.cliApp.id), "list not back after the fix");
});

scenario("mcp bootup/shutdown drive only their own copy", async () => {
  await quitHub(B, hubPids(B)[0]);
  hubB = null;
  const m = mcpClient(B);
  try {
    await m.request("initialize", { protocolVersion: "2024-11-05", capabilities: {}, clientInfo: { name: "e2e", version: "0" } });
    const down = await m.tool("moonpool_list_apps");
    assert(down.isError && down.text.includes(`${B.label} is not running`), `list while down: ${down.text}`);
    const up = await m.tool("moonpool_bootup_launcher");
    assert(!up.isError, `bootup: ${up.text}`);
    assert(await pingUp(B), "B not up after bootup");
    assert(await pingUp(A), "A disturbed by B's bootup");
    const stop = await m.tool("moonpool_shutdown_launcher");
    assert(!stop.isError && /shut down/.test(stop.text), `shutdown: ${stop.text}`);
    assert(await pingDown(B), "B still up after shutdown");
    assert(await pingUp(A), "A disturbed by B's shutdown");
  } finally {
    await m.close();
  }
});

scenario("quit via the channel ends the hub", async () => {
  await quitHub(A, hubPids(A)[0]);
  hubA = null;
  assert(hubPids(A).length === 0, "an A hub process is still running");
});

// --------------------------------------------------------------------------------------------
// Runner
// --------------------------------------------------------------------------------------------

async function cleanup() {
  for (const c of [A, B]) {
    if (await pingUp(c)) await call(c.endpoint, "quit", [], 3000);
  }
  await sleep(1500);
  const left = new Set([...spawned].filter(plat.isAlive));
  for (const p of ourProcesses()) left.add(p.pid);
  for (const pid of left) plat.killTree(pid);
  await sleep(500);
  const still = ourProcesses();
  if (still.length) console.error(`e2e: processes still running from ${ROOT}: ${still.map((p) => p.pid)}`);
  if (!KEEP) {
    for (let i = 0; i < 20; i++) {
      try {
        fs.rmSync(ROOT, { recursive: true, force: true });
        break;
      } catch {
        await sleep(500); // WebView2 can hold its profile for a moment after exit
      }
    }
  }
  return still.length === 0;
}

let failed = 0;
const started = Date.now();
console.log(`e2e: exe ${EXE}\ne2e: temp ${ROOT}\ne2e: A ${A.endpoint}  B ${B.endpoint}`);
try {
  for (const s of scenarios) {
    if (ONLY && !s.name.includes(ONLY)) continue;
    const t0 = Date.now();
    try {
      await s.fn();
      console.log(`PASS  ${s.name}  (${((Date.now() - t0) / 1000).toFixed(1)}s)`);
    } catch (e) {
      failed++;
      console.log(`FAIL  ${s.name}\n      ${String(e.message).split("\n").join("\n      ")}`);
    }
  }
} finally {
  const clean = await cleanup();
  if (!clean) failed++;
  console.log(`e2e: ${failed ? `${failed} failure(s)` : "all passed"} in ${((Date.now() - started) / 1000).toFixed(0)}s`);
  process.exit(failed ? 1 : 0);
}
