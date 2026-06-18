pub mod overview;
pub mod purchase;
pub mod master;
pub mod logistics;
pub mod outbound;
pub mod inbound;
pub mod summary;

pub use overview::OverviewPage;
pub use purchase::PurchasePage;
pub use master::MasterPage;
pub use logistics::LogisticsPage;
pub use inbound::InboundPage; 
pub use outbound::OutboundPage;
pub use summary::SummaryPage;
