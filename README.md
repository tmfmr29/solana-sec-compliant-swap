# Compliant Tokenized Securities AMM (Solana)
![Test SEC Compliance Engine](https://github.com/tmfmr29/solana-sec-compliant-swap/actions/workflows/test-compliance.yml/badge.svg)
An on-chain Automated Market Maker (AMM) pool designed to swap Circle USDC for Tokenized Securities (such as Tokenized NMS Stocks) in accordance with the SEC's 5-Year Innovation Exemption for Tokenized Securities Venues (TSVs).

## Regulatory Architecture & Safeguards

1. **Permissioned Trading (OFAC & U.S. Person Verification)**
   - Swaps utilize Solana Token-2022 Transfer Hooks.
   - Every participant wallet must verify U.S. person status and clear sanctions screening before receiving tokenized assets.

2. **Synchronized Trading Halts**
   - Implements an automated circuit breaker linked to primary equity exchanges (NYSE/NASDAQ).
   - If the underlying security is halted on traditional markets, on-chain AMM liquidity is automatically frozen.

3. **Compliant Liquidity Pooling**
   - Facilitates liquidity between Circle USDC and tokenized shares without requiring exchange or dealer registration under the TSV exemption criteria.

## Tech Stack
- **Network:** Solana (Devnet / Mainnet)
- **Token Standard:** SPL Token-2022 with Transfer Hooks
- **Settlement Asset:** Circle USDC (`EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`)
- **Framework:** Anchor / Rust & TypeScript
