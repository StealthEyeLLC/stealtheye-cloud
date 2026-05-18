# Authority Queue

## Purpose

The Authority Queue records which mission actions are authorized before execution.

## S1 contract

Schema: `AuthorityQueueV0`

The queue is public-safe and declarative. It does not grant hidden authority, bypass user boundaries, or create secret-bearing execution.

## Fields

1. schema version
2. queue id
3. mission id
4. approved items

## S1 role

S1 uses the Authority Queue as a validation and documentation contract. Later drops may connect it to richer workflow dispatch and file-bundle application.
