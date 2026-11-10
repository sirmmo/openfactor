// WebMCP: lets an AI agent running in the browser look at and build the
// factory, through the page's model context (https://webmachinelearning.github.io/webmcp/).
//
// Every tool goes through the same calls as the mouse and keyboard, so an
// agent can do nothing a player couldn't, and pays the same costs. The API
// only exists in secure contexts (HTTPS or localhost) in browsers that ship
// it or load a polyfill; elsewhere this module does nothing.

const FACINGS = ['north', 'east', 'south', 'west'];
const MAX_AREA = 48;
const MAX_COUNT = 64;
const MAX_WAIT_SECONDS = 120;

/** The page's model context: the current spec, or the older navigator form. */
function modelContext() {
  if (typeof document !== 'undefined' && document.modelContext && document.modelContext.registerTool) {
    return { context: document.modelContext, legacy: false };
  }
  if (typeof navigator !== 'undefined' && navigator.modelContext && navigator.modelContext.registerTool) {
    return { context: navigator.modelContext, legacy: true };
  }
  return null;
}

class ToolError extends Error {}

function int(input, name, fallback) {
  const value = input[name] === undefined ? fallback : input[name];
  if (!Number.isInteger(value)) throw new ToolError(name + ' must be a whole number');
  return value;
}

/**
 * Builds the tool list. `game` is what the page shares: the wasm exports, the
 * pack, the describe module, and a few of its helpers.
 */
export function makeTools(game) {
  const { of, pack, D, beltCount } = game;
  const resourceName = (id) => pack.resources[id].name;
  const tickSeconds = () => Math.floor(of.of_tick() / D.TICKS_PER_SECOND);
  const isOpen = (lock) => lock === null || of.of_researched(lock) === 1;
  const stockOf = (id) => of.of_stock(id) >>> 0;
  const kindName = (kind) => (kind < beltCount ? pack.belts[kind].name : pack.units[kind - beltCount].name);
  const kindId = (kind) => (kind < beltCount ? pack.belts[kind].id : pack.units[kind - beltCount].id);
  const normal = (text) => String(text).toLowerCase().replace(/[^a-z0-9]/g, '');

  function buildingKind(name) {
    const wanted = normal(name);
    const belt = pack.belts.findIndex((b) => normal(b.id) === wanted || normal(b.name) === wanted);
    if (belt >= 0) return belt;
    const unit = pack.units.findIndex((u) => normal(u.id) === wanted || normal(u.name) === wanted);
    if (unit >= 0) return beltCount + unit;
    throw new ToolError('No building called "' + name + '". openfactor_catalogue lists them.');
  }
  function facing(input) {
    const value = input.facing === undefined ? 'east' : String(input.facing).toLowerCase();
    const dir = FACINGS.indexOf(value);
    if (dir < 0) throw new ToolError('facing must be north, east, south or west');
    return dir;
  }
  function level(input) {
    const z = int(input, 'level', 0);
    if (z < 0 || z >= pack.levels) throw new ToolError('level must be from 0 to ' + (pack.levels - 1));
    return z;
  }
  // In someone else's shared game, edits go to the host's browser.
  const remote = () => !!(game.remote && game.remote());
  const SENT = 'sent';
  // Applies queued edits in one tick and reports whether they were refused.
  function commit() {
    if (remote()) return SENT;
    const refused = of.of_step(1);
    game.refresh();
    return refused > 0 ? game.rejectText[of.of_last_reject()] || 'Refused.' : null;
  }
  const costList = (cost) => cost.map(([id, count]) => ({ resource: resourceName(id), amount: count, have: stockOf(id) }));

  function stock() {
    const out = {};
    pack.resources.forEach((resource, id) => { const held = stockOf(id); if (held) out[resource.name] = held; });
    return out;
  }
  function researchState(index) {
    if (of.of_researched(index) === 1) return 'done';
    if (of.of_research_active() === index) return 'in progress';
    return pack.techs[index].requires.every((t) => of.of_researched(t) === 1) ? 'available' : 'needs earlier research';
  }

  const tools = [];
  const tool = (definition) => tools.push(definition);

  tool({
    name: 'openfactor_overview',
    title: 'Factory overview',
    description: 'Summary of the factory: time, the shared stock that pays for buildings and research, research progress, how many of each building there are and which have problems. Start here.',
    inputSchema: { type: 'object', properties: {} },
    annotations: { readOnlyHint: true },
    execute: () => {
      game.refresh();
      const counts = {}, problems = [];
      for (const b of game.buildings().values()) {
        const name = kindName(b.kind);
        counts[name] = (counts[name] || 0) + 1;
        const status = b.kind >= beltCount ? game.statusText[b.status] : null;
        if ([2, 3, 4, 6].includes(b.status) && b.kind >= beltCount) {
          problems.push({ building: name, x: b.x, y: b.y, level: b.z, status });
        }
      }
      const active = of.of_research_active();
      return {
        seconds: tickSeconds(),
        tick: of.of_tick(),
        stock: stock(),
        research: {
          running: active >= 0 ? { name: pack.techs[active].name, secondsLeft: Math.ceil(of.of_research_left() / 60) } : null,
          done: pack.techs.filter((_, i) => of.of_researched(i) === 1).map((t) => t.name),
          available: pack.techs.map((t, i) => [t, i]).filter(([, i]) => researchState(i) === 'available').map(([t]) => t.name),
        },
        buildings: counts,
        problems: problems.slice(0, 40),
        totals: Object.fromEntries(pack.metrics.map((m, i) => [m.name, of.of_metric(i) >>> 0]).filter(([, v]) => v)),
        levels: pack.levels,
        howToPay: 'Buildings and research are paid from the shared stock. Belt items into a depot to add to it. Erasing a building refunds its cost.',
      };
    },
  });

  tool({
    name: 'openfactor_catalogue',
    title: 'Buildings, research and resources',
    description: 'Everything in the rules: each building with its description, cost, recipes and placement rules and whether it is unlocked; each research with cost, requirements and what it unlocks; each resource.',
    inputSchema: {
      type: 'object',
      properties: { section: { type: 'string', enum: ['buildings', 'research', 'resources'], description: 'Which part to list. Defaults to buildings.' } },
    },
    annotations: { readOnlyHint: true },
    execute: (input = {}) => {
      const section = input.section || 'buildings';
      if (section === 'resources') return pack.resources.map((r) => ({ name: r.name, about: r.about, inStock: stockOf(pack.resources.indexOf(r)) }));
      if (section === 'research') {
        const unlocks = D.unlocksByTech(pack);
        return pack.techs.map((tech, index) => ({
          name: tech.name,
          about: tech.about,
          state: researchState(index),
          cost: costList(tech.cost),
          seconds: tech.time / D.TICKS_PER_SECOND,
          requires: tech.requires.map((t) => pack.techs[t].name),
          unlocks: unlocks[index].map((u) => u.name),
        }));
      }
      if (section !== 'buildings') throw new ToolError('section must be buildings, research or resources');
      const belts = pack.belts.map((belt) => ({
        id: belt.id, name: belt.name, section: 'Logistics', about: belt.about, unlocked: isOpen(belt.lock),
        cost: costList(belt.cost), facts: D.beltFacts(pack, belt),
      }));
      const units = pack.units.map((unit) => ({
        id: unit.id, name: unit.name, section: unit.group, about: unit.about, unlocked: isOpen(unit.lock),
        cost: costList(unit.cost),
        recipes: unit.rules.map((rule) => {
          const text = D.recipeText(pack, rule, unit);
          return { name: D.recipeName(pack, rule), does: text.line, notes: text.notes, unlocked: isOpen(rule.lock) };
        }),
        facts: D.unitFacts(pack, unit),
      }));
      return [...belts, ...units];
    },
  });

  tool({
    name: 'openfactor_look',
    title: 'Look at an area',
    description: 'What is in a rectangle of the map on one level: the ground (ore fields, water, oil), smog, and every building with the way it faces and its status. x grows to the east, y to the south. At most 48 by 48 tiles.',
    inputSchema: {
      type: 'object',
      properties: {
        x: { type: 'integer', description: 'West edge.' },
        y: { type: 'integer', description: 'North edge.' },
        width: { type: 'integer', minimum: 1, maximum: MAX_AREA },
        height: { type: 'integer', minimum: 1, maximum: MAX_AREA },
        level: { type: 'integer', minimum: 0, description: 'Level to look at; 0 is the ground.' },
      },
      required: ['x', 'y', 'width', 'height'],
    },
    annotations: { readOnlyHint: true },
    execute: (input = {}) => {
      const x0 = int(input, 'x'), y0 = int(input, 'y'), z = level(input);
      const width = int(input, 'width'), height = int(input, 'height');
      if (width < 1 || height < 1 || width > MAX_AREA || height > MAX_AREA) throw new ToolError('width and height must be 1 to ' + MAX_AREA);
      game.refresh();
      const inside = (x, y) => x >= x0 && x < x0 + width && y >= y0 && y < y0 + height;
      const ground = [];
      for (const [where, layer] of game.ground()) {
        const [x, y] = where.split(',').map(Number);
        if (inside(x, y)) ground.push({ x, y, ground: pack.layers[layer].name });
      }
      const smog = game.haze().filter((h) => inside(h.x, h.y)).map((h) => ({ x: h.x, y: h.y, layer: pack.layers[h.layer].name, percent: Math.round(h.strength / 2.55) }));
      const buildings = [];
      for (const b of game.buildings().values()) {
        if (b.z !== z || !inside(b.x, b.y)) continue;
        const entry = { x: b.x, y: b.y, building: kindId(b.kind), name: kindName(b.kind), facing: FACINGS[b.dir] };
        if (b.kind >= beltCount) entry.status = game.statusText[b.status];
        buildings.push(entry);
      }
      return { level: z, ground, smog, buildings, note: 'Tiles not listed are empty ground.' };
    },
  });

  tool({
    name: 'openfactor_inspect',
    title: 'Inspect a building',
    description: 'Everything about one building: what it is doing and why, its recipe, what it holds, and the networks it is on.',
    inputSchema: {
      type: 'object',
      properties: { x: { type: 'integer' }, y: { type: 'integer' }, level: { type: 'integer', minimum: 0 } },
      required: ['x', 'y'],
    },
    annotations: { readOnlyHint: true },
    execute: (input = {}) => {
      const x = int(input, 'x'), y = int(input, 'y'), z = level(input);
      const view = game.inspect({ x, y, z });
      if (!view) return { empty: true, x, y, level: z };
      const out = { x, y, level: z, building: kindId(view.kind), name: kindName(view.kind), facing: FACINGS[view.dir] };
      if (view.kind < beltCount) return out;
      const unit = pack.units[view.kind - beltCount];
      out.status = (game.statusText[view.status] || '').replace('%', view.missing >= 0 ? resourceName(view.missing) : 'input');
      if (unit.route === 'filter') out.sendsStraightOn = view.setting === null ? 'everything' : resourceName(view.setting);
      else if (unit.rules.length > 1) out.recipe = view.setting === null ? 'automatic' : D.recipeName(pack, unit.rules[view.setting]);
      if (view.job) {
        const rule = unit.rules[view.job[0]];
        out.working = { on: D.recipeName(pack, rule), secondsLeft: Math.round(view.job[1] / 6) / 10 };
      }
      out.holds = Object.fromEntries(view.held.map(([id, count]) => [resourceName(id), count]));
      out.networks = view.networks.map(([id, members, producers, stored]) => ({ resource: resourceName(id), members, producers, stored }));
      if (view.sendsTo !== null) out.sendsToLevel = view.sendsTo;
      if (view.receivesFrom !== null) out.receivesFromLevel = view.receivesFrom;
      return out;
    },
  });

  tool({
    name: 'openfactor_build',
    title: 'Build',
    description: 'Builds one building, or a straight row of them (count) starting at x, y and running the way it faces: the usual way to lay a belt. Costs come from the shared stock. Machines send their output out of their front, so face them toward the belt or building that takes it. Each placement is reported as built or refused, with the reason.',
    inputSchema: {
      type: 'object',
      properties: {
        building: { type: 'string', description: 'Building id or name from openfactor_catalogue, such as "belt" or "Coal furnace".' },
        x: { type: 'integer' },
        y: { type: 'integer' },
        level: { type: 'integer', minimum: 0, description: 'Defaults to 0, the ground.' },
        facing: { type: 'string', enum: FACINGS, description: 'Defaults to east.' },
        count: { type: 'integer', minimum: 1, maximum: MAX_COUNT, description: 'How many in a row. Defaults to 1.' },
      },
      required: ['building', 'x', 'y'],
    },
    execute: (input = {}) => {
      const kind = buildingKind(input.building);
      const dir = facing(input), z = level(input);
      let x = int(input, 'x'), y = int(input, 'y');
      const count = int(input, 'count', 1);
      if (count < 1 || count > MAX_COUNT) throw new ToolError('count must be 1 to ' + MAX_COUNT);
      const [dx, dy] = [[0, -1], [1, 0], [0, 1], [-1, 0]][dir];
      const results = [];
      for (let i = 0; i < count; i++, x += dx, y += dy) {
        of.of_place(x, y, z, dir, kind);
        const refused = commit();
        results.push(refused === SENT ? { x, y, sentToHost: true } : refused ? { x, y, refused } : { x, y, built: true });
      }
      if (remote()) return { sentToHost: count, note: 'This is a shared game run by the host. Check the result with openfactor_look.' };
      const built = results.filter((r) => r.built).length;
      game.notify('An assistant built ' + built + ' × ' + kindName(kind) + (built < count ? ' (' + (count - built) + ' refused)' : '') + '.');
      return { built, refused: count - built, results };
    },
  });

  tool({
    name: 'openfactor_remove',
    title: 'Remove',
    description: 'Removes every building in a rectangle on one level and refunds its cost. Whatever the buildings held is lost.',
    inputSchema: {
      type: 'object',
      properties: {
        x: { type: 'integer' }, y: { type: 'integer' },
        width: { type: 'integer', minimum: 1, maximum: MAX_AREA, description: 'Defaults to 1.' },
        height: { type: 'integer', minimum: 1, maximum: MAX_AREA, description: 'Defaults to 1.' },
        level: { type: 'integer', minimum: 0 },
      },
      required: ['x', 'y'],
    },
    annotations: { consequentialHint: true },
    execute: (input = {}) => {
      const x0 = int(input, 'x'), y0 = int(input, 'y'), z = level(input);
      const width = int(input, 'width', 1), height = int(input, 'height', 1);
      if (width < 1 || height < 1 || width > MAX_AREA || height > MAX_AREA) throw new ToolError('width and height must be 1 to ' + MAX_AREA);
      game.refresh();
      const targets = [...game.buildings().values()].filter((b) => b.z === z && b.x >= x0 && b.x < x0 + width && b.y >= y0 && b.y < y0 + height);
      let removed = 0;
      const refused = [];
      for (const b of targets) {
        of.of_remove(b.x, b.y, b.z);
        const reason = commit();
        if (reason && reason !== SENT) refused.push({ x: b.x, y: b.y, refused: reason }); else removed++;
      }
      if (removed) game.notify('An assistant removed ' + removed + ' building' + (removed === 1 ? '' : 's') + '.');
      return { removed, refused };
    },
  });

  tool({
    name: 'openfactor_configure',
    title: 'Choose a recipe or filter',
    description: 'For a machine with several recipes, picks the recipe by what it makes (or "automatic" to run the first it has inputs for). For a sorter, picks the resource that goes straight on (or "everything").',
    inputSchema: {
      type: 'object',
      properties: {
        x: { type: 'integer' }, y: { type: 'integer' }, level: { type: 'integer', minimum: 0 },
        choice: { type: 'string', description: 'A recipe or resource name, "automatic" or "everything".' },
      },
      required: ['x', 'y', 'choice'],
    },
    execute: (input = {}) => {
      const x = int(input, 'x'), y = int(input, 'y'), z = level(input);
      const view = game.inspect({ x, y, z });
      if (!view || view.kind < beltCount) throw new ToolError('There is no machine at ' + x + ', ' + y + ' on level ' + z + '.');
      const unit = pack.units[view.kind - beltCount];
      const wanted = normal(input.choice);
      const reset = ['automatic', 'auto', 'everything', 'all'].includes(wanted);
      let value = -1;
      if (unit.route === 'filter') value = pack.resources.findIndex((r) => normal(r.name) === wanted || normal(r.id) === wanted);
      else if (unit.rules.length > 1) value = unit.rules.findIndex((rule) => normal(D.recipeName(pack, rule)) === wanted || normal(rule.name) === wanted);
      if (!reset && value < 0) {
        const options = unit.route === 'filter' ? pack.resources.map((r) => r.name) : unit.rules.map((rule) => D.recipeName(pack, rule));
        throw new ToolError('No choice "' + input.choice + '" for a ' + unit.name + '. Options: ' + options.join(', ') + '.');
      }
      of.of_configure(x, y, z, reset ? -1 : value);
      const refused = commit();
      return refused === SENT ? { sentToHost: true } : refused ? { refused } : { done: true };
    },
  });

  tool({
    name: 'openfactor_research',
    title: 'Start research',
    description: 'Pays for and starts a research from the shared stock. One research runs at a time.',
    inputSchema: { type: 'object', properties: { research: { type: 'string', description: 'Research name, such as "Routing".' } }, required: ['research'] },
    execute: (input = {}) => {
      const wanted = normal(input.research);
      const index = pack.techs.findIndex((t) => normal(t.name) === wanted || normal(t.id) === wanted);
      if (index < 0) throw new ToolError('No research called "' + input.research + '".');
      of.of_research(index);
      const refused = commit();
      if (refused === SENT) return { sentToHost: true };
      if (!refused) game.notify('An assistant started researching ' + pack.techs[index].name + '.');
      return refused ? { refused } : { started: pack.techs[index].name, seconds: pack.techs[index].time / D.TICKS_PER_SECOND };
    },
  });

  tool({
    name: 'openfactor_wait',
    title: 'Let time pass',
    description: 'Runs the factory forward quickly by up to 120 seconds of game time, then reports what the stock gained or lost.',
    inputSchema: { type: 'object', properties: { seconds: { type: 'integer', minimum: 1, maximum: MAX_WAIT_SECONDS } }, required: ['seconds'] },
    execute: (input = {}) => {
      const seconds = int(input, 'seconds');
      if (seconds < 1 || seconds > MAX_WAIT_SECONDS) throw new ToolError('seconds must be 1 to ' + MAX_WAIT_SECONDS);
      if (remote()) throw new ToolError('In a shared game only the host’s browser moves time on.');
      const before = stock();
      of.of_step(seconds * D.TICKS_PER_SECOND);
      game.refresh();
      const after = stock(), change = {};
      for (const name of new Set([...Object.keys(before), ...Object.keys(after)])) {
        const delta = (after[name] || 0) - (before[name] || 0);
        if (delta) change[name] = delta;
      }
      return { seconds: tickSeconds(), stockChange: change, stock: after };
    },
  });

  return tools;
}

/** Registers the tools with the browser, if it has a model context. Returns how many. */
export async function registerTools(game) {
  const found = modelContext();
  if (!found) return 0;
  const { context, legacy } = found;
  const controller = new AbortController();
  let count = 0;
  for (const definition of makeTools(game)) {
    const execute = async (input) => {
      let value;
      try {
        value = await definition.execute(input || {});
      } catch (error) {
        if (!(error instanceof ToolError)) throw error;
        value = { error: error.message };
      }
      // The older navigator form expects an MCP tool result.
      return legacy ? { content: [{ type: 'text', text: JSON.stringify(value) }], isError: !!value.error } : value;
    };
    try {
      await context.registerTool({ ...definition, execute }, { signal: controller.signal });
      count++;
    } catch (error) {
      console.warn('WebMCP tool not registered:', definition.name, error);
    }
  }
  return count;
}
