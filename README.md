# Substrate tutorial v3 - Proof of Existence DApp.

## Application Specifications
The proof of existence application exposes the following callable functions:

- create_claim() allows a user to claim the existence of a file by uploading a hash.

- revoke_claim() allows the current owner of a claim to revoke ownership.

These functions only require information about the proofs that have been claimed is stored, as well as who made the claims.
