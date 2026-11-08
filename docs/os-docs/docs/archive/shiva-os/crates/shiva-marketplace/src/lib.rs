// PROVENANZ (Org-Adoption 13.09.2026): Quelle: privates Repo ShivaCoreDev/A-TownChain--kosystems- (05.07.2026). Archiv-Referenz (nicht buildbar gepflegt).

#![no_std]
#![no_main]

use shiva_agent::{agent_task, ShivaAgent, AgentContext};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;

// ===================================================
// 1. Task-Definitionen
// ===================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketplaceTask {
    ListServices,
    FindService { service_type: String, max_price: u64 },
    OfferService { service_type: String, price: u64, description: String },
    PurchaseService { service_id: String, agent_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOffer {
    pub id: String,
    pub agent_id: String,
    pub service_type: String,
    pub price: u64,
    pub description: String,
    pub reputation: u8,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketplaceResult {
    Services { offers: Vec<ServiceOffer>, count: usize },
    Match { found: Option<ServiceOffer> },
    OfferAccepted { service_id: String, status: String },
    PurchaseConfirmed { transaction_id: String, amount: u64 },
    Error(String),
}

// ===================================================
// 2. Agent-Implementierung
// ===================================================
#[derive(Default)]
pub struct MarketplaceAgent {
    services: BTreeMap<String, ServiceOffer>,
    transactions: BTreeMap<String, u64>,
}

#[shiva_agent]
impl MarketplaceAgent {
    pub fn new() -> Self {
        MarketplaceAgent {
            services: BTreeMap::new(),
            transactions: BTreeMap::new(),
        }
    }

    #[agent_task("list_services")]
    pub fn list_services(&self) -> MarketplaceResult {
        let offers: Vec<ServiceOffer> = self.services
            .values()
            .filter(|s| s.active)
            .cloned()
            .collect();
        MarketplaceResult::Services { offers, count: offers.len() }
    }

    #[agent_task("find_service")]
    pub fn find_service(&self, service_type: String, max_price: u64) -> MarketplaceResult {
        let found = self.services
            .values()
            .find(|s| s.service_type == service_type && s.price <= max_price && s.active)
            .cloned();
        MarketplaceResult::Match { found }
    }

    #[agent_task("offer_service")]
    pub fn offer_service(&mut self, service_type: String, price: u64, description: String) -> MarketplaceResult {
        // In der Realität: Agent-ID aus Kontext holen
        let agent_id = "shiva-agent-001".to_string();
        let service_id = format!("{}-{}", service_type, self.services.len() + 1);
        let offer = ServiceOffer {
            id: service_id.clone(),
            agent_id: agent_id.clone(),
            service_type,
            price,
            description,
            reputation: 100,
            active: true,
        };
        self.services.insert(service_id.clone(), offer);
        MarketplaceResult::OfferAccepted {
            service_id,
            status: "Successfully offered".to_string(),
        }
    }

    #[agent_task("purchase_service")]
    pub fn purchase_service(&mut self, service_id: String, agent_id: String) -> MarketplaceResult {
        if let Some(offer) = self.services.get_mut(&service_id) {
            if !offer.active {
                return MarketplaceResult::Error("Service not active".to_string());
            }
            // Transaktion simulieren
            let transaction_id = format!("tx-{}", self.transactions.len() + 1);
            self.transactions.insert(transaction_id.clone(), offer.price);
            offer.active = false; // Service deaktivieren
            MarketplaceResult::PurchaseConfirmed {
                transaction_id,
                amount: offer.price,
            }
        } else {
            MarketplaceResult::Error("Service not found".to_string())
        }
    }

    pub fn init(&mut self, ctx: &AgentContext) -> Result<(), String> {
        log::info!("💰 Marketplace initialisiert mit ID: {}", ctx.agent_id);
        // Demo-Services hinzufügen
        self.services.insert(
            "math-1".to_string(),
            ServiceOffer {
                id: "math-1".to_string(),
                agent_id: "math-agent-1".to_string(),
                service_type: "math".to_string(),
                price: 1,
                description: "Mathematische Berechnungen".to_string(),
                reputation: 95,
                active: true,
            }
        );
        Ok(())
    }

    pub fn shutdown(&self) {
        log::info!("💰 Marketplace wird beendet.");
    }
}

// ===================================================
// 4. WASM-Exporte
// ===================================================
#[wasm_bindgen]
pub fn process_task(task_json: &str) -> String {
    let mut agent = MarketplaceAgent::new();
    let task: MarketplaceTask = serde_json::from_str(task_json).unwrap();
    let result = match task {
        MarketplaceTask::ListServices => agent.list_services(),
        MarketplaceTask::FindService { service_type, max_price } => agent.find_service(service_type, max_price),
        MarketplaceTask::OfferService { service_type, price, description } => agent.offer_service(service_type, price, description),
        MarketplaceTask::PurchaseService { service_id, agent_id } => agent.purchase_service(service_id, agent_id),
    };
    serde_json::to_string(&result).unwrap()
}

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Info).unwrap();
    }
}
