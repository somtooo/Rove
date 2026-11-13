# Rove: agent instructions

## Subagents
- When you spawn a subagent, use **Haiku 5.5 on high reasoning**: `model: "claude-agent/haiku#high"`.
- **Never spawn subagents on Opus.**
- **When to spawn one:** spawn Haiku for work that reads things you haven't read yet, such as a new repo, folder or doc set, especially when there are several independent pieces. Reading new material costs a cache miss either way, so the main agent saves nothing by doing it itself. Haiku does the same reading for much less, and several Haiku agents run truly in parallel.
- **When not to:** keep work in the main agent when it builds on context the main agent already holds (files it has read, decisions in flight). Handing that to a subagent means paying to re-read it.
- Give each subagent a self-contained prompt (paths, questions, the output you want), and ask for file:line citations so the main agent can check claims without re-reading everything.

## Roles
- **som2** is the owner (an engineer). Design changes go through som2, who hands them to the designer.
- **The designer** wrote the handoff. The design in `.plan/` (tokens, layout, references, components, journeys) is fixed: match it exactly.
- **The lead engineer** (the agent working here) owns every engineering decision: architecture, milestones, implementation.

## The plan
- `.plan/` is the single place for the current plan. Its engineering content (milestones, architecture) is ours to verify and change; its design content is not. If the design needs to change, tell som2.
