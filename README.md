# halcn

Cross-Market Signal Propagation Protocol - Predict where signals land before they arrive

## Overview

halcn is a cross-market signal propagation protocol that detects, models, and predicts how trading signals propagate across interconnected markets.

The core engine is built on Solana using the Anchor framework.

## Architecture

```mermaid
graph LR
    A[Signal Source] --> B[Detection Layer]
    B --> C[Propagation Engine]
    C --> D[Impact Predictor]
    D --> E[Pre-arrival Alert]
```

## License

MIT -- see [LICENSE](LICENSE) for details.
