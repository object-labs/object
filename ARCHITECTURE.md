# Object Architecture

Object is a byzantine-safe EVM blockchain layer on Transit for a spot and perpetual exchange.
The design goal is to make node operation economically rational by splitting consensus-costly responsibilities from read and relay pathways while keeping finality, replay, and auditability explicit.

## Architecture at a Glance

```mermaid
flowchart TB
  subgraph External
    C["Clients\n(RPC/API Traders)"]
    B["Bridge Networks"]
    O["Operators"]
  end

  subgraph Gateway["Gateway Layer"]
    A["Transaction Admission"]
    R["RPC/Control APIs"]
  end

  subgraph Transit["Transit (Event/Substrate Layer)"]
    TI["tx.ingress"]
    TC["consensus.candidate"]
    TV["consensus.vote"]
    TF["consensus.finality"]
    TE["exec.block"]
    TP["query.projection"]
    TB["bridge.events"]
  end

  subgraph Consensus["Byzantine Consensus"]
    S["Slot/Round Scheduler"]
    P["Proposal Engine"]
    V["Vote Aggregation"]
    F["Finality Engine"]
  end

  subgraph Execution["Execution"]
    E["EVM Execution Core"]
    SX["Spot Market Runtime"]
    PX["Perps Market Runtime"]
    J["Receipt / Trace Journal"]
    M["State Materialization"]
  end

  subgraph Data["Query & Analytics"]
    Q["Query Nodes"]
    I["Indexes / Analytics"]
  end

  subgraph Bridge["Bridge Stack"]
    L["Lock Monitor"]
    R1["Relayer/Attestation"]
    M1["Mint/Burn Reconciliation"]
  end

  C --> A
  A --> TI
  B --> R1
  O --> R
  R --> A
  R --> Q

  TI --> S
  S --> P
  P --> TC
  S --> E
  P --> SX
  P --> PX
  E --> J
  SX --> J
  PX --> J
  TC --> V
  V --> F
  J --> TE
  F --> TF
  F --> M
  M --> TP
  TP --> Q
  TE --> I
  I --> R

  TF --> L
  L --> R1
  R1 --> M1
  M1 --> TB
```

## Design Rationale

- **Byzantine Layer owns consensus finality**: ordering and commitment are consensus outputs, not assumed from transport.
- **Transit owns durable sequencing and lineage**: proposals, votes, finality, and execution artifacts are immutable stream records with parent ancestry and stream policy.
- **Execution is deterministic but not authoritative**: execution outputs are committed only after byzantine finality signals are validated.
- **Query/data paths are separated from consensus paths**: read-optimized nodes can scale independently and recover from finalized checkpoints.
- **Bridges are consumer-of-finality**: cross-domain mint/burn actions use finalized evidence, not speculative lanes.

## Exchange Product Model

Object is purpose-built as an exchange infrastructure stack with two native market paths:

- **Spot markets**: order lifecycle, match commitment, and settlement accounting are finalized via byzantine finality and EVM state updates.
- **Perpetual markets**: perpetual position accounting, funding rate updates, and liquidation controls share the same finality substrate with stricter market invariants.

The intended execution model is "one finality fabric, two market types":

- Spot and perpetual order streams share the same ingress and candidate ordering path.
- Settlement and risk metadata are routed into market-specific modules at execution time.
- Both market types emit into the same projection layer so query nodes can serve unified account and trade state.

```mermaid
flowchart LR
  subgraph Input["Market Ingress"]
    OI["Order/Cancel Ingress"]
    RM["Risk Policy"]
  end

  subgraph Core["Market Engine"]
    SE["Spot Engine"]
    PE["Perps Engine"]
    ME["Market Router"]
  end

  subgraph Tx["Execution & Settlement"]
    EV["EVM Contracts"]
    FC["Finality Checkpoint"]
  end

  OI --> ME
  RM --> ME
  ME --> SE
  ME --> PE
  SE --> EV
  PE --> EV
  EV --> FC
```

## Canonical Runtime Flow

```mermaid
sequenceDiagram
  autonumber
  participant G as Gateway
  participant I as Transit.tx.ingress
  participant V as Validators
  participant T as Transit Streams
  participant X as EVM Core
  participant F as Finality Engine
  participant Q as Query Node
  participant BR as Bridge Relayer

  G->>I: Append tx envelope
  I->>T: Persist to tx.ingress
  V->>T: Observe pending candidates
  V->>X: Execute candidate set
  X->>T: Emit exec.block outputs
  V->>T: Emit proposal and votes
  T->>F: Aggregate votes against round policy
  F->>T: Emit consensus.finality checkpoint
  T->>Q: Publish finalized checkpoint + state references
  Q->>Q: Project state/indexes from finalized streams
  T->>BR: Publish bridge-relevant finalized events
  BR->>BR: Produce and broadcast attestations
```

## High Frequency Exchange Flow

```mermaid
sequenceDiagram
  autonumber
  participant UI as Trader UI / API Client
  participant GW as Exchange Gateway
  participant TX as tx.ingress
  participant M as Market Router
  participant S as Spot Engine
  participant P as Perps Engine
  participant E as EVM Core
  participant F as Finality Engine
  participant Q as Query Service

  UI->>GW: Submit signed order/cancel
  GW->>TX: append market tx
  TX->>M: classify market type
  M->>S: route spot intent
  M->>P: route perpetual intent
  S->>E: execute spot match transition
  P->>E: execute perpetual position transition
  E->>F: include execution proofs + receipts
  F->>Q: update spot/perps projections
```

## Primary Subsystems

### 1) Byzantine Consensus Subsystem
Responsibility: protocol safety and liveness for candidate ordering and commitment.

- Slot/round scheduling with deterministic proposer rotation.
- Proposal encoding, signed acknowledgements, and vote aggregation.
- Finality condition thresholds and fault-handling logic.
- Finality stream records as the single canonical commit signal.

### 2) Transit Stream Subsystem
Responsibility: immutable transport and lineage.

- Stream namespaces for each subsystem and pipeline stage.
- Stream policies that define admissible writers and expected provenance.
- Branching/reconciliation semantics for uncommitted candidate work.
- Recovery via checkpoint lineage and manifest verification.

### 3) Execution Subsystem
Responsibility: state transition correctness.

- Deterministic EVM execution from candidate payloads.
- Receipts, trie roots, and trace metadata.
- Checkpoint coupling for deterministic replay and fork closure.

### 4) Interface Subsystem
Responsibility: external accessibility and service stability.

- Transaction admission and mempool policy enforcement.
- Node control, health, and status APIs.
- Read-side serving and analytics through finalized projections.

### 5) Bridge Subsystem
Responsibility: secure interoperability.

- Observe finalized events and finality proofs from Transit.
- Validate transfer proofs against chain history.
- Publish bridge event and relay attestation streams.

## Transit Stream Taxonomy (Proposed)

All stream names are prefixed with `object.<chain>` and follow explicit suffix structure.

- `object.<chain>.tx.ingress`
- `object.<chain>.exchange.orders`
- `object.<chain>.exchange.cancellations`
- `object.<chain>.exchange.match.plan.<slot>`
- `object.<chain>.consensus.candidate.<slot>`
- `object.<chain>.consensus.vote.<slot>.<validator_id>`
- `object.<chain>.consensus.finality.<slot>`
- `object.<chain>.exec.block.<height>`
- `object.<chain>.state.snapshot.<kind>`
- `object.<chain>.spot.orderbook.<market>`
- `object.<chain>.spot.trade.<market>`
- `object.<chain>.perps.position.<market>`
- `object.<chain>.perps.funding.<market>`
- `object.<chain>.query.accounts`
- `object.<chain>.query.logs`
- `object.<chain>.bridge.events`
- `object.<chain>.bridge.attestations`

Every stream entry includes:

- chain/slot context
- previous stream references (lineage)
- actor identity and policy marker
- finality pointer when committed
- hash commitments for replay integrity

## Node Topologies

```mermaid
flowchart LR
  subgraph Validators["Validator Nodes"]
    V1["Validator"]
    V2["Validator"]
    V3["Validator"]
  end

  subgraph Infrastructure["Infrastructure Nodes"]
    G1["Gateway Node"]
    Q1["Query Node"]
    B1["Bridge Node"]
  end

  V1 -->|proposal/vote/finality| TBus["Transit Streams"]
  V2 -->|proposal/vote/finality| TBus
  V3 -->|proposal/vote/finality| TBus
  G1 -->|submission + control| TBus
  B1 -->|proof emit| TBus
  TBus -->|finalized checkpoints| Q1
```

- Validators produce consensus-critical records and keep byzantine safety invariants.
- Gateways and query nodes are operationally separable from voting paths.
- Bridge nodes consume finalized outputs and emit attestation artifacts.

## Fault and Recovery Model

- Divergent candidates can exist transiently as branches in Transit.
- Canonical replay always starts from the latest byzantine-finalized checkpoint.
- Non-finalized branches are eventually garbage-collected once finality has moved forward and proofs are reconciled.
- Operator recovery uses checkpoint + checkpoint-manifest replay, then deterministic re-execution over replayed candidates.

## Native Bridge Fund Flow

Funds move in and out of Object through explicit bridge-native protocol actions:

- **Deposit / ingress**:
  - bridge module observes upstream lock or mint attestations,
  - emits finalized credit events in `object.<chain>.bridge.events`,
  - and credits execution state through finalized checkpoints.
- **Withdrawal / egress**:
  - users burn or escrow native claim objects on Object,
  - validators finalize burn events under byzantine finality,
  - relayers relay proofs to destination chains for release.

Native bridging is also the settlement path for cross-market liquidity transfers, which avoids introducing non-canonical transfer pathways outside consensus and Transit-recorded events.

## Current State

This repository is at the design and scaffolding stage for these components.

- Rust project and tooling layout are present.
- Core protocol modules are not fully implemented yet.
- The architecture and process documents are intended to remain the authoritative implementation target as those modules land.

## References

- Transit repository: https://github.com/spoke-sh/transit  
- Transit reference docs: https://www.spoke.sh/transit
