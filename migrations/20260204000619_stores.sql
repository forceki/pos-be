-- Add migration script here

CREATE TABLE stores (
    id VARCHAR(64) PRIMARY KEY,
    company_id VARCHAR(64) NOT NULL,
    code VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    address TEXT,
    store_number_phone VARCHAR(50),
    slug VARCHAR(255) NOT NULL,
    is_warehouse BOOLEAN NOT NULL DEFAULT FALSE,
    status BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL,
    
    UNIQUE(company_id, code),
    UNIQUE(company_id, slug),
    INDEX(company_id)
);