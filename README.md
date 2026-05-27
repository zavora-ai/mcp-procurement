# Procurement MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-procurement.svg)](https://crates.io/crates/mcp-procurement)
[![Docs.rs](https://docs.rs/mcp-procurement/badge.svg)](https://docs.rs/mcp-procurement)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)
[![Registry Ready](https://img.shields.io/badge/ADK_Registry-Ready-green.svg)](https://enterprise.adk-rust.com)

AI-powered procurement engine for [ADK-Rust Enterprise](https://enterprise.adk-rust.com) agents. Provides 37 MCP tools covering the full source-to-pay lifecycle — supplier management, purchase orders, RFQs, contracts, goods receipt, 3-way matching, budgets, diversity tracking, and **10 unique intelligence tools** for risk scoring, price benchmarking, carbon footprint, demand forecasting, and AI-powered negotiation briefs. **Feature parity with SAP Ariba and Coupa, plus capabilities neither offers.**

## Architecture

<p align="center">
  <img src="https://raw.githubusercontent.com/zavora-ai/mcp-procurement/main/docs/assets/architecture.svg" alt="Procurement MCP Architecture" width="900"/>
</p>

## Key Principles

- **Source-to-pay lifecycle** — from supplier onboarding through payment reconciliation.
- **AI-powered intelligence** — 10 tools that no competitor offers: risk scoring, benchmarking, ESG, forecasting.
- **3-way matching** — automatic PO vs receipt vs invoice reconciliation with tolerance.
- **ESG built-in** — carbon footprint estimation and supplier diversity tracking.
- **Governance** — multi-level approvals, budget checks, maverick spend detection.
- **Zero configuration** — starts immediately with no external dependencies.

## Tools (37)

### Supplier Management (4)

| Tool | Description |
|------|-------------|
| `supplier_create` | Register supplier (name, category, country, terms, contact) |
| `supplier_list` | List all suppliers |
| `supplier_get` | Get supplier details |
| `supplier_rate` | Rate supplier (0-5 stars) |

### Purchase Orders (6)

| Tool | Description |
|------|-------------|
| `po_create` | Create PO with line items |
| `po_list` | List all POs |
| `po_get` | Get PO details |
| `po_approve` | Approve PO (governance gate) |
| `po_send` | Send to supplier |
| `po_cancel` | Cancel PO |

### RFQ — Request for Quotation (4)

| Tool | Description |
|------|-------------|
| `rfq_create` | Create RFQ, send to multiple suppliers |
| `rfq_respond` | Submit supplier quote (prices, lead time) |
| `rfq_compare` | Side-by-side comparison of all responses |
| `rfq_award` | Award to winning supplier |

### Contracts (3)

| Tool | Description |
|------|-------------|
| `contract_create` | Create contract (fixed price, T&M, blanket, framework) |
| `contract_list` | List contracts (flags expiring within 90 days) |
| `contract_get` | Get contract details |

### Goods Receipt & Matching (2)

| Tool | Description |
|------|-------------|
| `goods_receive` | Receive goods (partial/full, rejections) |
| `three_way_match` | PO vs receipt vs invoice — flags discrepancies |

### Budget & Compliance (5)

| Tool | Description |
|------|-------------|
| `budget_set` | Set department budget by category/period |
| `budget_check` | Verify PO fits budget (approve/reject recommendation) |
| `diversity_set` | Set supplier diversity certifications |
| `diversity_report` | Diverse spend %, certification breakdown |
| `approval_escalate` | Route to Manager/Director/CFO by amount |

### Catalogs (2)

| Tool | Description |
|------|-------------|
| `catalog_add` | Add items to supplier punch-out catalog |
| `catalog_search` | Search across all supplier catalogs |

### Analytics (1)

| Tool | Description |
|------|-------------|
| `spend_analysis` | Total spend by supplier/category |

### 🏆 Intelligence — Unique Selling Proposition (10)

| Tool | Description | Why unique |
|------|-------------|-----------|
| `supplier_risk_score` | Multi-factor risk score (0-100): country, performance, contracts, rating | Real-time risk assessment |
| `price_benchmark` | Compare quote vs historical avg/min/max — verdict + recommendation | Automatic price intelligence |
| `carbon_footprint` | CO2 kg per shipment (weight × distance × transport mode) | ESG compliance built-in |
| `supplier_recommend` | AI-ranked supplier suggestions by score | Eliminates manual searching |
| `contract_clause_check` | Flag risky clauses (auto-renewal, liability, IP, non-compete) | Legal risk automation |
| `demand_forecast` | Predict future spend from historical patterns | Proactive procurement |
| `savings_opportunity` | Find same SKU at different prices across suppliers | Automatic cost reduction |
| `supplier_scorecard` | A/B/C/D grade (delivery %, quality %, rating) | Data-driven management |
| `maverick_spend_detect` | Purchases outside contracted suppliers | Compliance automation |
| `negotiation_brief` | Leverage analysis, BATNA, talking points, strategy | AI negotiation prep |

## Installation

```bash
cargo install mcp-procurement
```

### Client Configuration

```json
{
  "mcpServers": {
    "procurement": { "command": "mcp-procurement" }
  }
}
```

## Quick Start

### 1. Register suppliers

```json
{"name": "supplier_create", "arguments": {"name": "Acme Steel Ltd", "category": "raw_materials", "country": "KE", "payment_terms": "net30"}}
{"name": "supplier_create", "arguments": {"name": "Global Parts Inc", "category": "raw_materials", "country": "CN", "payment_terms": "net60"}}
```

### 2. Run an RFQ

```json
{"name": "rfq_create", "arguments": {"title": "Steel Rods Q3", "items": [{"description": "Steel Rod 12mm", "quantity": 5000, "unit": "kg"}], "supplier_ids": ["sup_abc", "sup_def"], "deadline": "2026-07-01", "created_by": "james"}}
{"name": "rfq_respond", "arguments": {"rfq_id": "rfq_xyz", "supplier_id": "sup_abc", "unit_prices": [2.50], "lead_time_days": 14}}
{"name": "rfq_respond", "arguments": {"rfq_id": "rfq_xyz", "supplier_id": "sup_def", "unit_prices": [2.20], "lead_time_days": 21}}
{"name": "rfq_compare", "arguments": {"po_id": "rfq_xyz"}}
```

### 3. Create and approve PO

```json
{"name": "po_create", "arguments": {"supplier_id": "sup_def", "lines": [{"sku": "STEEL-12MM", "description": "Steel Rod 12mm", "quantity": 5000, "unit_price": 2.20}], "created_by": "james"}}
{"name": "budget_check", "arguments": {"department": "manufacturing", "amount": 11000}}
{"name": "po_approve", "arguments": {"po_id": "po_abc", "approved_by": "director_kim"}}
```

### 4. Receive and match

```json
{"name": "goods_receive", "arguments": {"po_id": "po_abc", "lines": [{"sku": "STEEL-12MM", "quantity_received": 4950, "quantity_rejected": 50, "rejection_reason": "surface defects"}], "received_by": "warehouse_john"}}
{"name": "three_way_match", "arguments": {"po_id": "po_abc", "invoice_number": "INV-2026-0891", "invoice_amount": 10890}}
```

### 5. Intelligence tools

```json
{"name": "supplier_risk_score", "arguments": {"supplier_id": "sup_def"}}
{"name": "price_benchmark", "arguments": {"sku": "STEEL-12MM", "quoted_price": 2.50}}
{"name": "carbon_footprint", "arguments": {"supplier_id": "sup_def", "weight_kg": 5000, "distance_km": 8500, "transport_mode": "sea"}}
{"name": "negotiation_brief", "arguments": {"supplier_id": "sup_def", "items": ["Steel Rod 12mm"], "target_discount_pct": 15}}
```

## Procurement Flow

```
supplier_create → rfq_create → rfq_respond (×N) → rfq_compare → rfq_award
                                                         │
                    budget_check ← po_create ←───────────┘
                         │
                    po_approve → po_send → goods_receive → three_way_match
                         │                                        │
                 approval_escalate (if over threshold)      invoice paid ✓
```

## Intelligence Tools — Deep Dive

### Supplier Risk Score
```json
{"risk_score": 72, "risk_level": "medium_risk", "factors": {"country_risk": 70, "performance": 85, "contract_coverage": 40, "rating": 80}}
```

### Price Benchmark
```json
{"quoted_price": 2.50, "historical_avg": 2.35, "vs_average_pct": 6.38, "verdict": "acceptable"}
```

### Carbon Footprint
```json
{"co2_kg": 6.8, "transport_mode": "sea", "trees_to_offset": 0.32, "rating": "low"}
```

### Negotiation Brief
```json
{"leverage": "strong", "alternative_suppliers": 4, "strategy": "competitive_pressure", "talking_points": ["We've spent USD 150,000 over 23 orders", "We have 4 alternative suppliers"], "batna": "Switch to alternative supplier"}
```

## Competitive Comparison

| Feature | SAP Ariba | Coupa | Oracle | **Us** |
|---------|:-:|:-:|:-:|:-:|
| Supplier management | ✅ | ✅ | ✅ | ✅ |
| Purchase orders | ✅ | ✅ | ✅ | ✅ |
| RFQ/RFP | ✅ | ✅ | ✅ | ✅ |
| 3-way match | ✅ | ✅ | ✅ | ✅ |
| Contracts | ✅ | ✅ | ✅ | ✅ |
| Supplier diversity | ✅ | ✅ | ❌ | ✅ |
| Punch-out catalogs | ✅ | ✅ | ✅ | ✅ |
| Budget checking | ✅ | ✅ | ✅ | ✅ |
| Multi-level approval | ✅ | ✅ | ✅ | ✅ |
| **AI risk scoring** | ❌ | ❌ | ❌ | ✅ |
| **Price benchmarking** | Partial | Partial | ❌ | ✅ |
| **Carbon footprint** | ❌ | ❌ | ❌ | ✅ |
| **AI supplier recommend** | ❌ | ❌ | ❌ | ✅ |
| **Contract clause check** | ❌ | ❌ | ❌ | ✅ |
| **Demand forecast** | ❌ | ❌ | ❌ | ✅ |
| **Savings detection** | Partial | ✅ | ❌ | ✅ |
| **Supplier scorecard** | ✅ | ✅ | ✅ | ✅ |
| **Maverick spend** | ✅ | ✅ | ❌ | ✅ |
| **Negotiation brief** | ❌ | ❌ | ❌ | ✅ |
| Zero config | ❌ | ❌ | ❌ | ✅ |
| Open source | ❌ | ❌ | ❌ | ✅ |

## Error Codes

| Code | Meaning |
|------|---------|
| `SUPPLIER_NOT_FOUND` | Supplier ID doesn't exist |
| `PO_NOT_FOUND` | Purchase order ID doesn't exist |
| `PO_NOT_IN_DRAFT` | PO already approved/sent |
| `PO_NOT_APPROVED` | Can't send unapproved PO |
| `RFQ_NOT_FOUND` | RFQ ID doesn't exist |
| `CONTRACT_NOT_FOUND` | Contract ID doesn't exist |

## Integration

| Server | How it connects |
|--------|----------------|
| `mcp-inventory` | `stock_receive` on goods receipt |
| `mcp-pricing` | Price rules for catalog items |
| `mcp-workflow` | Approval workflows for POs |
| `mcp-legal` | Sanctions screening for suppliers |
| `mcp-messaging` | Notify suppliers on PO send |

## Documentation

| Document | Description |
|----------|-------------|
| [mcp-server.toml](mcp-server.toml) | ADK-Rust Enterprise registry manifest |
| [Rust Docs](https://docs.rs/mcp-procurement) | Generated API documentation |

## Contributing

Contributions welcome. Priority areas:
- Supplier portal (self-service onboarding)
- E-procurement marketplace
- Auction/reverse auction support
- AP automation (invoice processing)
- Supplier collaboration messaging

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START -->
| [<img src="https://github.com/jkmaina.png" width="80px;" alt=""/><br /><sub><b>James Karanja Maina</b></sub>](https://github.com/jkmaina) |
|:---:|
<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Apache-2.0 — see [LICENSE](LICENSE) for details.

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem.

Built with ❤️ by [Zavora AI](https://zavora.ai)

## Registry Compliance

This server implements the [ADK MCP SDK](https://crates.io/crates/adk-mcp-sdk) contract:

- **HealthCheck** — async health probe for registry monitoring
- **mcp-server.toml** — manifest declaring tools, risk classes, and credentials
- **Structured tracing** — `RUST_LOG` env-filter for observability
- **Governance gates** — PO approval required, budget checks, multi-level escalation
