// Plain-language descriptions of a rule pack's content, worked out from the
// same JSON the game uses. Shared by the play page and the docs page, so the
// two always agree with the pack.
//
// Amounts in the pack JSON are [scope, id, amount] (plus a radius for map
// amounts). Scopes: l = the building's own slots, g = the shared stock,
// n = the building's network, m = a map layer.

export const TICKS_PER_SECOND = 60;
// Belt items sit at least this far apart, in 1/256 of a tile.
const ITEM_SPACING = 64;

const PORTS = ['front', 'left side', 'right side'];
const TESTS = { atLeast: 'at least', atMost: 'at most', greater: 'above', less: 'below', is: 'exactly' };

export function seconds(ticks) {
  const s = ticks / TICKS_PER_SECOND;
  return (Number.isInteger(s) ? String(s) : s.toFixed(s < 1 ? 2 : 1).replace(/0$/, '')) + ' s';
}

/** "2 × Iron plate". */
export function amount(pack, id, count) {
  return count + ' × ' + pack.resources[id].name;
}

export function list(parts, last = 'and') {
  if (parts.length < 2) return parts.join('');
  return parts.slice(0, -1).join(', ') + ' ' + last + ' ' + parts[parts.length - 1];
}

/** What a cost reads as, such as "8 × Iron plate, 2 × Gear". */
export function costText(pack, cost) {
  if (!cost.length) return 'Free';
  return cost.map(([id, count]) => amount(pack, id, count)).join(', ');
}

/** What is missing from `stock(id)` to pay `cost`, such as "3 × Iron plate". */
export function shortfall(pack, cost, stock) {
  const missing = cost.filter(([id, count]) => stock(id) < count).map(([id, count]) => amount(pack, id, count - stock(id)));
  return missing.join(', ');
}

/** Which belts, units and recipes each research unlocks, as {kind, name, unit?}. */
export function unlocksByTech(pack) {
  const unlocks = pack.techs.map(() => []);
  pack.belts.forEach((belt) => { if (belt.lock !== null) unlocks[belt.lock].push({ kind: 'belt', name: belt.name, belt }); });
  pack.units.forEach((unit) => {
    if (unit.lock !== null) unlocks[unit.lock].push({ kind: 'unit', name: unit.name, unit });
    unit.rules.forEach((rule) => {
      if (rule.lock !== null && rule.lock !== unit.lock) {
        unlocks[rule.lock].push({ kind: 'recipe', name: recipeName(pack, rule) + ' in the ' + unit.name, unit, rule });
      }
    });
  });
  return unlocks;
}

const isHaze = (pack, layer) => pack.layers[layer].haze !== null;

/** A short name for a recipe: what it makes. */
export function recipeName(pack, rule) {
  const made = rule.out.filter(([scope]) => scope !== 'm').map(([, id]) => pack.resources[id].name);
  return made.length ? list(made) : rule.name;
}

/**
 * One recipe as a sentence plus notes:
 * { line: "Every 2 s: 2 × Iron ore + 1 × Coal → 2 × Iron plate", notes: [...] }
 */
export function recipeText(pack, rule, unit = null) {
  const notes = [];
  const side = (where) => (entries) => entries.filter(([scope]) => scope === where);
  const named = (entries) => entries.map(([, id, count]) => amount(pack, id, count));
  const takes = named(side('l')(rule.in));
  const gives = named(side('l')(rule.out));
  const inputs = [...takes];
  const outputs = [...gives];
  const networkIn = named(side('n')(rule.in)), networkOut = named(side('n')(rule.out));
  const stockIn = named(side('g')(rule.in)), stockOut = named(side('g')(rule.out));
  if (networkIn.length) inputs.push(list(networkIn) + ' from its network');
  if (stockIn.length) inputs.push(list(stockIn) + ' from the stock');
  if (networkOut.length) outputs.push(list(networkOut) + ' to its network');
  if (stockOut.length) outputs.push(list(stockOut) + ' to the stock');

  for (const [, layer, count] of side('m')(rule.in)) {
    notes.push('Works on ' + pack.layers[layer].name + ', using up ' + count + ' of it each time.');
  }
  for (const [, layer, count, radius] of side('m')(rule.out)) {
    const name = pack.layers[layer].name;
    notes.push(radius
      ? 'Adds ' + count + ' ' + name + ' to every tile within ' + radius + ' step' + (radius === 1 ? '' : 's') + '.'
      : 'Adds ' + count + ' ' + name + ' to its tile.');
  }
  for (const [layer, test, value] of rule.when || []) {
    const name = pack.layers[layer].name;
    const onIt = !isHaze(pack, layer) && ((test === 'atLeast' && value === 1) || (test === 'greater' && value === 0));
    // Placement already says where it can be built.
    if (onIt && unit && unit.placeOn && unit.placeOn.includes(layer)) continue;
    notes.push(onIt ? 'Only on ' + name + '.' : 'Only while ' + name + ' is ' + TESTS[test] + ' ' + value + '.');
  }
  for (const [id, port] of rule.sides || []) {
    notes.push(pack.resources[id].name + ' leaves by its ' + PORTS[port] + '.');
  }
  for (const [metric, count] of rule.logs || []) {
    notes.push('Counts ' + count + ' toward ' + pack.metrics[metric].name + '.');
  }
  if (rule.lock !== null) notes.push('Needs the research ' + pack.techs[rule.lock].name + '.');

  let line = 'Every ' + seconds(rule.rate) + ': ';
  if (inputs.length && outputs.length) line += list(inputs) + ' → ' + list(outputs);
  else if (outputs.length) line += (gives.length ? 'makes ' : 'gives ') + list(outputs);
  else if (inputs.length) line += 'uses ' + list(inputs);
  else line += 'runs';
  return { line, notes };
}

/** How a belt tier moves items, in items per second. */
export function beltFacts(pack, belt) {
  const perSecond = (TICKS_PER_SECOND * belt.speed) / ITEM_SPACING;
  const facts = ['Carries up to ' + (Number.isInteger(perSecond) ? perSecond : perSecond.toFixed(1)) + ' items a second.'];
  if (belt.lock !== null) facts.push('Unlocked by the research ' + pack.techs[belt.lock].name + '.');
  return facts;
}

/** Plain facts about a unit beyond its recipes: where it goes, what it joins. */
export function unitFacts(pack, unit) {
  const facts = [];
  if (unit.placeOn && unit.placeOn.length) {
    facts.push('Build it on ' + list(unit.placeOn.map((layer) => pack.layers[layer].name), 'or') +
      (pack.levels > 1 ? ', on the ground level.' : '.'));
  }
  if (unit.route === 'split') facts.push('Sends what it receives out of its front, left and right in turn.');
  if (unit.route === 'filter') facts.push('The resource chosen in its inspector goes straight on; the rest leave by the sides.');
  if (unit.route === 'tunnel') facts.push('Passes items to the next ' + unit.name + ' facing the same way within ' + unit.range + ' tiles.');
  if (unit.route === 'lift') {
    facts.push('Moves items ' + (unit.down ? 'down' : 'up') + ' to the next ' + unit.name + ' in the same column, up to ' +
      unit.range + ' level' + (unit.range === 1 ? '' : 's') + ' away, with nothing in between.');
  }
  if (unit.delivers) facts.push('Everything it receives joins the shared stock.');
  else if (unit.storesAny) facts.push('Stores any item, without limit.');
  for (const [resource, reach] of unit.links) {
    const name = pack.resources[resource].name;
    if (reach > 0) {
      facts.push('Connects ' + name + ' users and producers within ' + reach + ' step' + (reach === 1 ? '' : 's') + '.');
    } else {
      const carriers = pack.units.filter((other) => other.links.some(([r, n]) => r === resource && n > 0)).map((other) => other.name);
      facts.push('Shares ' + name + ' over a network' + (carriers.length ? ': connect it with a ' + list(carriers, 'or') + '.' : '.'));
    }
  }
  if (unit.lock !== null) facts.push('Unlocked by the research ' + pack.techs[unit.lock].name + '.');
  return facts;
}

export const REFUND_NOTE = 'Erasing it gives the full cost back; whatever it held is lost.';
