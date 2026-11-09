// A 3D view of the same snapshot the 2D view draws. It only reads; the simulation
// doesn't know it exists. Loaded the first time the player switches to it.

import * as THREE from 'three';
import { GLTFLoader } from './vendor/jsm/loaders/GLTFLoader.js';

const MODEL_ROOT = 'assets/models/';
const PITCH = THREE.MathUtils.degToRad(38);
const CAMERA_DISTANCE = 80;
const TILE_FIT = 0.94;
const ROTATE_EASE = 0.18;
const BELT_TOP = 0.34;
const ITEM_HEIGHT = 0.16;
const HAZE_BUCKETS = [0.15, 0.3, 0.45, 0.62];
const SAND = 0xe3a76c, GRID = 0xb9824e;
const STATUS_COLORS = { 2: 0xe05252, 3: 0xb9bdc2, 4: 0xf2b63c };
// Edges are numbered from north, clockwise. The kit's corner piece opens on edge 2
// and the one after it, and its straight piece runs north to south.
const CORNER_BASE_EDGE = 2;
const STRAIGHT_BELT_TURN = Math.PI / 2;

const quarterTurn = (steps) => -steps * Math.PI / 2;
const facingYaw = (dir) => quarterTurn(dir - 1);

/** An instanced mesh that grows when it needs to. */
function makeBatch(parent, geometry, material, { shadow = false, receive = false, colored = false } = {}) {
  let capacity = 0, mesh = null;
  return {
    /** Makes room for `count` instances and returns the mesh. */
    resize(count) {
      if (count > capacity) {
        if (mesh) { parent.remove(mesh); mesh.dispose(); }
        capacity = Math.max(64, count * 2);
        mesh = new THREE.InstancedMesh(geometry, material, capacity);
        mesh.castShadow = shadow;
        mesh.receiveShadow = receive;
        mesh.frustumCulled = false;
        if (colored) mesh.setColorAt(0, new THREE.Color(0xffffff));
        parent.add(mesh);
      }
      if (mesh) mesh.count = count;
      return mesh;
    },
    commit() {
      if (!mesh) return;
      mesh.instanceMatrix.needsUpdate = true;
      if (mesh.instanceColor) mesh.instanceColor.needsUpdate = true;
    },
  };
}

function averageColor(source) {
  const canvas = document.createElement('canvas');
  canvas.width = canvas.height = 16;
  const context = canvas.getContext('2d', { willReadFrequently: true });
  context.drawImage(source.image, source.sx, source.sy, 16, 16, 0, 0, 16, 16);
  const data = context.getImageData(0, 0, 16, 16).data;
  let red = 0, green = 0, blue = 0, count = 0;
  for (let i = 0; i < data.length; i += 4) {
    // Skip transparency and the dark outline every sprite has.
    if (data[i + 3] < 200 || data[i] + data[i + 1] + data[i + 2] < 150) continue;
    red += data[i]; green += data[i + 1]; blue += data[i + 2]; count++;
  }
  if (!count) return new THREE.Color(0x26262e);
  return new THREE.Color(`rgb(${Math.round(red / count)},${Math.round(green / count)},${Math.round(blue / count)})`);
}

export async function createView3d({ pack, canvas, spriteSource, isBelt, DIRS, PORT_TURNS, beltCount }) {
  const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
  renderer.shadowMap.enabled = true;
  renderer.shadowMap.type = THREE.PCFSoftShadowMap;

  const scene = new THREE.Scene();
  scene.background = new THREE.Color(SAND);
  const camera = new THREE.OrthographicCamera(-1, 1, 1, -1, 0.1, 400);

  scene.add(new THREE.HemisphereLight(0xffffff, 0xb08a68, 1.5));
  const sun = new THREE.DirectionalLight(0xfff2dd, 2.2);
  sun.castShadow = true;
  sun.shadow.mapSize.set(2048, 2048);
  sun.shadow.bias = -0.0006;
  scene.add(sun, sun.target);

  const floor = new THREE.Mesh(new THREE.PlaneGeometry(4000, 4000), new THREE.MeshLambertMaterial({ color: SAND }));
  floor.rotation.x = -Math.PI / 2;
  floor.receiveShadow = true;
  scene.add(floor);
  const grid = new THREE.GridHelper(240, 240, GRID, GRID);
  grid.material.transparent = true;
  grid.material.opacity = 0.35;
  grid.position.y = 0.02;
  scene.add(grid);

  // --- Models

  const names = new Set();
  for (const belt of pack.belts) { if (belt.model) names.add(belt.model); if (belt.corner) names.add(belt.corner); }
  for (const unit of pack.units) if (unit.model) names.add(unit.model);
  const loader = new GLTFLoader();
  const models = new Map();
  await Promise.all([...names].map(async (name) => {
    const gltf = await loader.loadAsync(MODEL_ROOT + name + '.glb');
    gltf.scene.updateMatrixWorld(true);
    const parts = [];
    gltf.scene.traverse((node) => {
      if (node.isMesh) parts.push({ geometry: node.geometry, material: node.material, matrix: node.matrixWorld.clone() });
    });
    models.set(name, { name, parts, bounds: new THREE.Box3().setFromObject(gltf.scene) });
  }));

  // Building models are centred, stood on the ground and fitted inside one tile.
  function fitOf(model, isTile) {
    if (isTile) return { matrix: new THREE.Matrix4(), height: model.bounds.max.y };
    const size = model.bounds.getSize(new THREE.Vector3());
    const center = model.bounds.getCenter(new THREE.Vector3());
    const scale = Math.min(1, TILE_FIT / Math.max(size.x, size.z));
    const matrix = new THREE.Matrix4().makeScale(scale, scale, scale)
      .multiply(new THREE.Matrix4().makeTranslation(-center.x, -model.bounds.min.y, -center.z));
    return { matrix, height: size.y * scale };
  }
  const fits = new Map();
  for (const model of models.values()) fits.set(model.name, fitOf(model, model.name.startsWith('conveyor')));

  const world = new THREE.Group();
  scene.add(world);
  const modelBatches = new Map();
  for (const model of models.values()) {
    modelBatches.set(model.name, model.parts.map((part) => (
      { part, batch: makeBatch(world, part.geometry, part.material, { shadow: true, receive: true }) })));
  }

  // --- Simple shapes

  const flatTile = new THREE.PlaneGeometry(1, 1);
  flatTile.rotateX(-Math.PI / 2);
  const tiles = makeBatch(world, flatTile, new THREE.MeshLambertMaterial({ color: 0xffffff }), { receive: true, colored: true });
  const hazeBatches = HAZE_BUCKETS.map((opacity) => makeBatch(world, flatTile,
    new THREE.MeshBasicMaterial({ color: 0xffffff, transparent: true, opacity, depthWrite: false }), { colored: true }));

  const itemColors = pack.resources.map((resource) => averageColor(spriteSource(resource.sprite, resource.tint)));
  const itemGeometry = new THREE.BoxGeometry(0.26, ITEM_HEIGHT, 0.26);
  const itemBatch = makeBatch(world, itemGeometry, new THREE.MeshLambertMaterial({ color: 0xffffff }), { shadow: true, colored: true });

  // A flat arrowhead pointing along +X.
  const arrowShape = new THREE.Shape([new THREE.Vector2(-0.12, -0.14), new THREE.Vector2(0.14, 0), new THREE.Vector2(-0.12, 0.14)]);
  const arrowGeometry = new THREE.ShapeGeometry(arrowShape);
  arrowGeometry.rotateX(-Math.PI / 2);
  const arrowBatch = makeBatch(world, arrowGeometry, new THREE.MeshBasicMaterial({ color: 0xfff4d6, side: THREE.DoubleSide }));

  const markBatch = makeBatch(world, new THREE.BoxGeometry(0.2, 0.2, 0.2), new THREE.MeshBasicMaterial({ color: 0xffffff }), { colored: true });
  const pipeBatch = makeBatch(world, new THREE.BoxGeometry(1, 0.2, 0.2), new THREE.MeshStandardMaterial({ color: 0x8ea0b8, metalness: 0.2, roughness: 0.6 }), { shadow: true });

  const wires = new THREE.LineSegments(new THREE.BufferGeometry(), new THREE.LineBasicMaterial({ color: 0xe8c04a }));
  wires.frustumCulled = false;
  world.add(wires);

  const scratch = new THREE.Matrix4(), place = new THREE.Matrix4(), turn = new THREE.Matrix4(), color = new THREE.Color();

  // --- Camera

  let yaw = Math.PI / 4, yawTarget = yaw;
  let viewWidth = 1, viewHeight = 1;

  function syncCamera(cam) {
    const offset = new THREE.Vector3(
      Math.sin(yaw) * Math.cos(PITCH), Math.sin(PITCH), Math.cos(yaw) * Math.cos(PITCH),
    ).multiplyScalar(CAMERA_DISTANCE);
    const target = new THREE.Vector3(cam.x, 0, cam.y);
    camera.position.copy(target).add(offset);
    camera.left = -viewWidth / 2 / cam.cell;
    camera.right = viewWidth / 2 / cam.cell;
    camera.top = viewHeight / 2 / cam.cell;
    camera.bottom = -viewHeight / 2 / cam.cell;
    camera.lookAt(target);
    camera.updateProjectionMatrix();
    camera.updateMatrixWorld();
    return target;
  }

  const ray = new THREE.Raycaster();
  const groundPlane = new THREE.Plane(new THREE.Vector3(0, 1, 0), 0);
  let lastCam = { x: 0, y: 0, cell: 48 };
  function groundAt(clientX, clientY, cam = lastCam) {
    syncCamera(cam);
    ray.setFromCamera(new THREE.Vector2(clientX / viewWidth * 2 - 1, 1 - clientY / viewHeight * 2), camera);
    const hit = ray.ray.intersectPlane(groundPlane, new THREE.Vector3());
    return hit ? { x: hit.x, y: hit.z } : { x: cam.x, y: cam.y };
  }

  // --- Layout, rebuilt only when the buildings change

  const buildingAt = (buildings, x, y) => buildings.get(x + ',' + y);
  const unitOf = (b) => pack.units[b.kind - beltCount];

  function feedsFrom(buildings, b, side) {
    const [dx, dy] = DIRS[side];
    const neighbour = buildingAt(buildings, b.x + dx, b.y + dy);
    if (!neighbour) return false;
    const toward = (side + 2) % 4;
    if (isBelt(neighbour.kind)) return neighbour.dir === toward;
    return unitOf(neighbour).ports.some((port) => (neighbour.dir + PORT_TURNS[port] + 4) % 4 === toward);
  }

  /** The model and quarter turns that draw a building. */
  function pieceOf(buildings, b) {
    if (!isBelt(b.kind)) return { model: unitOf(b).model, yaw: facingYaw(b.dir) };
    const belt = pack.belts[b.kind];
    const behind = (b.dir + 2) % 4;
    const sides = [(b.dir + 1) % 4, (b.dir + 3) % 4].filter((side) => feedsFrom(buildings, b, side));
    if (belt.corner && sides.length === 1 && !feedsFrom(buildings, b, behind)) {
      // The piece joins two neighbouring edges; find the turn that lines them up.
      const side = sides[0];
      const first = (side + 1) % 4 === b.dir ? side : b.dir;
      return { model: belt.corner, yaw: quarterTurn(first - CORNER_BASE_EDGE) };
    }
    return { model: belt.model, yaw: facingYaw(b.dir) + STRAIGHT_BELT_TURN };
  }

  function setInstance(mesh, index, x, y, z, angle, inner) {
    place.makeTranslation(x, y, z);
    turn.makeRotationY(angle);
    scratch.copy(place).multiply(turn).multiply(inner);
    mesh.setMatrixAt(index, scratch);
  }

  let unitHeights = new Map();
  function rebuildBuildings(buildings) {
    const lists = new Map([...models.keys()].map((name) => [name, []]));
    const arrows = [];
    unitHeights = new Map();
    for (const b of buildings.values()) {
      const piece = pieceOf(buildings, b);
      if (piece.model) lists.get(piece.model).push({ b, yaw: piece.yaw });
      if (isBelt(b.kind)) {
        arrows.push({ x: b.x + 0.5, z: b.y + 0.5, y: BELT_TOP + 0.09, dir: b.dir });
      } else {
        const unit = unitOf(b);
        unitHeights.set(b.x + ',' + b.y, piece.model ? fits.get(piece.model).height : 0.5);
        for (const port of unit.ports) {
          const dir = (b.dir + PORT_TURNS[port] + 4) % 4, [dx, dz] = DIRS[dir];
          arrows.push({ x: b.x + 0.5 + dx * 0.42, z: b.y + 0.5 + dz * 0.42, y: 0.12, dir });
        }
      }
    }

    for (const [name, list] of lists) {
      const fit = fits.get(name).matrix;
      for (const { part, batch } of modelBatches.get(name)) {
        const mesh = batch.resize(list.length);
        const inner = new THREE.Matrix4().multiplyMatrices(fit, part.matrix);
        list.forEach(({ b, yaw: angle }, index) => setInstance(mesh, index, b.x + 0.5, 0, b.y + 0.5, angle, inner));
        batch.commit();
      }
    }

    const arrowMesh = arrowBatch.resize(arrows.length);
    const identity = new THREE.Matrix4();
    arrows.forEach((a, index) => setInstance(arrowMesh, index, a.x, a.y, a.z, facingYaw(a.dir), identity));
    arrowBatch.commit();

    rebuildLinks(buildings);
  }

  function rebuildLinks(buildings) {
    const pipes = [], segments = [];
    for (const b of buildings.values()) {
      if (isBelt(b.kind)) continue;
      for (const [resource, reach] of unitOf(b).links) {
        if (reach === 0) continue;
        for (let dx = -reach; dx <= reach; dx++) {
          const spread = reach - Math.abs(dx);
          for (let dy = -spread; dy <= spread; dy++) {
            if (!dx && !dy) continue;
            const other = buildingAt(buildings, b.x + dx, b.y + dy);
            if (!other || isBelt(other.kind) || !unitOf(other).linked.has(resource)) continue;
            // A link seen from both ends is only drawn once.
            const otherReach = unitOf(other).links.find(([linked]) => linked === resource)[1];
            if (otherReach >= Math.abs(dx) + Math.abs(dy) && (dx < 0 || (dx === 0 && dy < 0))) continue;
            if (reach === 1) {
              pipes.push({ x: b.x + 0.5 + dx / 2, z: b.y + 0.5 + dy / 2, along: dx !== 0 });
            } else {
              const fromHeight = Math.max(0.3, unitHeights.get(b.x + ',' + b.y) - 0.1);
              const toHeight = Math.max(0.3, unitHeights.get(other.x + ',' + other.y) - 0.1);
              segments.push(b.x + 0.5, fromHeight, b.y + 0.5, other.x + 0.5, toHeight, other.y + 0.5);
            }
          }
        }
      }
    }
    const pipeMesh = pipeBatch.resize(pipes.length);
    const identity = new THREE.Matrix4();
    pipes.forEach((pipe, index) => setInstance(pipeMesh, index, pipe.x, 0.3, pipe.z, pipe.along ? 0 : Math.PI / 2, identity));
    pipeBatch.commit();
    wires.geometry.dispose();
    wires.geometry = new THREE.BufferGeometry();
    wires.geometry.setAttribute('position', new THREE.Float32BufferAttribute(segments, 3));
  }

  function rebuildGround(ground) {
    const mesh = tiles.resize(ground.size);
    let index = 0;
    for (const [key, layer] of ground) {
      const [x, y] = key.split(',').map(Number);
      place.makeTranslation(x + 0.5, 0.03, y + 0.5);
      mesh.setMatrixAt(index, place);
      mesh.setColorAt(index, color.set(pack.layers[layer].tint ?? 0x777777));
      index++;
    }
    tiles.commit();
  }

  function updateHaze(haze) {
    const buckets = HAZE_BUCKETS.map(() => []);
    for (const h of haze) {
      if (h.strength < 12) continue;
      buckets[Math.min(HAZE_BUCKETS.length - 1, Math.floor(h.strength / 256 * HAZE_BUCKETS.length))].push(h);
    }
    buckets.forEach((list, bucket) => {
      const mesh = hazeBatches[bucket].resize(list.length);
      list.forEach((h, index) => {
        place.makeTranslation(h.x + 0.5, 1.7 + bucket * 0.01, h.y + 0.5);
        mesh.setMatrixAt(index, place);
        mesh.setColorAt(index, color.set(pack.layers[h.layer].haze));
      });
      hazeBatches[bucket].commit();
    });
  }

  function updateItems(items) {
    const mesh = itemBatch.resize(items.length);
    items.forEach((it, index) => {
      place.makeTranslation(it.x, BELT_TOP + ITEM_HEIGHT / 2, it.y);
      mesh.setMatrixAt(index, place);
      mesh.setColorAt(index, itemColors[it.item]);
    });
    itemBatch.commit();
  }

  function updateMarks(buildings) {
    const marked = [];
    for (const b of buildings.values()) if (STATUS_COLORS[b.status]) marked.push(b);
    const mesh = markBatch.resize(marked.length);
    marked.forEach((b, index) => {
      place.makeTranslation(b.x + 0.5, (unitHeights.get(b.x + ',' + b.y) ?? 1) + 0.25, b.y + 0.5);
      mesh.setMatrixAt(index, place);
      mesh.setColorAt(index, color.set(STATUS_COLORS[b.status]));
    });
    markBatch.commit();
  }

  // --- Pointer feedback

  const highlightMaterial = (hex, opacity) => new THREE.MeshBasicMaterial({ color: hex, transparent: true, opacity, depthWrite: false });
  const hoverTile = new THREE.Mesh(flatTile, highlightMaterial(0xffffff, 0.3));
  const eraseMaterial = highlightMaterial(0xe05252, 0.5), hoverMaterial = hoverTile.material;
  const selectTile = new THREE.Mesh(flatTile, highlightMaterial(0xf2b63c, 0.55));
  hoverTile.position.y = selectTile.position.y = 0.06;
  hoverTile.renderOrder = selectTile.renderOrder = 2;
  world.add(hoverTile, selectTile);

  const rangeGroup = new THREE.Group();
  world.add(rangeGroup);
  const rangeMaterial = highlightMaterial(0xf2b63c, 0.22);

  const ghost = new THREE.Group();
  world.add(ghost);
  let ghostTool = null;

  function ghostModelName(tool) {
    if (tool.belt) return pack.belts[tool.kind].model;
    return tool.unit ? tool.unit.model : null;
  }

  function buildGhost(tool) {
    ghost.clear();
    rangeGroup.clear();
    ghostTool = tool;
    const name = ghostModelName(tool);
    if (name) {
      const fit = fits.get(name).matrix;
      for (const part of models.get(name).parts) {
        const material = part.material.clone();
        material.transparent = true;
        material.opacity = 0.55;
        const mesh = new THREE.Mesh(part.geometry, material);
        mesh.matrixAutoUpdate = false;
        mesh.matrix.multiplyMatrices(fit, part.matrix);
        ghost.add(mesh);
      }
    }
    if (tool.unit) {
      const reach = tool.unit.links.reduce((most, [, r]) => Math.max(most, r), 0);
      if (reach > 1) {
        for (let dx = -reach; dx <= reach; dx++) {
          const spread = reach - Math.abs(dx);
          for (let dy = -spread; dy <= spread; dy++) addRangeTile(dx, dy);
        }
      }
    }
  }

  function addRangeTile(dx, dy) {
    const tile = new THREE.Mesh(flatTile, rangeMaterial);
    tile.position.set(dx, 0.05, dy);
    rangeGroup.add(tile);
  }

  function updatePointer({ hover, tool, rotation, buildings, selected }) {
    selectTile.visible = !!selected;
    if (selected) selectTile.position.set(selected.x + 0.5, 0.06, selected.y + 0.5);

    const active = hover && tool.id !== 'move';
    hoverTile.visible = !!active;
    ghost.visible = rangeGroup.visible = false;
    if (!active) return;
    hoverTile.position.set(hover.x + 0.5, 0.06, hover.y + 0.5);
    hoverTile.material = tool.id === 'erase' ? eraseMaterial : hoverMaterial;
    if (tool.id === 'erase' || tool.id === 'inspect' || buildings.has(hover.x + ',' + hover.y)) return;

    if (ghostTool !== tool) buildGhost(tool);
    ghost.visible = true;
    ghost.position.set(hover.x + 0.5, 0, hover.y + 0.5);
    ghost.rotation.y = facingYaw(rotation) + (tool.belt ? STRAIGHT_BELT_TURN : 0);
    rangeGroup.visible = true;
    rangeGroup.position.copy(ghost.position);
    if (tool.unit && tool.unit.route === 'tunnel') {
      rangeGroup.clear();
      const [dx, dy] = DIRS[rotation];
      for (let step = 1; step <= tool.unit.range; step++) addRangeTile(dx * step, dy * step);
    }
  }

  function hashBuildings(buildings) {
    let hash = buildings.size;
    for (const b of buildings.values()) {
      hash = Math.imul(hash, 31) + b.x | 0;
      hash = Math.imul(hash, 31) + b.y | 0;
      hash = Math.imul(hash, 31) + b.dir * 7 + b.kind | 0;
    }
    return hash;
  }

  let builtHash = null, groundCount = -1;

  return {
    groundAt,
    cellAt(clientX, clientY) {
      const point = groundAt(clientX, clientY);
      return { x: Math.floor(point.x), y: Math.floor(point.y) };
    },
    resize(width, height) {
      viewWidth = width;
      viewHeight = height;
      renderer.setSize(width, height, false);
    },
    rotateView(steps) {
      yawTarget += steps * Math.PI / 2;
    },
    /** Turns a screen-relative step into a world step for the current camera. */
    panVector(stepX, stepY) {
      const right = [Math.cos(yaw), -Math.sin(yaw)], ahead = [-Math.sin(yaw), -Math.cos(yaw)];
      return [right[0] * stepX - ahead[0] * stepY, right[1] * stepX - ahead[1] * stepY];
    },
    render(state) {
      yaw += (yawTarget - yaw) * ROTATE_EASE;
      lastCam = state.cam;
      const target = syncCamera(state.cam);

      const reach = Math.max(viewWidth, viewHeight) / state.cam.cell;
      sun.target.position.copy(target);
      sun.position.copy(target).add(new THREE.Vector3(-30, 60, 25));
      sun.shadow.camera.left = sun.shadow.camera.bottom = -reach;
      sun.shadow.camera.right = sun.shadow.camera.top = reach;
      sun.shadow.camera.updateProjectionMatrix();
      grid.position.x = Math.round(state.cam.x);
      grid.position.z = Math.round(state.cam.y);

      const hash = hashBuildings(state.buildings);
      if (hash !== builtHash) { builtHash = hash; rebuildBuildings(state.buildings); }
      if (state.ground.size !== groundCount) { groundCount = state.ground.size; rebuildGround(state.ground); }
      updateHaze(state.haze);
      updateItems(state.items);
      updateMarks(state.buildings);
      updatePointer(state);
      renderer.render(scene, camera);
    },
  };
}
