# Changelog

## [2.0.0] - 2026-05-27

### Added — AI Intelligence Layer (10 USP tools)
- `supplier_risk_score` — multi-factor risk assessment (0-100): country, performance, contracts, rating
- `price_benchmark` — compare quote vs historical avg/min/max with verdict (excellent/good/overpriced)
- `carbon_footprint` — CO2 kg per shipment (weight × distance × transport mode emission factor)
- `supplier_recommend` — AI-ranked supplier suggestions by composite score
- `contract_clause_check` — flag risky clauses (auto-renewal, unlimited liability, IP, non-compete, penalties)
- `demand_forecast` — predict future spend from historical PO patterns
- `savings_opportunity` — find same SKU at different prices across suppliers (>10% variance)
- `supplier_scorecard` — A/B/C/D grade (delivery %, quality %, rating, spend)
- `maverick_spend_detect` — purchases outside contracted suppliers (policy violations)
- `negotiation_brief` — leverage analysis, BATNA, talking points, recommended strategy

## [1.1.0] - 2026-05-27

### Added — Enterprise Features
- `three_way_match` — PO vs goods receipt vs invoice reconciliation (2% tolerance)
- `contract_create` — fixed price, time & materials, blanket, framework contracts
- `contract_list` — active contracts with 90-day expiry alerts
- `contract_get` — contract details
- `diversity_set` — supplier diversity certifications (minority, women, veteran, small biz, disabled, LGBTQ)
- `diversity_report` — diverse spend percentage and certification breakdown
- `budget_set` — department budget allocation by category/period
- `budget_check` — verify PO fits budget with approve/reject recommendation
- `catalog_add` — supplier punch-out catalog items
- `catalog_search` — search across all supplier catalogs
- `approval_escalate` — route to Manager/Director/CFO by amount threshold

## [1.0.0] - 2026-05-27

### Added — Core Procurement
- `supplier_create` / `supplier_list` / `supplier_get` / `supplier_rate` — supplier registry
- `po_create` / `po_list` / `po_get` / `po_approve` / `po_send` / `po_cancel` — purchase order lifecycle
- `rfq_create` / `rfq_respond` / `rfq_compare` / `rfq_award` — competitive bidding
- `goods_receive` — receive goods against PO (partial/full, rejections)
- `spend_analysis` — total spend by supplier/category
