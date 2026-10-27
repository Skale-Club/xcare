'use strict';
// UI-state tests run with Node's VM and an IPC/DOM stub; no desktop or browser.
const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');
const path = require('node:path');
const sourceDir = process.env.XCARE_SOURCE_DIR || path.join(__dirname, '..', 'src');

async function harness(options = {}) {
  const elements = new Map(), calls = [];
  function element(id) {
    if (!elements.has(id)) elements.set(id, {
      innerHTML: '', textContent: '', value: '', hidden: true, className: '', listeners: {},
      classList: { add() {}, remove() {}, toggle() {} }, focus() {},
      addEventListener(type, fn) { this.listeners[type] = fn; },
    });
    return elements.get(id);
  }
  let drop;
  let prepare = (args) => ({ token: 'review-token', paths: args.paths, files: 2, folders: 0, bytes: 10, blockers: [], blockerError: null });
  let execute = () => ({ deleted: 2, scheduled: 0, failed: 0, closed: 0, errors: [] });
  const context = vm.createContext({
    console, setTimeout, clearTimeout,
    document: { documentElement: {}, getElementById: element, querySelector: () => element('ver'), querySelectorAll: () => [], addEventListener() {}, removeEventListener() {} },
    window: { __TAURI__: {
      core: { async invoke(command, args) {
        calls.push({ command, args });
        if (command === 'prepare_force_delete') return prepare(args);
        if (command === 'execute_force_delete') return execute(args);
        if (command === 'pick_force_delete') return options.picked || [];
        return ({ get_settings: { language: 'en' }, get_status: { admin: options.admin ?? false, total: 0, free: 0, drive: 'C:', uptime: 0, memoryLoad: 0 }, scan_junk: [], list_startup: [], list_power_plans: [], check_update: null })[command];
      } },
      event: { listen: async () => () => {} },
      app: { getVersion: async () => '0.1.0' },
      webview: { getCurrentWebview: () => ({ onDragDropEvent: async (fn) => { drop = fn; return () => {}; } }) },
    } },
  });
  for (const name of ['i18n.js', 'force-delete.js', 'app.js']) vm.runInContext(fs.readFileSync(path.join(sourceDir, name), 'utf8'), context, { filename: name });
  await new Promise(setImmediate);
  vm.runInContext("go('forceDelete')", context);
  const api = context.window.XcareForceDelete;
  return {
    api, calls, elements, context,
    html: () => element('page').innerHTML,
    drop: (paths) => drop({ payload: { type: 'drop', paths } }),
    input: (value) => { element('fd-path').value = value; element('fd-path').listeners.input(); },
    configure: (config) => { if (config.prepare) prepare = config.prepare; if (config.execute) execute = config.execute; },
    respond: (confirmed) => element(confirmed ? 'modal-ok' : 'modal-cancel').onclick(),
    forced: () => calls.filter((c) => c.command.includes('force_delete')),
  };
}

test('opening the page and dropping files never triggers deletion or inspection', async () => {
  const h = await harness(); assert.equal(h.forced().length, 0);
  h.drop(['C:\\temp\\a.txt']); assert.equal(h.forced().length, 0); assert.match(h.html(), /a\.txt/);
  await h.api.act('inspect'); assert.equal(h.forced().length, 1); assert.equal(h.forced()[0].command, 'prepare_force_delete');
});

test('cancel keeps the selection and never sends a destructive command', async () => {
  const h = await harness(); h.drop(['C:\\temp\\a.txt']); await h.api.act('inspect');
  const pending = h.api.act('delete'); assert.equal(h.elements.get('modal').hidden, false); h.respond(false); await pending;
  assert.equal(h.calls.filter((c) => c.command === 'execute_force_delete').length, 0); assert.match(h.html(), /Ready for your review/);
});

test('confirmation sends only the reviewed token and explicit mode once', async () => {
  const h = await harness(); h.drop(['C:\\temp\\a.txt']); await h.api.act('inspect');
  const pending = h.api.act('delete'); await h.api.act('delete'); h.respond(true); await pending;
  const executions = h.calls.filter((c) => c.command === 'execute_force_delete'); assert.equal(executions.length, 1);
  assert.equal(executions[0].args.token, 'review-token'); assert.equal(executions[0].args.mode, 'delete');
  assert.match(h.html(), /2 deleted/); await h.api.act('delete'); assert.equal(h.calls.filter((c) => c.command === 'execute_force_delete').length, 1);
});

test('editing the selection invalidates the previous review', async () => {
  const h = await harness(); h.drop(['C:\\temp\\a.txt']); await h.api.act('inspect');
  h.input('C:\\temp\\b.txt'); await h.api.act('add'); await h.api.act('delete');
  assert.equal(h.calls.filter((c) => c.command === 'execute_force_delete').length, 0); assert.doesNotMatch(h.html(), /Ready for your review/);
});

test('picker cancellation preserves the reviewed selection', async () => {
  const h = await harness(); h.drop(['C:\\temp\\a.txt']); await h.api.act('inspect'); await h.api.act('files');
  assert.match(h.html(), /Ready for your review/); assert.match(h.html(), /a\.txt/);
});

test('inspection errors remain visible and selection can be retried', async () => {
  const h = await harness(); h.drop(['C:\\temp\\a.txt']); h.configure({ prepare: () => { throw new Error('fixture failure'); } });
  await h.api.act('inspect'); assert.match(h.html(), /role="alert"/); assert.match(h.html(), /fixture failure/);
  h.configure({ prepare: (a) => ({ token: 'retry', paths: a.paths, files: 1, folders: 0, bytes: 1, blockers: [], blockerError: null }) });
  await h.api.act('inspect'); assert.match(h.html(), /Ready for your review/);
});

test('partial results distinguish scheduled, deleted and failed items and escape errors', async () => {
  const h = await harness({ admin: true }); h.drop(['C:\\temp\\a.txt']); await h.api.act('inspect');
  h.configure({ execute: () => ({ deleted: 0, scheduled: 1, failed: 1, closed: 0, errors: [{ path: '<script>bad</script>', error: '<img src=x>' }] }) });
  const pending = h.api.act('reboot'); h.respond(true); await pending;
  assert.match(h.html(), /0 deleted · 1 scheduled · 1 failed/); assert.match(h.html(), /not deleted yet/);
  assert.doesNotMatch(h.html(), /<script>bad/); assert.match(h.html(), /&lt;script&gt;/);
});

test('protected processes and non-admin reboot actions are disabled', async () => {
  const h = await harness(); h.drop(['C:\\temp\\a.txt']);
  h.configure({ prepare: (a) => ({ token: 't', paths: a.paths, files: 1, folders: 0, bytes: 1, blockers: [{ pid: 123, name: 'Protected app', canTerminate: false }], blockerError: null }) });
  await h.api.act('inspect'); assert.match(h.html(), /data-fd="terminate" disabled/); assert.match(h.html(), /data-fd="reboot" disabled/);
});

test('EN, PT and ES feature strings have matching keys and placeholders', async () => {
  const h = await harness(); const strings = vm.runInContext('I18N', h.context);
  const keys = Object.keys(strings.en.ui).filter((k) => k.startsWith('fd') || k === 'forceDelete').sort();
  for (const lang of ['pt', 'es']) {
    assert.deepEqual(Object.keys(strings[lang].ui).filter((k) => k.startsWith('fd') || k === 'forceDelete').sort(), keys);
    for (const key of keys) assert.deepEqual([...strings[lang].ui[key].matchAll(/\{(\w+)\}/g)].map((m) => m[1]).sort(), [...strings.en.ui[key].matchAll(/\{(\w+)\}/g)].map((m) => m[1]).sort());
  }
});
