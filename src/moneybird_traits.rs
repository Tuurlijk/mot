use crate::moneybird::types::{Contact, ContactRead};

impl From<ContactRead> for Contact {
    fn from(read: ContactRead) -> Self {
        Contact {
            address1: read.address1,
            address2: read.address2,
            bank_account: read.bank_account,
            chamber_of_commerce: read.chamber_of_commerce,
            city: read.city,
            company_name: read.company_name,
            country: read.country,
            customer_id: read.customer_id,
            delivery_method: read.delivery_method,
            direct_debit: read.direct_debit,
            email_ubl: read.email_ubl,
            estimate_workflow_id: read.estimate_workflow_id,
            firstname: read.firstname,
            id: read.id,
            invoice_workflow_id: read.invoice_workflow_id,
            lastname: read.lastname,
            phone: read.phone,
            send_estimates_to_attention: read.send_estimates_to_attention,
            send_estimates_to_email: read.send_estimates_to_email,
            send_invoices_to_attention: read.send_invoices_to_attention,
            send_invoices_to_email: read.send_invoices_to_email,
            sepa_active: read.sepa_active,
            sepa_bic: read.sepa_bic,
            sepa_iban: read.sepa_iban,
            sepa_iban_account_name: read.sepa_iban_account_name,
            sepa_mandate_date: read.sepa_mandate_date,
            sepa_mandate_id: read.sepa_mandate_id,
            sepa_sequence_type: read.sepa_sequence_type,
            si_identifier: read.si_identifier,
            si_identifier_type: read.si_identifier_type,
            tax_number: read.tax_number,
            zipcode: read.zipcode,
        }
    }
}
