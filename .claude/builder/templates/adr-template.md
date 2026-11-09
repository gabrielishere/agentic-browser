---
name: adr-template
description: One architecture decision — what forced it, what was chosen, what was rejected, and what it costs.
---

# Architecture Decision Record template

One question per record; a record may hold several numbered decisions (`D<n>`) that
answer it together. Never rewritten once accepted: a decision that turns out wrong is
**superseded** by a new record, or annotated in place with a dated note, because the
reasoning that looked sound at the time is the thing worth keeping.

**An ADR answers a question no other document does.** The spec says what to
build; the log says what happened; an ADR says *why this way and not the
alternative.* If a constraint in a spec has no obvious justification, its
justification is an ADR.

**Not everything is an ADR**, and the common failure is not writing too many — it is
writing the wrong *kind*. Three questions, in order. A no to any of them means this is not
an ADR, and the third column says where it goes instead.

| Ask | If no | It goes |
|---|---|---|
| **1. Is it about the system, or about how this project is built?** | it is a routine rule of the trade: a naming habit, a formatting choice | with the trade rules in this method (`.claude/`) |
| **2. Was there a real alternative?** | a constraint forced the answer | a line in the spec's `Constraints`, citing what forced it |
| **3. Does it outlive the task?** | it is an implementation choice | nowhere, or a comment |

Then the sanity check the questions serve: **would someone six months from now ask "why is
it like this?"**

Question 1 differs from the method's default here. In this repository, decisions about
how the work is done are ADRs too, in the same series as decisions about the browser
(ADR 0008–0010 are examples). They are scrutinised as process because they are written as
decisions with alternatives and costs, not because they are filed apart.

Question 2 catches the record that inflates a forced choice into a deliberation. If a
stated constraint already settles it, the honest artefact is one line naming the
constraint, not a page of rejected alternatives that were never available.

**A failed task is neither.** It is an event, and it belongs in the log.

---

````markdown
# ADR <nnnn> — <short title, the decision itself>

- **Status:** Proposed · Accepted · Superseded by ADR <nnnn>
- **Date:** <YYYY-MM-DD>
- **Deciders:** Gabriel
- **Affects:** <cycle spec ids (e.g. `01-workspace-skeleton`), tasks, or crates this bears on; omit if general>

## Context
<What forced the decision. The situation, the constraint, the failure. If it came
out of a run, cite the run record: "T3 FAILED — evidence showed …". State the
facts that were true at the time, not the conclusion.>

## Decision
<What was chosen, stated as a position: "We will …". Each decision gets the next
number in the series, `### D<n> — <title>`, contiguous across all ADRs: check the
last one in `docs/adr/` before adding.>

## Alternatives
<What else was considered, and why each was rejected. A record with no rejected
alternatives is describing a default, not a decision — and the rejected options
are the part your future self will want.>

- **<alternative>** — <why not>
- **<alternative>** — <why not>

## Consequences
<What this makes easier, and what it makes harder. Both. A record listing only
benefits is advocacy, and it will not help anyone deciding whether to revisit it.>

- <…>
- <…>
````

---

## Worked example

An illustration of the form. The record below is not one of the host repository's;
its number is part of the example.

````markdown
# ADR-001 — Commit per task, at verification

**Date:** 2026-09-07
**Status:** accepted

## Context
Tasks are dispatched to agent threads and may run concurrently. When one fails,
the mainline needs to be recoverable to a known-good state. Commit granularity
determines what can be reverted, and it is not otherwise constrained by anything
in the spec.

## Decision
We will commit once per task, at the point its row reads `DONE / MADE /
VERIFIED` — or `UNVERIFIED` where nothing can evidence the outcome. Failed and
partial tasks are not committed to the mainline.

## Alternatives
- **Commit per run** — a failed run leaves a commit indistinguishable from a
  successful one, and the mainline accumulates work that was never shown to hold.
- **Commit per spec** — one bad task poisons a commit containing several good
  ones. The log would still name the failing task, but git could no longer revert
  only that task's changes.
- **No commits until human review** — safe, but serialises the orchestrator on a
  human and forfeits the per-task recovery point that makes concurrent dispatch
  worth doing.

## Consequences
- Commit boundaries match task boundaries, so a failure is revertible exactly.
- History carries the verification state: an `UNVERIFIED` commit is visible as
  one, via its run record.
- More commits than a human would make by hand. Squashing on merge is available
  if that matters, and loses the per-task granularity when used.
- A project needing different behaviour must say so in its spec's `Constraints`,
  since this is the default the orchestrator assumes.
````
