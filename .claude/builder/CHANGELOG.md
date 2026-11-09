# Builder method: changelog

Changes to the method (`.claude/builder/`) and the agents that run it (`.claude/agents/`).
The change itself goes in those files; this log records what changed, why, and what
triggered it. A change that moves a boundary (what agents may reach or run) is an ADR in
`docs/adr/` instead, linked from here.

- **Proposed:** not yet approved. Moves to **Changes** once Gabriel approves and it is made.
- **Changes:** newest first, dated.

## Proposed

### P1: Pre-gate a spec locally before launching it
- **Trigger:** Cycle 1, 2026-11-09. The gate refused T2 (`docs/cycles/01-workspace-skeleton/logs/runs/002-T2.json`)
  after one cloud round trip; reading T2 and T3 against the same gate questions afterwards
  found three more defects, each of which would have cost its own refused run.
- **Change:** before a cycle's cloud session is launched, `main` reads every prompt in the
  spec against the orchestrator's gate questions (`.claude/agents/orchestrator.md`, "Before
  every dispatch"), fixes what it finds, and commits. Candidate form: a `/pregate <spec>`
  skill.
- **Does not replace** the orchestrator's own gate: that still runs, from disk, before each
  dispatch.

### P2: Orchestrator gates every prompt at the start of a spec
- **Trigger:** as P1. The orchestrator gates one prompt at a time and stops the spec at the
  first defect, so defects surface one per round trip.
- **Change:** at the start of a run, gate every prompt on the questions that do not depend on
  the workspace (1, 3, 5, 7, 8) and report all defects together. The workspace-dependent
  questions (2, 6) stay immediately before each dispatch, because earlier tasks change what
  is on disk.
- **Revisit:** after Cycle 1, once P1 shows how many defects remain for the orchestrator.

## Changes

_None yet. Earlier changes (Cycle 0, commit `2265f03`) predate this log._
