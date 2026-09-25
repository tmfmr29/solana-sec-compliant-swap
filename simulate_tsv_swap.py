"""
SEC Tokenized Securities Venue (TSV) Swap Simulation
Demonstrates on-chain AMM compliance rules:
1. Circle USDC settlement pairing
2. Mandatory OFAC & U.S. person verification
3. Coordinated trading halt circuit breaker (NYSE/NASDAQ)
"""

from dataclasses import dataclass
from typing import Optional

@dataclass
class ComplianceProfile:
    wallet_address: str
    is_us_person: bool
    is_ofac_sanctioned: bool
    kyc_cleared: bool

@dataclass
class MarketStatus:
    primary_exchange: str      # e.g., "NASDAQ" or "NYSE"
    symbol: str                # e.g., "NVDA"
    is_trading_halted: bool    # True if primary exchange halts trading

class TSVSwapPool:
    def __init__(self, reserve_usdc: float, reserve_security: float, symbol: str):
        self.reserve_usdc = reserve_usdc
        self.reserve_security = reserve_security
        self.symbol = symbol

    def execute_swap(
        self,
        investor: ComplianceProfile,
        usdc_in: float,
        market: MarketStatus
    ) -> Optional[float]:
        print(f"\n--- Initiating Swap: {investor.wallet_address[:8]}... depositing {usdc_in:,.2f} USDC for {self.symbol} ---")

        # Check 1: SEC Trading Halt Condition
        if market.is_trading_halted:
            print(f"[REVERT] SEC Circuit Breaker: Trading in {self.symbol} is currently halted on {market.primary_exchange}.")
            return None

        # Check 2: OFAC Sanctions Screening
        if investor.is_ofac_sanctioned:
            print("[REVERT] Compliance Transfer Hook: Wallet flagged by OFAC sanctions screening.")
            return None

        # Check 3: Verified Investor / U.S. Person Requirement
        if not (investor.is_us_person and investor.kyc_cleared):
            print("[REVERT] Compliance Transfer Hook: Participant not authorized under exemption criteria.")
            return None

        # Constant Product AMM Math: (x * y = k)
        k = self.reserve_usdc * self.reserve_security
        new_reserve_usdc = self.reserve_usdc + usdc_in
        new_reserve_security = k / new_reserve_usdc
        security_out = self.reserve_security - new_reserve_security

        self.reserve_usdc = new_reserve_usdc
        self.reserve_security = new_reserve_security

        print(f"[SUCCESS] Compliance cleared. Received {security_out:.4f} tokenized {self.symbol}.")
        print(f"Pool Reserves: {self.reserve_usdc:,.2f} USDC | {self.reserve_security:,.4f} {self.symbol}")
        return security_out


def run_compliance_tests():
    # Initialize liquidity pool: 500,000 USDC paired with 4,000 tokenized NVDA shares
    pool = TSVSwapPool(reserve_usdc=500_000.0, reserve_security=4_000.0, symbol="tokenized-NVDA")
    nasdaq_status = MarketStatus(primary_exchange="NASDAQ", symbol="NVDA", is_trading_halted=False)

    # Test Case 1: Fully verified U.S. person
    investor_alice = ComplianceProfile(
        wallet_address="7XwPz9R2tK1...Alice",
        is_us_person=True,
        is_ofac_sanctioned=False,
        kyc_cleared=True
    )
    pool.execute_swap(investor_alice, usdc_in=5_000.0, market=nasdaq_status)

    # Test Case 2: Sanctioned entity attempted swap
    investor_blocked = ComplianceProfile(
        wallet_address="4YqL8wM1vN5...Blocked",
        is_us_person=True,
        is_ofac_sanctioned=True,
        kyc_cleared=True
    )
    pool.execute_swap(investor_blocked, usdc_in=1_000.0, market=nasdaq_status)

    # Test Case 3: Primary exchange triggers a trading halt (LULD circuit breaker)
    print("\n>>> NASDAQ triggers market-wide trading halt for NVDA <<<")
    nasdaq_status.is_trading_halted = True
    pool.execute_swap(investor_alice, usdc_in=2_500.0, market=nasdaq_status)


if __name__ == "__main__":
    run_compliance_tests()
