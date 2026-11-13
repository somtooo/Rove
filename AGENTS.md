# Rove: agent instructions

## Subagents
- When you spawn a subagent, use **Haiku 5.5 on high reasoning**: `model: "claude-agent/haiku#high"`.
- **Never spawn subagents on Opus.**

## Roles
- **som2** is the owner (an engineer). Design changes go through som2, who hands them to the designer.
- **The designer** wrote the handoff. The design in `.plan/` (tokens, layout, references, components, journeys) is fixed: match it exactly.
- **The lead engineer** (the agent working here) owns every engineering decision: architecture, milestones, implementation.

## The plan
- `.plan/` is the single place for the current plan. Its engineering content (milestones, architecture) is ours to verify and change; its design content is not. If the design needs to change, tell som2.
