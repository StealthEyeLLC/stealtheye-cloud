# Hardening and Cost Guards

## Purpose

S5 hardening confirms that StealthEye Cloud remains public-safe, no-local, and cost-controlled.

## Required hardening checks

1. no forbidden files
2. complete schema inventory
3. green fast proof
4. green full proof
5. green browser proof
6. green targeted Windows proof when triggered
7. no local requirement
8. no paid compute requirement

## Cost posture

Default proof uses public GitHub Actions on the public repository. Paid compute, premium workers, deployment, and account-level changes require explicit approval.

## Boundary

Hardening checks are gates. They must not be bypassed by weakening validators, deleting tests, or hiding failures.
