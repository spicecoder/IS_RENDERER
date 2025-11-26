# CPUX Architecture (diagram) and Appendix

Below is a publication-quality SVG architecture diagram followed by a concise appendix describing the database tables used by the reference CPUX implementation and how each table contributes to runtime behavior.

<!-- SVG architecture diagram (publication-quality) -->

<div style="text-align:center">

```html
<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="700" viewBox="0 0 1100 700">
  <defs>
    <filter id="shadow" x="-20%" y="-20%" width="140%" height="140%">
      <feDropShadow dx="0" dy="5" stdDeviation="6" flood-color="#000" flood-opacity="0.12"/>
    </filter>
    <style>
      .box { fill:#ffffff; stroke:#2b2b2b; stroke-width:1.2px; rx:10; }
      .title { font: bold 16px sans-serif; fill:#111; }
      .label { font: 13px sans-serif; fill:#222; }
      .muted { font: 12px sans-serif; fill:#666; }
      .arrow { stroke:#2b2b2b; stroke-width:2; marker-end:url(#arrowhead); fill:none }
      .small { font: 11px sans-serif; fill:#333 }
    </style>
    <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="10" refY="3.5" orient="auto">
      <polygon points="0 0, 10 3.5, 0 7" fill="#2b2b2b" />
    </marker>
  </defs>

  <!-- Progressor + Field central region -->
  <g>
    <rect x="320" y="40" width="460" height="160" class="box" filter="url(#shadow)" />
    <text x="550" y="68" text-anchor="middle" class="title">CPUX Progressor / Field</text>
    <text x="550" y="94" text-anchor="middle" class="muted">(visits ex_instances in order, accumulates Field)</text>

    <!-- Field area inside -->
    <rect x="355" y="110" width="170" height="60" rx="8" fill="#f7fbff" stroke="#aac9ff" />
    <text x="440" y="140" text-anchor="middle" class="small">Field: Pulses + Active Intentions</text>

    <!-- Progressor box -->
    <rect x="555" y="110" width="160" height="60" rx="8" fill="#fff8eb" stroke="#ffd59a" />
    <text x="635" y="140" text-anchor="middle" class="small">Progressor Loop & Golden‑pass</text>
  </g>

  <!-- ex_instance strip (top) -->
  <g>
    <text x="80" y="28" class="title">Design-time CPUX Thread (ex_instances)</text>

    <!-- Boxes for ex_instances -->
    <rect x="30" y="50" width="170" height="60" class="box" />
    <text x="115" y="80" text-anchor="middle" class="label">O (Object)\nnamespace: cpux:...:p0</text>

    <rect x="210" y="50" width="170" height="60" class="box" />
    <text x="295" y="80" text-anchor="middle" class="label">I (Intention)</text>

    <rect x="390" y="50" width="170" height="60" class="box" />
    <text x="475" y="80" text-anchor="middle" class="label">DN (Design Node)\nnamespace: cpux:...:p1</text>

    <rect x="570" y="50" width="170" height="60" class="box" />
    <text x="655" y="80" text-anchor="middle" class="label">I (Intention)</text>

    <rect x="750" y="50" width="170" height="60" class="box" />
    <text x="835" y="80" text-anchor="middle" class="label">O (Object)\nnamespace: cpux:...:p2</text>

    <!-- arrows to progressor -->
    <path d="M 190 110 Q 300 140 360 120" class="arrow"/>
    <path d="M 360 110 Q 470 130 540 120" class="arrow"/>
    <path d="M 540 110 Q 650 140 720 120" class="arrow"/>
  </g>

  <!-- Workers & DN webhooks left -->
  <g>
    <rect x="30" y="240" width="240" height="120" class="box" />
    <text x="150" y="268" text-anchor="middle" class="title">Workers / Executors</text>
    <text x="150" y="290" text-anchor="middle" class="muted">(claim intentions, call DN webhooks or invoke Objects)</text>

    <rect x="50" y="310" width="80" height="36" rx="6" class="box"/>
    <text x="90" y="333" text-anchor="middle" class="small">worker w1</text>
    <rect x="140" y="310" width="80" height="36" rx="6" class="box"/>
    <text x="180" y="333" text-anchor="middle" class="small">worker w2</text>

    <!-- arrows to progressor -->
    <path d="M 125 260 L 350 200" class="arrow"/>
  </g>

  <!-- DN webhooks -->
  <g>
    <rect x="30" y="380" width="240" height="220" class="box" />
    <text x="150" y="408" text-anchor="middle" class="title">DN Webhook Processes</text>
    <text x="150" y="428" text-anchor="middle" class="muted">(HTTP wake, execute DN logic, emit Intentions/Pulses)</text>

    <rect x="50" y="460" width="160" height="36" rx="6" class="box"/>
    <text x="130" y="485" text-anchor="middle" class="small">dn_get_name (9001)</text>

    <rect x="50" y="510" width="160" height="36" rx="6" class="box"/>
    <text x="130" y="535" text-anchor="middle" class="small">dn_check_age (9002)</text>

    <rect x="50" y="560" width="160" height="36" rx="6" class="box"/>
    <text x="130" y="585" text-anchor="middle" class="small">dn_room_directive (9003)</text>

    <!-- arrows to progressor/field -->
    <path d="M 210 460 L 360 300" class="arrow"/>
    <path d="M 210 520 L 400 330" class="arrow"/>
  </g>

  <!-- Object Server (right) -->
  <g>
    <rect x="820" y="240" width="230" height="220" class="box" />
    <text x="935" y="268" text-anchor="middle" class="title">Object Server</text>
    <text x="935" y="288" text-anchor="middle" class="muted">(persist object_state, reflect intentions)</text>

    <rect x="840" y="320" width="190" height="36" rx="6" class="box"/>
    <text x="935" y="344" text-anchor="middle" class="small">obj_person API</text>

    <path d="M 720 260 L 840 300" class="arrow"/>
    <path d="M 680 360 L 840 360" class="arrow"/>
  </g>

  <!-- Database (bottom center) -->
  <g>
    <rect x="300" y="460" width="500" height="180" class="box" />
    <text x="550" y="488" text-anchor="middle" class="title">Durable Store (SQLite / Postgres)</text>
    <text x="550" y="510" text-anchor="middle" class="muted">tables: intentions, field_pulses, ex_instance, object_state, object_history, object_mapping, dn_namespace, cpux_registry, runtime_control, outbound_queue, webhook_map, worker_registry</text>

    <path d="M 540 360 L 540 460" class="arrow"/>
  </g>

  <!-- GridLookout renderer (optional) -->
  <g>
    <rect x="820" y="480" width="230" height="140" class="box" />
    <text x="935" y="508" text-anchor="middle" class="title">GridLookout Renderer</text>
    <text x="935" y="528" text-anchor="middle" class="muted">(visualize Field as grid cells / pulses)</text>

    <path d="M 640 520 L 820 520" class="arrow"/>
  </g>

  <!-- Legend -->
  <g>
    <rect x="30" y="620" width="1040" height="60" fill="#fff8f0" stroke="#ffd59a" rx="6" />
    <text x="60" y="642" class="small">Legend:</text>
    <text x="150" y="642" class="small">• Progressor: ordered visitation & golden-pass detection</text>
    <text x="420" y="642" class="small">• Field: persistent pulses + intentions (per thread)</text>
    <text x="780" y="642" class="small">• Claim/Atomicity: DB check‑and‑set on intentions (TTL)</text>
  </g>

</svg>
```

</div>

---

# Appendix — CPUX reference DB tables (concise)

Below are **short descriptions** of the 12 tables used by the reference implementation and the role each plays at runtime. For each table: purpose, key columns, typical access patterns, and a short note on transactional concerns.

> **Note:** the SQL `CREATE TABLE` snippets below are concise; your implementation may add `INDEX` and foreign-key constraints where appropriate.

---

## 1. `intentions`
**Purpose:** Ledger of emitted Intentions. The canonical coordination point for claims and progression.

**Key columns:** `id` (UUID), `name` (text), `scope` (`thread`|`global`), `thread_id` (nullable), `claimed_by` (worker id, nullable), `claim_time`, `created_at`.

**Runtime role:** Workers atomically claim an unclaimed row to execute it; the progressor reads recent intentions to populate the Field. Intentions are appended on DN/object completion.

**Usage / queries:**
- `INSERT` to append new intent on emission.
- `UPDATE ... WHERE id = ? AND claimed_by IS NULL` to claim atomically.
- `SELECT * WHERE scope='thread' AND thread_id=? ORDER BY created_at` to list thread-scoped intents.

**Transactional note:** Claims must be atomic; use a single-row `UPDATE` that checks `claimed_by IS NULL` and check rows affected.

---

## 2. `field_pulses`
**Purpose:** Persistent store of Pulses (name, TV, response JSON, timestamp) visible to a CPUX thread's Field.

**Key columns:** `pulse_name`, `tv` (Y/N/UN), `response` (JSON/text), `created_at`.

**Runtime role:** Signal matching and gating reads pulses from this table. DN/Object emissions append pulses.

**Usage:** `INSERT` for new pulses; `SELECT` to build signal for matching; optional TTL/garbage collection if transient pulses are used.

**Note:** Consider compacting by signal-hash and storing small payloads to avoid unbounded growth.

---

## 3. `ex_instance` (execution instance)
**Purpose:** Design-time registry of the CPUX thread members (Objects and DNs) and their namespaces, inbound/outbound mappings, and visit order.

**Key columns:** `id` (string namespace e.g., `cpux:demo:p1`), `cpux_id`, `visit_order`, `member_name` (dn or object), `member_type` (`dn`|`object`), `namespace` (execution-unique namespace), `inbound_names` (JSON array), `outbound_map` (JSON object mapping emitted intention name → target ex_instance namespace), `required_payload_keys` (JSON array)

**Runtime role:** Progressor uses `ex_instance` to know the ordered sequence to visit and what inbound intentions/signals to expect. The namespace isolates per-execution-state.

**Usage:** `SELECT * ORDER BY visit_order` to build the progressor sequence. During instantiation, new ex_instance rows are inserted (with a unique namespace for that thread instance).

**Note:** The design-time `ex_instance` template is used to derive per-thread execution instances (namespaced rows) at thread instantiation time.

---

## 4. `object_state`
**Purpose:** Persistent state JSON for Objects. Represents object-level memory (e.g., stored name, age, room) and a `last_updated` timestamp.

**Key columns:** `object_name`, `state_json` (JSON), `last_updated`, `version`

**Runtime role:** Objects persist state before reflection so their resulting intention can be made deterministic and auditable. Object server updates this row synchronously when reflecting.

**Usage:** `UPDATE object_state SET state_json=?, last_updated=?, version=version+1 WHERE object_name=?`.

**Note:** Use optimistic versions for concurrency control and a history table for auditing.

---

## 5. `object_history`
**Purpose:** Append-only timeline of object state writes for audit and debugging.

**Key columns:** `history_id`, `object_name`, `timestamp`, `state_snapshot` (JSON)

**Runtime role:** Enables rollback, debugging, and reconstructing field changes caused by Objects.

**Usage:** `INSERT` each time `object_state` is updated.

---

## 6. `object_mapping`
**Purpose:** Design-time mapping that defines how an Object reflects an inbound intention to an outbound intention (and optionally which pulses to produce or require).

**Key columns:** `id`, `object_name`, `incoming_intention`, `required_signal` (JSON), `outgoing_intention` (text or JSON mapping)

**Runtime role:** Object server uses this to decide whether to reflect an incoming intention into an outbound intention (possibly to another thread) based on pulse matching.

**Usage:** Read by Object server at reflection time; small, infrequently updated table.

---

## 7. `cpux_registry`
**Purpose:** Catalog of CPUX thread templates and status (e.g., registered cpux IDs, start intention names).

**Key columns:** `cpux_id`, `start_intention`, `status`, `created_at`

**Runtime role:** Used to enumerate available CPUX definitions and spawn threads when an initial intention arrives.

---

## 8. `dn_namespace` (DN registry)
**Purpose:** Registry describing available DN implementations, their local name, webhook URL (if remote), expected interface, and optional auth token.

**Key columns:** `dn_name`, `url`, `auth_info`, `preferred_execution` (local|webhook)

**Runtime role:** Progressor/workers use this to find how to invoke a DN (HTTP wake or local function) given an ex_instance referencing that DN.

---

## 9. `runtime_control`
**Purpose:** Operational flags and counters used by the progressor and coordinator (e.g., golden-pass counters, last_pass_time, exit_on_done flag).

**Key columns:** `id`, `golden_pass_count`, `last_pass_ts`, `exit_on_done`

**Runtime role:** Helps the coordinator detect termination conditions and apply global behaviors like `exit_on_done`.

---

## 10. `webhook_map`
**Purpose:** Convenience mapping (optionally persisted) from DN name → webhook URL for the `run_all.sh` orchestration and fast lookup.

**Key columns:** `dn_name`, `url`

**Runtime role:** Written by `run_all.sh` and read by the notifier/producer when it issues HTTP wake notifications.

---

## 11. `outbound_queue` (optional)
**Purpose:** Temporary queue for outbound notifications or intents that need guaranteed delivery to remote DN endpoints or other systems.

**Key columns:** `id`, `payload` (JSON), `target_url`, `status`, `attempts`, `last_attempt`.

**Runtime role:** If HTTP notify fails, the producer may enqueue to `outbound_queue` for retries. In the simplest local demo this is optional and may be unused.

**Note:** This table is optional; keeping the architecture queue‑optional keeps the progressor simple as you prefer.

---

## 12. `worker_registry` (or `claims`)
**Purpose:** Track active worker processes, last heartbeat, and optionally which ex_instance they are currently executing.

**Key columns:** `worker_id`, `pid`, `last_heartbeat`, `current_claim_id`

**Runtime role:** Operational visibility for coordinator and to detect crashed workers (for TTL-based claim recovery).

**Usage:** Workers periodically update heartbeats. Coordinator can decide to recover stale claims if `last_heartbeat` is old.

---

# Short SQL schema snippets (examples)

```sql
CREATE TABLE intentions (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  scope TEXT,
  thread_id TEXT,
  claimed_by TEXT,
  claim_time TEXT,
  created_at TEXT NOT NULL
);

CREATE TABLE field_pulses (
  pulse_name TEXT,
  tv TEXT,
  response TEXT,
  created_at TEXT
);

CREATE TABLE ex_instance (
  id TEXT PRIMARY KEY,
  cpux_id TEXT,
  visit_order INTEGER,
  member_name TEXT,
  member_type TEXT,
  namespace TEXT,
  inbound_names TEXT,
  outbound_map TEXT,
  required_payload_keys TEXT
);

-- object_state, object_history, object_mapping, dn_namespace etc follow similarly
```

---

# How these tables make the runtime model live (summary)
1. **Durable ledger (intentions)** ensures that emitted Intentions are persisted and claimable by any worker (atomicity is implemented via conditional UPDATE). This is the canonical coordination primitive.
2. **Field materialization (field_pulses)** records the pulses that DNs and Objects produce; progressor consults this to perform match/gating and designer-specified preconditions.
3. **Execution templates (ex_instance)** provide the order, per-instance namespaces, and the mapping that ties design-time flow to runtime behavior—progressor reads it to know what to try to execute next.
4. **Objects (object_state + mapping)** let the system persist application state and reflect intentions deterministically while allowing the DN implementations to be non-deterministic.
5. **Operational tables (runtime_control, worker_registry)** enable the coordinator semantics, golden-pass termination, and claim recovery.
6. **Optional outbound_queue / webhook_map** provide reliable delivery for remote invocation while keeping the progressor simple and local-first by default.

---

If you want, I can now:
- produce a standalone `architecture.svg` file (separate file) in the same canvas, or
- generate a downloadable ZIP containing the `SVG` and a `appendix.md` file (requires file output support).

Which would you like next?

