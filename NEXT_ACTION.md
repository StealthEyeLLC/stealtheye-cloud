# Next Action

Continue **S8 — StealthEye Cloud Remediator** on:

```text
build/s8-remediator-mode
```

Immediate work:

1. Open one PR from `build/s8-remediator-mode` to `main`.
2. Let the first CI wave run.
3. Inspect all failures before patching.
4. Batch repair exact failures only.
5. Merge when green.

S8 proof workflow:

```text
proof-remediator
```

S8 proof rule:

```text
A repo is not remediated until the failing behavior is reproduced, a bounded patch is applied, and proof gates pass. If failure cannot be reproduced, emit diagnosis-only status and do not claim remediation.
```

Boundary:

```text
No secrets, no paid compute, no production deployment, no database mutation, and no browser-cookie/session-token automation. Commercial quote/risk artifacts do not activate billing.
```
