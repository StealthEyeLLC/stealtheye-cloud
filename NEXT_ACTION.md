# Next Action

Continue **S7 — First Real Activations** on:

```text
build/s7-first-real-activations
```

Immediate work:

1. Open one PR from `build/s7-first-real-activations` to `main`.
2. Let the first CI wave run.
3. Inspect all failures before patching.
4. Batch repair exact failures only.
5. Merge when green.

S7 activated lanes:

1. Mobile Browser Game Preview and Playtest Activation
2. Notification Lane Activation
3. Knowledge Mirror Export Activation

S7 proof workflows:

```text
proof-mobile
proof-activations
```

Boundary:

```text
No secrets, no paid compute, no production deployment, no database mutation, and no browser-cookie/session-token automation. Notification real dispatch requires explicit secret plus enable flag. Knowledge mirror export is a static public-safe GitHub artifact, not live external sync.
```

After S7 merges green, proceed to:

```text
S8 — StealthEye Cloud Remediator
build/s8-remediator-mode
```
