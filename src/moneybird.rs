#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
    ///Object describing an administration.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Object describing an administration.",
    ///  "type": "object",
    ///  "properties": {
    ///    "access": {
    ///      "description": "The access level of the current user to the administration.",
    ///      "type": "string"
    ///    },
    ///    "country": {
    ///      "description": "The country of the administration.",
    ///      "type": "string"
    ///    },
    ///    "currency": {
    ///      "description": "The currency used in the administration.",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "The ID of the administration.",
    ///      "type": "string"
    ///    },
    ///    "language": {
    ///      "description": "The language of the administration.",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The name of the administration.",
    ///      "type": "string"
    ///    },
    ///    "time_zone": {
    ///      "description": "The timezone of the administration.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Administration {
        ///The access level of the current user to the administration.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub access: ::std::option::Option<::std::string::String>,
        ///The country of the administration.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub country: ::std::option::Option<::std::string::String>,
        ///The currency used in the administration.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub currency: ::std::option::Option<::std::string::String>,
        ///The ID of the administration.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        ///The language of the administration.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub language: ::std::option::Option<::std::string::String>,
        ///The name of the administration.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///The timezone of the administration.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time_zone: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&Administration> for Administration {
        fn from(value: &Administration) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Administration {
        fn default() -> Self {
            Self {
                access: Default::default(),
                country: Default::default(),
                currency: Default::default(),
                id: Default::default(),
                language: Default::default(),
                name: Default::default(),
                time_zone: Default::default(),
            }
        }
    }
    impl Administration {
        pub fn builder() -> builder::Administration {
            Default::default()
        }
    }
    ///Object which describes a contact.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Object which describes a contact.",
    ///  "type": "object",
    ///  "properties": {
    ///    "address1": {
    ///      "type": "string"
    ///    },
    ///    "address2": {
    ///      "type": "string"
    ///    },
    ///    "bank_account": {
    ///      "type": "string"
    ///    },
    ///    "chamber_of_commerce": {
    ///      "type": "string"
    ///    },
    ///    "city": {
    ///      "type": "string"
    ///    },
    ///    "company_name": {
    ///      "description": "A contact requires a non-blank company_name, firstname or lastname.",
    ///      "type": "string"
    ///    },
    ///    "country": {
    ///      "description": "ISO two-character country code, e.g. NL or DE.",
    ///      "type": "string"
    ///    },
    ///    "customer_id": {
    ///      "description": "Will be assigned automatically if empty. Should be unique for the administration.",
    ///      "type": "string"
    ///    },
    ///    "delivery_method": {
    ///      "description": "Can be Email, Simplerinvoicing, Post or Manual.",
    ///      "type": "string"
    ///    },
    ///    "direct_debit": {
    ///      "type": "boolean"
    ///    },
    ///    "email_ubl": {
    ///      "type": "boolean"
    ///    },
    ///    "estimate_workflow_id": {
    ///      "description": "Should be a valid estimate workflow id.",
    ///      "type": "string"
    ///    },
    ///    "firstname": {
    ///      "description": "A contact requires a non-blank company_name, firstname or lastname.",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "The ID of the contact.",
    ///      "type": "string"
    ///    },
    ///    "invoice_workflow_id": {
    ///      "description": "Should be a valid invoice workflow id.",
    ///      "type": "string"
    ///    },
    ///    "lastname": {
    ///      "description": "A contact requires a non-blank company_name, firstname or lastname.",
    ///      "type": "string"
    ///    },
    ///    "phone": {
    ///      "type": "string"
    ///    },
    ///    "send_estimates_to_attention": {
    ///      "type": "string"
    ///    },
    ///    "send_estimates_to_email": {
    ///      "description": "Should be one or more valid email addresses, separated by a comma.",
    ///      "type": "string"
    ///    },
    ///    "send_invoices_to_attention": {
    ///      "type": "string"
    ///    },
    ///    "send_invoices_to_email": {
    ///      "description": "Should be one or more valid email addresses, separated by a comma.",
    ///      "type": "string"
    ///    },
    ///    "sepa_active": {
    ///      "description": "When true, all other SEPA fields are required.",
    ///      "type": "boolean"
    ///    },
    ///    "sepa_bic": {
    ///      "description": "Should be a valid BIC.",
    ///      "type": "string"
    ///    },
    ///    "sepa_iban": {
    ///      "description": "Should be a valid IBAN.",
    ///      "type": "string"
    ///    },
    ///    "sepa_iban_account_name": {
    ///      "type": "string"
    ///    },
    ///    "sepa_mandate_date": {
    ///      "description": "Should be a date in the past.",
    ///      "type": "string"
    ///    },
    ///    "sepa_mandate_id": {
    ///      "type": "string"
    ///    },
    ///    "sepa_sequence_type": {
    ///      "description": "Can be RCUR, FRST, OOFF or FNAL.",
    ///      "type": "string"
    ///    },
    ///    "si_identifier": {
    ///      "type": "string"
    ///    },
    ///    "si_identifier_type": {
    ///      "description": "Can be 0002, 0007, 0009, 0037, 0060, 0088, 0096, 0097, 0106, 0130, 0135, 0142, 0151, 0183, 0184, 0190, 0191, 0192, 0193, 0195, 0196, 0198, 0199, 0200, 0201, 0202, 0204, 0208, 0209, 9901, 9902, 9904, 9905, 9906, 9907, 9908, 9909, 9910, 9912, 9913, 9914, 9915, 9917, 9918, 9919, 9920, 9921, 9922, 9923, 9924, 9925, 9926, 9927, 9928, 9929, 9930, 9931, 9932, 9933, 9934, 9935, 9936, 9937, 9938, 9939, 9940, 9941, 9942, 9943, 9944, 9945, 9946, 9947, 9948, 9949, 9950, 9951, 9952, 9953, 9954, 9955, 9956, 9957 or 9958.",
    ///      "type": "string"
    ///    },
    ///    "tax_number": {
    ///      "type": "string"
    ///    },
    ///    "zipcode": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Contact {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub address1: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub address2: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bank_account: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub chamber_of_commerce: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub city: ::std::option::Option<::std::string::String>,
        ///A contact requires a non-blank company_name, firstname or lastname.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub company_name: ::std::option::Option<::std::string::String>,
        ///ISO two-character country code, e.g. NL or DE.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub country: ::std::option::Option<::std::string::String>,
        ///Will be assigned automatically if empty. Should be unique for the administration.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub customer_id: ::std::option::Option<::std::string::String>,
        ///Can be Email, Simplerinvoicing, Post or Manual.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub delivery_method: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub direct_debit: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email_ubl: ::std::option::Option<bool>,
        ///Should be a valid estimate workflow id.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub estimate_workflow_id: ::std::option::Option<::std::string::String>,
        ///A contact requires a non-blank company_name, firstname or lastname.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub firstname: ::std::option::Option<::std::string::String>,
        ///The ID of the contact.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        ///Should be a valid invoice workflow id.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub invoice_workflow_id: ::std::option::Option<::std::string::String>,
        ///A contact requires a non-blank company_name, firstname or lastname.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lastname: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub phone: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_estimates_to_attention: ::std::option::Option<::std::string::String>,
        ///Should be one or more valid email addresses, separated by a comma.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_estimates_to_email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_invoices_to_attention: ::std::option::Option<::std::string::String>,
        ///Should be one or more valid email addresses, separated by a comma.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_invoices_to_email: ::std::option::Option<::std::string::String>,
        ///When true, all other SEPA fields are required.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_active: ::std::option::Option<bool>,
        ///Should be a valid BIC.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_bic: ::std::option::Option<::std::string::String>,
        ///Should be a valid IBAN.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_iban: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_iban_account_name: ::std::option::Option<::std::string::String>,
        ///Should be a date in the past.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_mandate_date: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_mandate_id: ::std::option::Option<::std::string::String>,
        ///Can be RCUR, FRST, OOFF or FNAL.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_sequence_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub si_identifier: ::std::option::Option<::std::string::String>,
        ///Can be 0002, 0007, 0009, 0037, 0060, 0088, 0096, 0097, 0106, 0130, 0135, 0142, 0151, 0183, 0184, 0190, 0191, 0192, 0193, 0195, 0196, 0198, 0199, 0200, 0201, 0202, 0204, 0208, 0209, 9901, 9902, 9904, 9905, 9906, 9907, 9908, 9909, 9910, 9912, 9913, 9914, 9915, 9917, 9918, 9919, 9920, 9921, 9922, 9923, 9924, 9925, 9926, 9927, 9928, 9929, 9930, 9931, 9932, 9933, 9934, 9935, 9936, 9937, 9938, 9939, 9940, 9941, 9942, 9943, 9944, 9945, 9946, 9947, 9948, 9949, 9950, 9951, 9952, 9953, 9954, 9955, 9956, 9957 or 9958.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub si_identifier_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tax_number: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub zipcode: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&Contact> for Contact {
        fn from(value: &Contact) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Contact {
        fn default() -> Self {
            Self {
                address1: Default::default(),
                address2: Default::default(),
                bank_account: Default::default(),
                chamber_of_commerce: Default::default(),
                city: Default::default(),
                company_name: Default::default(),
                country: Default::default(),
                customer_id: Default::default(),
                delivery_method: Default::default(),
                direct_debit: Default::default(),
                email_ubl: Default::default(),
                estimate_workflow_id: Default::default(),
                firstname: Default::default(),
                id: Default::default(),
                invoice_workflow_id: Default::default(),
                lastname: Default::default(),
                phone: Default::default(),
                send_estimates_to_attention: Default::default(),
                send_estimates_to_email: Default::default(),
                send_invoices_to_attention: Default::default(),
                send_invoices_to_email: Default::default(),
                sepa_active: Default::default(),
                sepa_bic: Default::default(),
                sepa_iban: Default::default(),
                sepa_iban_account_name: Default::default(),
                sepa_mandate_date: Default::default(),
                sepa_mandate_id: Default::default(),
                sepa_sequence_type: Default::default(),
                si_identifier: Default::default(),
                si_identifier_type: Default::default(),
                tax_number: Default::default(),
                zipcode: Default::default(),
            }
        }
    }
    impl Contact {
        pub fn builder() -> builder::Contact {
            Default::default()
        }
    }
    ///Object for when creating contacts.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Object for when creating contacts.",
    ///  "examples": [
    ///    {
    ///      "address1": "Test street 1",
    ///      "address2": "",
    ///      "bank_account": "NL90INGB0001234657",
    ///      "chamber_of_commerce": "",
    ///      "city": "",
    ///      "company_name": "Test company",
    ///      "country": "NL",
    ///      "custom_fields_attributes": [
    ///        {
    ///          "id": 12345678901234,
    ///          "value": "testing"
    ///        }
    ///      ],
    ///      "customer_id": "6",
    ///      "delivery_method": "Email",
    ///      "direct_debit": false,
    ///      "email_ubl": true,
    ///      "firstname": "Dennis",
    ///      "lastname": "DEMO",
    ///      "phone": "",
    ///      "send_estimates_to_attention": "",
    ///      "send_estimates_to_email": "example@example.com",
    ///      "send_invoices_to_attention": "",
    ///      "send_invoices_to_email": "example@example.com",
    ///      "sepa_active": false,
    ///      "sepa_bic": "INGBNL2A",
    ///      "sepa_iban": "NL90INGB0001234567",
    ///      "sepa_iban_account_name": "Dennis DEMO",
    ///      "sepa_mandate_date": "2020-09-30",
    ///      "sepa_mandate_id": "DEMO123",
    ///      "sepa_sequence_type": "RCUR",
    ///      "si_identifier": "",
    ///      "tax_number": "",
    ///      "zipcode": "1234 AB"
    ///    }
    ///  ],
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/ContactUpdate"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "contact_person": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "object",
    ///            "properties": {
    ///              "firstname": {
    ///                "type": "string"
    ///              },
    ///              "lastname": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          }
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ContactCreate {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub address1: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub address2: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bank_account: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub chamber_of_commerce: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub city: ::std::option::Option<::std::string::String>,
        ///A contact requires a non-blank company_name, firstname or lastname.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub company_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub contact_person: ::std::vec::Vec<ContactCreateContactPersonItem>,
        ///ISO two-character country code, e.g. NL or DE.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub country: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub custom_fields_attributes: ::std::vec::Vec<ContactCreateCustomFieldsAttributesItem>,
        ///Will be assigned automatically if empty. Should be unique for the administration.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub customer_id: ::std::option::Option<::std::string::String>,
        ///Can be Email, Simplerinvoicing, Post or Manual.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub delivery_method: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub direct_debit: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email_ubl: ::std::option::Option<bool>,
        ///Should be a valid estimate workflow id.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub estimate_workflow_id: ::std::option::Option<::std::string::String>,
        ///A contact requires a non-blank company_name, firstname or lastname.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub firstname: ::std::option::Option<::std::string::String>,
        ///The ID of the contact.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        ///Should be a valid invoice workflow id.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub invoice_workflow_id: ::std::option::Option<::std::string::String>,
        ///A contact requires a non-blank company_name, firstname or lastname.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lastname: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub phone: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_estimates_to_attention: ::std::option::Option<::std::string::String>,
        ///Should be one or more valid email addresses, separated by a comma.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_estimates_to_email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_invoices_to_attention: ::std::option::Option<::std::string::String>,
        ///Should be one or more valid email addresses, separated by a comma.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_invoices_to_email: ::std::option::Option<::std::string::String>,
        ///When true, all other SEPA fields are required.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_active: ::std::option::Option<bool>,
        ///Should be a valid BIC.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_bic: ::std::option::Option<::std::string::String>,
        ///Should be a valid IBAN.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_iban: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_iban_account_name: ::std::option::Option<::std::string::String>,
        ///Should be a date in the past.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_mandate_date: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_mandate_id: ::std::option::Option<::std::string::String>,
        ///Can be RCUR, FRST, OOFF or FNAL.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_sequence_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub si_identifier: ::std::option::Option<::std::string::String>,
        ///Can be 0002, 0007, 0009, 0037, 0060, 0088, 0096, 0097, 0106, 0130, 0135, 0142, 0151, 0183, 0184, 0190, 0191, 0192, 0193, 0195, 0196, 0198, 0199, 0200, 0201, 0202, 0204, 0208, 0209, 9901, 9902, 9904, 9905, 9906, 9907, 9908, 9909, 9910, 9912, 9913, 9914, 9915, 9917, 9918, 9919, 9920, 9921, 9922, 9923, 9924, 9925, 9926, 9927, 9928, 9929, 9930, 9931, 9932, 9933, 9934, 9935, 9936, 9937, 9938, 9939, 9940, 9941, 9942, 9943, 9944, 9945, 9946, 9947, 9948, 9949, 9950, 9951, 9952, 9953, 9954, 9955, 9956, 9957 or 9958.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub si_identifier_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tax_number: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub zipcode: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ContactCreate> for ContactCreate {
        fn from(value: &ContactCreate) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ContactCreate {
        fn default() -> Self {
            Self {
                address1: Default::default(),
                address2: Default::default(),
                bank_account: Default::default(),
                chamber_of_commerce: Default::default(),
                city: Default::default(),
                company_name: Default::default(),
                contact_person: Default::default(),
                country: Default::default(),
                custom_fields_attributes: Default::default(),
                customer_id: Default::default(),
                delivery_method: Default::default(),
                direct_debit: Default::default(),
                email_ubl: Default::default(),
                estimate_workflow_id: Default::default(),
                firstname: Default::default(),
                id: Default::default(),
                invoice_workflow_id: Default::default(),
                lastname: Default::default(),
                phone: Default::default(),
                send_estimates_to_attention: Default::default(),
                send_estimates_to_email: Default::default(),
                send_invoices_to_attention: Default::default(),
                send_invoices_to_email: Default::default(),
                sepa_active: Default::default(),
                sepa_bic: Default::default(),
                sepa_iban: Default::default(),
                sepa_iban_account_name: Default::default(),
                sepa_mandate_date: Default::default(),
                sepa_mandate_id: Default::default(),
                sepa_sequence_type: Default::default(),
                si_identifier: Default::default(),
                si_identifier_type: Default::default(),
                tax_number: Default::default(),
                zipcode: Default::default(),
            }
        }
    }
    impl ContactCreate {
        pub fn builder() -> builder::ContactCreate {
            Default::default()
        }
    }
    ///ContactCreateContactPersonItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "firstname": {
    ///      "type": "string"
    ///    },
    ///    "lastname": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ContactCreateContactPersonItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub firstname: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lastname: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ContactCreateContactPersonItem> for ContactCreateContactPersonItem {
        fn from(value: &ContactCreateContactPersonItem) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ContactCreateContactPersonItem {
        fn default() -> Self {
            Self {
                firstname: Default::default(),
                lastname: Default::default(),
            }
        }
    }
    impl ContactCreateContactPersonItem {
        pub fn builder() -> builder::ContactCreateContactPersonItem {
            Default::default()
        }
    }
    ///ContactCreateCustomFieldsAttributesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "type": "integer"
    ///    },
    ///    "value": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ContactCreateCustomFieldsAttributesItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ContactCreateCustomFieldsAttributesItem>
        for ContactCreateCustomFieldsAttributesItem
    {
        fn from(value: &ContactCreateCustomFieldsAttributesItem) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ContactCreateCustomFieldsAttributesItem {
        fn default() -> Self {
            Self {
                id: Default::default(),
                value: Default::default(),
            }
        }
    }
    impl ContactCreateCustomFieldsAttributesItem {
        pub fn builder() -> builder::ContactCreateCustomFieldsAttributesItem {
            Default::default()
        }
    }
    ///Object for when reading contacts.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Object for when reading contacts.",
    ///  "examples": [
    ///    {
    ///      "address1": "Hoofdstraat 12",
    ///      "address2": "",
    ///      "administration_id": "412839047230990",
    ///      "archived": false,
    ///      "attention": "",
    ///      "bank_account": "",
    ///      "chamber_of_commerce": "",
    ///      "city": "Amsterdam",
    ///      "company_name": "Foobar Holding B.V.",
    ///      "contact_people": [
    ///        {
    ///          "administration_id": "412839047230990",
    ///          "created_at": "2023-05-09T09:26:35.840Z",
    ///          "department": null,
    ///          "email": null,
    ///          "firstname": "John",
    ///          "id": 387348560997254835,
    ///          "lastname": "Appleseed",
    ///          "phone": null,
    ///          "updated_at": "2023-05-09T09:26:35.840Z",
    ///          "version": 1683624395
    ///        }
    ///      ],
    ///      "country": "NL",
    ///      "created_at": "2023-05-09T09:26:35.832Z",
    ///      "credit_card_number": "",
    ///      "credit_card_reference": "",
    ///      "credit_card_type": null,
    ///      "custom_fields": [],
    ///      "customer_id": "1",
    ///      "delivery_method": "Email",
    ///      "email": "info@example.com",
    ///      "email_ubl": true,
    ///      "estimate_workflow_id": null,
    ///      "events": [
    ///        {
    ///          "action": "contact_created",
    ///          "administration_id": "412839047230990",
    ///          "created_at": "2023-05-09T09:26:35.847Z",
    ///          "data": {},
    ///          "link_entity_id": null,
    ///          "link_entity_type": null,
    ///          "updated_at": "2023-05-09T09:26:35.847Z",
    ///          "user_id": 16836243678565
    ///        }
    ///      ],
    ///      "firstname": null,
    ///      "id": "387348560989914801",
    ///      "invoice_workflow_id": null,
    ///      "lastname": "Appleseed",
    ///      "moneybird_payments_mandate": false,
    ///      "notes": [],
    ///      "phone": "",
    ///      "sales_invoices_url": "https://moneybird.dev/123/sales_invoices/2d977c4c1ec0f1e7feff0df2b0411a3a4960e29a5bc785bfe30dd974ca9a2e10/all",
    ///      "send_estimates_to_attention": "",
    ///      "send_estimates_to_email": "info@example.com",
    ///      "send_invoices_to_attention": "",
    ///      "send_invoices_to_email": "info@example.com",
    ///      "sepa_active": false,
    ///      "sepa_bic": "",
    ///      "sepa_iban": "",
    ///      "sepa_iban_account_name": "",
    ///      "sepa_mandate_date": null,
    ///      "sepa_mandate_id": "",
    ///      "sepa_sequence_type": "RCUR",
    ///      "si_identifier": "",
    ///      "si_identifier_type": null,
    ///      "tax_number": "",
    ///      "tax_number_valid": null,
    ///      "tax_number_validated_at": null,
    ///      "updated_at": "2023-05-09T09:26:35.852Z",
    ///      "version": 1683624395,
    ///      "zipcode": "1234 AB"
    ///    }
    ///  ],
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Contact"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "administration_id": {
    ///          "type": "string"
    ///        },
    ///        "archived": {
    ///          "type": "boolean"
    ///        },
    ///        "contact_people": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "object",
    ///            "properties": {
    ///              "administration_id": {
    ///                "type": "string"
    ///              },
    ///              "created_at": {
    ///                "type": "string"
    ///              },
    ///              "department": {
    ///                "type": "string"
    ///              },
    ///              "email": {
    ///                "type": "string"
    ///              },
    ///              "firstname": {
    ///                "type": "string"
    ///              },
    ///              "id": {
    ///                "type": "string"
    ///              },
    ///              "lastname": {
    ///                "type": "string"
    ///              },
    ///              "phone": {
    ///                "type": "string"
    ///              },
    ///              "updated_at": {
    ///                "type": "string"
    ///              },
    ///              "version": {
    ///                "type": "integer"
    ///              }
    ///            }
    ///          }
    ///        },
    ///        "custom_fields": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/CustomField"
    ///          }
    ///        },
    ///        "events": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/Event"
    ///          }
    ///        },
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "notes": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/Note"
    ///          }
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ContactRead {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub address1: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub address2: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub administration_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub archived: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bank_account: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub chamber_of_commerce: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub city: ::std::option::Option<::std::string::String>,
        ///A contact requires a non-blank company_name, firstname or lastname.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub company_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub contact_people: ::std::vec::Vec<ContactReadContactPeopleItem>,
        ///ISO two-character country code, e.g. NL or DE.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub country: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub custom_fields: ::std::vec::Vec<CustomField>,
        ///Will be assigned automatically if empty. Should be unique for the administration.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub customer_id: ::std::option::Option<::std::string::String>,
        ///Can be Email, Simplerinvoicing, Post or Manual.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub delivery_method: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub direct_debit: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email_ubl: ::std::option::Option<bool>,
        ///Should be a valid estimate workflow id.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub estimate_workflow_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub events: ::std::vec::Vec<Event>,
        ///A contact requires a non-blank company_name, firstname or lastname.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub firstname: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        ///Should be a valid invoice workflow id.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub invoice_workflow_id: ::std::option::Option<::std::string::String>,
        ///A contact requires a non-blank company_name, firstname or lastname.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lastname: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub notes: ::std::vec::Vec<Note>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub phone: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_estimates_to_attention: ::std::option::Option<::std::string::String>,
        ///Should be one or more valid email addresses, separated by a comma.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_estimates_to_email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_invoices_to_attention: ::std::option::Option<::std::string::String>,
        ///Should be one or more valid email addresses, separated by a comma.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_invoices_to_email: ::std::option::Option<::std::string::String>,
        ///When true, all other SEPA fields are required.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_active: ::std::option::Option<bool>,
        ///Should be a valid BIC.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_bic: ::std::option::Option<::std::string::String>,
        ///Should be a valid IBAN.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_iban: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_iban_account_name: ::std::option::Option<::std::string::String>,
        ///Should be a date in the past.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_mandate_date: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_mandate_id: ::std::option::Option<::std::string::String>,
        ///Can be RCUR, FRST, OOFF or FNAL.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_sequence_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub si_identifier: ::std::option::Option<::std::string::String>,
        ///Can be 0002, 0007, 0009, 0037, 0060, 0088, 0096, 0097, 0106, 0130, 0135, 0142, 0151, 0183, 0184, 0190, 0191, 0192, 0193, 0195, 0196, 0198, 0199, 0200, 0201, 0202, 0204, 0208, 0209, 9901, 9902, 9904, 9905, 9906, 9907, 9908, 9909, 9910, 9912, 9913, 9914, 9915, 9917, 9918, 9919, 9920, 9921, 9922, 9923, 9924, 9925, 9926, 9927, 9928, 9929, 9930, 9931, 9932, 9933, 9934, 9935, 9936, 9937, 9938, 9939, 9940, 9941, 9942, 9943, 9944, 9945, 9946, 9947, 9948, 9949, 9950, 9951, 9952, 9953, 9954, 9955, 9956, 9957 or 9958.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub si_identifier_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tax_number: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub zipcode: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ContactRead> for ContactRead {
        fn from(value: &ContactRead) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ContactRead {
        fn default() -> Self {
            Self {
                address1: Default::default(),
                address2: Default::default(),
                administration_id: Default::default(),
                archived: Default::default(),
                bank_account: Default::default(),
                chamber_of_commerce: Default::default(),
                city: Default::default(),
                company_name: Default::default(),
                contact_people: Default::default(),
                country: Default::default(),
                custom_fields: Default::default(),
                customer_id: Default::default(),
                delivery_method: Default::default(),
                direct_debit: Default::default(),
                email_ubl: Default::default(),
                estimate_workflow_id: Default::default(),
                events: Default::default(),
                firstname: Default::default(),
                id: Default::default(),
                invoice_workflow_id: Default::default(),
                lastname: Default::default(),
                notes: Default::default(),
                phone: Default::default(),
                send_estimates_to_attention: Default::default(),
                send_estimates_to_email: Default::default(),
                send_invoices_to_attention: Default::default(),
                send_invoices_to_email: Default::default(),
                sepa_active: Default::default(),
                sepa_bic: Default::default(),
                sepa_iban: Default::default(),
                sepa_iban_account_name: Default::default(),
                sepa_mandate_date: Default::default(),
                sepa_mandate_id: Default::default(),
                sepa_sequence_type: Default::default(),
                si_identifier: Default::default(),
                si_identifier_type: Default::default(),
                tax_number: Default::default(),
                zipcode: Default::default(),
            }
        }
    }
    impl ContactRead {
        pub fn builder() -> builder::ContactRead {
            Default::default()
        }
    }
    ///ContactReadContactPeopleItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "administration_id": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "department": {
    ///      "type": "string"
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "firstname": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "lastname": {
    ///      "type": "string"
    ///    },
    ///    "phone": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    },
    ///    "version": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ContactReadContactPeopleItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub administration_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub department: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub firstname: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lastname: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub phone: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version: ::std::option::Option<i64>,
    }
    impl ::std::convert::From<&ContactReadContactPeopleItem> for ContactReadContactPeopleItem {
        fn from(value: &ContactReadContactPeopleItem) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ContactReadContactPeopleItem {
        fn default() -> Self {
            Self {
                administration_id: Default::default(),
                created_at: Default::default(),
                department: Default::default(),
                email: Default::default(),
                firstname: Default::default(),
                id: Default::default(),
                lastname: Default::default(),
                phone: Default::default(),
                updated_at: Default::default(),
                version: Default::default(),
            }
        }
    }
    impl ContactReadContactPeopleItem {
        pub fn builder() -> builder::ContactReadContactPeopleItem {
            Default::default()
        }
    }
    ///Object for when updating contacts.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Object for when updating contacts.",
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Contact"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "custom_fields_attributes": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "object",
    ///            "properties": {
    ///              "id": {
    ///                "type": "integer"
    ///              },
    ///              "value": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          }
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ContactUpdate {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub address1: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub address2: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bank_account: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub chamber_of_commerce: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub city: ::std::option::Option<::std::string::String>,
        ///A contact requires a non-blank company_name, firstname or lastname.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub company_name: ::std::option::Option<::std::string::String>,
        ///ISO two-character country code, e.g. NL or DE.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub country: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub custom_fields_attributes: ::std::vec::Vec<ContactUpdateCustomFieldsAttributesItem>,
        ///Will be assigned automatically if empty. Should be unique for the administration.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub customer_id: ::std::option::Option<::std::string::String>,
        ///Can be Email, Simplerinvoicing, Post or Manual.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub delivery_method: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub direct_debit: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email_ubl: ::std::option::Option<bool>,
        ///Should be a valid estimate workflow id.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub estimate_workflow_id: ::std::option::Option<::std::string::String>,
        ///A contact requires a non-blank company_name, firstname or lastname.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub firstname: ::std::option::Option<::std::string::String>,
        ///The ID of the contact.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        ///Should be a valid invoice workflow id.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub invoice_workflow_id: ::std::option::Option<::std::string::String>,
        ///A contact requires a non-blank company_name, firstname or lastname.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lastname: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub phone: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_estimates_to_attention: ::std::option::Option<::std::string::String>,
        ///Should be one or more valid email addresses, separated by a comma.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_estimates_to_email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_invoices_to_attention: ::std::option::Option<::std::string::String>,
        ///Should be one or more valid email addresses, separated by a comma.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub send_invoices_to_email: ::std::option::Option<::std::string::String>,
        ///When true, all other SEPA fields are required.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_active: ::std::option::Option<bool>,
        ///Should be a valid BIC.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_bic: ::std::option::Option<::std::string::String>,
        ///Should be a valid IBAN.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_iban: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_iban_account_name: ::std::option::Option<::std::string::String>,
        ///Should be a date in the past.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_mandate_date: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_mandate_id: ::std::option::Option<::std::string::String>,
        ///Can be RCUR, FRST, OOFF or FNAL.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sepa_sequence_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub si_identifier: ::std::option::Option<::std::string::String>,
        ///Can be 0002, 0007, 0009, 0037, 0060, 0088, 0096, 0097, 0106, 0130, 0135, 0142, 0151, 0183, 0184, 0190, 0191, 0192, 0193, 0195, 0196, 0198, 0199, 0200, 0201, 0202, 0204, 0208, 0209, 9901, 9902, 9904, 9905, 9906, 9907, 9908, 9909, 9910, 9912, 9913, 9914, 9915, 9917, 9918, 9919, 9920, 9921, 9922, 9923, 9924, 9925, 9926, 9927, 9928, 9929, 9930, 9931, 9932, 9933, 9934, 9935, 9936, 9937, 9938, 9939, 9940, 9941, 9942, 9943, 9944, 9945, 9946, 9947, 9948, 9949, 9950, 9951, 9952, 9953, 9954, 9955, 9956, 9957 or 9958.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub si_identifier_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tax_number: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub zipcode: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ContactUpdate> for ContactUpdate {
        fn from(value: &ContactUpdate) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ContactUpdate {
        fn default() -> Self {
            Self {
                address1: Default::default(),
                address2: Default::default(),
                bank_account: Default::default(),
                chamber_of_commerce: Default::default(),
                city: Default::default(),
                company_name: Default::default(),
                country: Default::default(),
                custom_fields_attributes: Default::default(),
                customer_id: Default::default(),
                delivery_method: Default::default(),
                direct_debit: Default::default(),
                email_ubl: Default::default(),
                estimate_workflow_id: Default::default(),
                firstname: Default::default(),
                id: Default::default(),
                invoice_workflow_id: Default::default(),
                lastname: Default::default(),
                phone: Default::default(),
                send_estimates_to_attention: Default::default(),
                send_estimates_to_email: Default::default(),
                send_invoices_to_attention: Default::default(),
                send_invoices_to_email: Default::default(),
                sepa_active: Default::default(),
                sepa_bic: Default::default(),
                sepa_iban: Default::default(),
                sepa_iban_account_name: Default::default(),
                sepa_mandate_date: Default::default(),
                sepa_mandate_id: Default::default(),
                sepa_sequence_type: Default::default(),
                si_identifier: Default::default(),
                si_identifier_type: Default::default(),
                tax_number: Default::default(),
                zipcode: Default::default(),
            }
        }
    }
    impl ContactUpdate {
        pub fn builder() -> builder::ContactUpdate {
            Default::default()
        }
    }
    ///ContactUpdateCustomFieldsAttributesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "type": "integer"
    ///    },
    ///    "value": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ContactUpdateCustomFieldsAttributesItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ContactUpdateCustomFieldsAttributesItem>
        for ContactUpdateCustomFieldsAttributesItem
    {
        fn from(value: &ContactUpdateCustomFieldsAttributesItem) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ContactUpdateCustomFieldsAttributesItem {
        fn default() -> Self {
            Self {
                id: Default::default(),
                value: Default::default(),
            }
        }
    }
    impl ContactUpdateCustomFieldsAttributesItem {
        pub fn builder() -> builder::ContactUpdateCustomFieldsAttributesItem {
            Default::default()
        }
    }
    ///CustomField
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "administration_id": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "source": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CustomField {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub administration_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&CustomField> for CustomField {
        fn from(value: &CustomField) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for CustomField {
        fn default() -> Self {
            Self {
                administration_id: Default::default(),
                id: Default::default(),
                name: Default::default(),
                source: Default::default(),
            }
        }
    }
    impl CustomField {
        pub fn builder() -> builder::CustomField {
            Default::default()
        }
    }
    ///Event
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "action": {
    ///      "type": "string"
    ///    },
    ///    "administration_id": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "data": {
    ///      "type": "object"
    ///    },
    ///    "link_entity_id": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "link_entity_type": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    },
    ///    "user_id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Event {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub action: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub administration_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub data: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub link_entity_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub link_entity_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&Event> for Event {
        fn from(value: &Event) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Event {
        fn default() -> Self {
            Self {
                action: Default::default(),
                administration_id: Default::default(),
                created_at: Default::default(),
                data: Default::default(),
                link_entity_id: Default::default(),
                link_entity_type: Default::default(),
                updated_at: Default::default(),
                user_id: Default::default(),
            }
        }
    }
    impl Event {
        pub fn builder() -> builder::Event {
            Default::default()
        }
    }
    ///GetProjectsFilter
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "state:all",
    ///    "state:archived",
    ///    "state:active"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetProjectsFilter {
        #[serde(rename = "state:all")]
        StateAll,
        #[serde(rename = "state:archived")]
        StateArchived,
        #[serde(rename = "state:active")]
        StateActive,
    }
    impl ::std::convert::From<&Self> for GetProjectsFilter {
        fn from(value: &GetProjectsFilter) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetProjectsFilter {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::StateAll => write!(f, "state:all"),
                Self::StateArchived => write!(f, "state:archived"),
                Self::StateActive => write!(f, "state:active"),
            }
        }
    }
    impl ::std::str::FromStr for GetProjectsFilter {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "state:all" => Ok(Self::StateAll),
                "state:archived" => Ok(Self::StateArchived),
                "state:active" => Ok(Self::StateActive),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetProjectsFilter {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetProjectsFilter {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetProjectsFilter {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Note
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "administration_id": {
    ///      "type": "string"
    ///    },
    ///    "assignee_id": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "completed_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "completed_by_id": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "data": {
    ///      "type": "object"
    ///    },
    ///    "entity_id": {
    ///      "type": "string"
    ///    },
    ///    "entity_type": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "note": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "todo": {
    ///      "type": "boolean"
    ///    },
    ///    "todo_type": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    },
    ///    "user_id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Note {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub administration_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub assignee_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub completed_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub completed_by_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub data: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub entity_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub entity_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub note: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub todo: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub todo_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&Note> for Note {
        fn from(value: &Note) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Note {
        fn default() -> Self {
            Self {
                administration_id: Default::default(),
                assignee_id: Default::default(),
                completed_at: Default::default(),
                completed_by_id: Default::default(),
                created_at: Default::default(),
                data: Default::default(),
                entity_id: Default::default(),
                entity_type: Default::default(),
                id: Default::default(),
                note: Default::default(),
                todo: Default::default(),
                todo_type: Default::default(),
                updated_at: Default::default(),
                user_id: Default::default(),
            }
        }
    }
    impl Note {
        pub fn builder() -> builder::Note {
            Default::default()
        }
    }
    ///Object which describes a project
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Object which describes a project",
    ///  "type": "object",
    ///  "properties": {
    ///    "budget": {
    ///      "type": "number"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "state": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Project {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub budget: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&Project> for Project {
        fn from(value: &Project) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Project {
        fn default() -> Self {
            Self {
                budget: Default::default(),
                id: Default::default(),
                name: Default::default(),
                state: Default::default(),
            }
        }
    }
    impl Project {
        pub fn builder() -> builder::Project {
            Default::default()
        }
    }
    ///Object for creating a project
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Object for creating a project",
    ///  "type": "object",
    ///  "required": [
    ///    "project"
    ///  ],
    ///  "properties": {
    ///    "project": {
    ///      "type": "object",
    ///      "required": [
    ///        "budget",
    ///        "name"
    ///      ],
    ///      "properties": {
    ///        "budget": {
    ///          "type": "number"
    ///        },
    ///        "name": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ProjectCreate {
        pub project: ProjectCreateProject,
    }
    impl ::std::convert::From<&ProjectCreate> for ProjectCreate {
        fn from(value: &ProjectCreate) -> Self {
            value.clone()
        }
    }
    impl ProjectCreate {
        pub fn builder() -> builder::ProjectCreate {
            Default::default()
        }
    }
    ///ProjectCreateProject
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "budget",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "budget": {
    ///      "type": "number"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ProjectCreateProject {
        pub budget: f64,
        pub name: ::std::string::String,
    }
    impl ::std::convert::From<&ProjectCreateProject> for ProjectCreateProject {
        fn from(value: &ProjectCreateProject) -> Self {
            value.clone()
        }
    }
    impl ProjectCreateProject {
        pub fn builder() -> builder::ProjectCreateProject {
            Default::default()
        }
    }
    ///ProjectRead
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Object for when reading projects",
    ///  "examples": [
    ///    {
    ///      "budget": 10,
    ///      "id": "446241767953532430",
    ///      "name": "Project Blackbird",
    ///      "state": "archived"
    ///    }
    ///  ],
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Project"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "budget": {
    ///          "type": "number"
    ///        },
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "name": {
    ///          "type": "string"
    ///        },
    ///        "state": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct ProjectRead(pub Project);
    impl ::std::ops::Deref for ProjectRead {
        type Target = Project;
        fn deref(&self) -> &Project {
            &self.0
        }
    }
    impl ::std::convert::From<ProjectRead> for Project {
        fn from(value: ProjectRead) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ProjectRead> for ProjectRead {
        fn from(value: &ProjectRead) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<Project> for ProjectRead {
        fn from(value: Project) -> Self {
            Self(value)
        }
    }
    ///Object for when updating projects.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Object for when updating projects.",
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Project"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct ProjectUpdate(pub Project);
    impl ::std::ops::Deref for ProjectUpdate {
        type Target = Project;
        fn deref(&self) -> &Project {
            &self.0
        }
    }
    impl ::std::convert::From<ProjectUpdate> for Project {
        fn from(value: ProjectUpdate) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ProjectUpdate> for ProjectUpdate {
        fn from(value: &ProjectUpdate) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<Project> for ProjectUpdate {
        fn from(value: Project) -> Self {
            Self(value)
        }
    }
    ///Object which describes a time entry
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Object which describes a time entry",
    ///  "examples": [
    ///    {
    ///      "administration_id": 123,
    ///      "billable": true,
    ///      "contact": {
    ///        "company_name": "Foobar Holding B.V.",
    ///        "firstname": "",
    ///        "id": "446241830553519137",
    ///        "lastname": ""
    ///      },
    ///      "contact_id": "446241830553519137",
    ///      "created_at": "2025-02-17T10:49:54.448Z",
    ///      "description": "Test",
    ///      "detail": null,
    ///      "ended_at": "2025-02-17T11:49:00.000Z",
    ///      "events": [
    ///        {
    ///          "action": "time_entry_created",
    ///          "administration_id": 123,
    ///          "created_at": "2025-02-17T10:49:54.450Z",
    ///          "data": {},
    ///          "link_entity_id": null,
    ///          "link_entity_type": null,
    ///          "updated_at": "2025-02-17T10:49:54.450Z",
    ///          "user_id": 17397890963220
    ///        }
    ///      ],
    ///      "id": "446241830577636389",
    ///      "notes": [],
    ///      "paused_duration": 0,
    ///      "project": {
    ///        "budget": 10,
    ///        "id": "446241830564004899",
    ///        "name": "My project name",
    ///        "state": "active"
    ///      },
    ///      "project_id": "446241830564004899",
    ///      "started_at": "2025-02-17T10:49:00.000Z",
    ///      "updated_at": "2025-02-17T10:49:54.448Z",
    ///      "user": {
    ///        "created_at": "2025-02-17T10:44:56.885Z",
    ///        "id": 17397890963220,
    ///        "name": "Moneybird",
    ///        "updated_at": "2025-02-17T10:44:57.120Z"
    ///      },
    ///      "user_id": 17397890963220
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "properties": {
    ///    "administration_id": {
    ///      "type": "string"
    ///    },
    ///    "billable": {
    ///      "type": "boolean"
    ///    },
    ///    "contact": {
    ///      "$ref": "#/components/schemas/Contact"
    ///    },
    ///    "contact_id": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "ended_at": {
    ///      "type": "string"
    ///    },
    ///    "events": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Event"
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "notes": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Note"
    ///      }
    ///    },
    ///    "paused_duration": {
    ///      "type": "number"
    ///    },
    ///    "project": {
    ///      "$ref": "#/components/schemas/Project"
    ///    },
    ///    "project_id": {
    ///      "type": "string"
    ///    },
    ///    "started_at": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    },
    ///    "user_id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TimeEntry {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub administration_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub billable: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub contact: ::std::option::Option<Contact>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub contact_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ended_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub events: ::std::vec::Vec<Event>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub notes: ::std::vec::Vec<Note>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub paused_duration: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub project: ::std::option::Option<Project>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub started_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&TimeEntry> for TimeEntry {
        fn from(value: &TimeEntry) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for TimeEntry {
        fn default() -> Self {
            Self {
                administration_id: Default::default(),
                billable: Default::default(),
                contact: Default::default(),
                contact_id: Default::default(),
                created_at: Default::default(),
                description: Default::default(),
                ended_at: Default::default(),
                events: Default::default(),
                id: Default::default(),
                notes: Default::default(),
                paused_duration: Default::default(),
                project: Default::default(),
                project_id: Default::default(),
                started_at: Default::default(),
                updated_at: Default::default(),
                user_id: Default::default(),
            }
        }
    }
    impl TimeEntry {
        pub fn builder() -> builder::TimeEntry {
            Default::default()
        }
    }
    ///Object for creating a time entry
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Object for creating a time entry",
    ///  "type": "object",
    ///  "required": [
    ///    "time_entry"
    ///  ],
    ///  "properties": {
    ///    "time_entry": {
    ///      "type": "object",
    ///      "required": [
    ///        "description",
    ///        "ended_at",
    ///        "started_at",
    ///        "user_id"
    ///      ],
    ///      "properties": {
    ///        "billable": {
    ///          "type": "boolean"
    ///        },
    ///        "contact_id": {
    ///          "type": "string"
    ///        },
    ///        "description": {
    ///          "type": "string"
    ///        },
    ///        "detail_id": {
    ///          "type": "string"
    ///        },
    ///        "ended_at": {
    ///          "type": "string"
    ///        },
    ///        "paused_duration": {
    ///          "type": "number"
    ///        },
    ///        "project_id": {
    ///          "type": "string"
    ///        },
    ///        "started_at": {
    ///          "type": "string"
    ///        },
    ///        "user_id": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TimeEntryCreate {
        pub time_entry: TimeEntryCreateTimeEntry,
    }
    impl ::std::convert::From<&TimeEntryCreate> for TimeEntryCreate {
        fn from(value: &TimeEntryCreate) -> Self {
            value.clone()
        }
    }
    impl TimeEntryCreate {
        pub fn builder() -> builder::TimeEntryCreate {
            Default::default()
        }
    }
    ///TimeEntryCreateTimeEntry
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "ended_at",
    ///    "started_at",
    ///    "user_id"
    ///  ],
    ///  "properties": {
    ///    "billable": {
    ///      "type": "boolean"
    ///    },
    ///    "contact_id": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "detail_id": {
    ///      "type": "string"
    ///    },
    ///    "ended_at": {
    ///      "type": "string"
    ///    },
    ///    "paused_duration": {
    ///      "type": "number"
    ///    },
    ///    "project_id": {
    ///      "type": "string"
    ///    },
    ///    "started_at": {
    ///      "type": "string"
    ///    },
    ///    "user_id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TimeEntryCreateTimeEntry {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub billable: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub contact_id: ::std::option::Option<::std::string::String>,
        pub description: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub detail_id: ::std::option::Option<::std::string::String>,
        pub ended_at: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub paused_duration: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub project_id: ::std::option::Option<::std::string::String>,
        pub started_at: ::std::string::String,
        pub user_id: ::std::string::String,
    }
    impl ::std::convert::From<&TimeEntryCreateTimeEntry> for TimeEntryCreateTimeEntry {
        fn from(value: &TimeEntryCreateTimeEntry) -> Self {
            value.clone()
        }
    }
    impl TimeEntryCreateTimeEntry {
        pub fn builder() -> builder::TimeEntryCreateTimeEntry {
            Default::default()
        }
    }
    ///Object for when reading time entries
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Object for when reading time entries",
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/TimeEntry"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct TimeEntryRead(pub TimeEntry);
    impl ::std::ops::Deref for TimeEntryRead {
        type Target = TimeEntry;
        fn deref(&self) -> &TimeEntry {
            &self.0
        }
    }
    impl ::std::convert::From<TimeEntryRead> for TimeEntry {
        fn from(value: TimeEntryRead) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&TimeEntryRead> for TimeEntryRead {
        fn from(value: &TimeEntryRead) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<TimeEntry> for TimeEntryRead {
        fn from(value: TimeEntry) -> Self {
            Self(value)
        }
    }
    ///Object for updating time entries
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Object for updating time entries",
    ///  "type": "object",
    ///  "required": [
    ///    "time_entry"
    ///  ],
    ///  "properties": {
    ///    "time_entry": {
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TimeEntry"
    ///        },
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "ended_at"
    ///          ],
    ///          "properties": {
    ///            "billable": {
    ///              "type": "boolean"
    ///            },
    ///            "contact_id": {
    ///              "type": "string"
    ///            },
    ///            "description": {
    ///              "type": "string"
    ///            },
    ///            "detail_id": {
    ///              "type": "string"
    ///            },
    ///            "ended_at": {
    ///              "type": "string"
    ///            },
    ///            "paused_duration": {
    ///              "type": "number"
    ///            },
    ///            "project_id": {
    ///              "type": "string"
    ///            },
    ///            "started_at": {
    ///              "type": "string"
    ///            },
    ///            "user_id": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TimeEntryUpdate {
        pub time_entry: TimeEntryUpdateTimeEntry,
    }
    impl ::std::convert::From<&TimeEntryUpdate> for TimeEntryUpdate {
        fn from(value: &TimeEntryUpdate) -> Self {
            value.clone()
        }
    }
    impl TimeEntryUpdate {
        pub fn builder() -> builder::TimeEntryUpdate {
            Default::default()
        }
    }
    ///TimeEntryUpdateTimeEntry
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/TimeEntry"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ended_at"
    ///      ],
    ///      "properties": {
    ///        "billable": {
    ///          "type": "boolean"
    ///        },
    ///        "contact_id": {
    ///          "type": "string"
    ///        },
    ///        "description": {
    ///          "type": "string"
    ///        },
    ///        "detail_id": {
    ///          "type": "string"
    ///        },
    ///        "ended_at": {
    ///          "type": "string"
    ///        },
    ///        "paused_duration": {
    ///          "type": "number"
    ///        },
    ///        "project_id": {
    ///          "type": "string"
    ///        },
    ///        "started_at": {
    ///          "type": "string"
    ///        },
    ///        "user_id": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TimeEntryUpdateTimeEntry {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub administration_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub billable: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub contact: ::std::option::Option<Contact>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub contact_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub detail_id: ::std::option::Option<::std::string::String>,
        pub ended_at: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub events: ::std::vec::Vec<Event>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub notes: ::std::vec::Vec<Note>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub paused_duration: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub project: ::std::option::Option<Project>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub started_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&TimeEntryUpdateTimeEntry> for TimeEntryUpdateTimeEntry {
        fn from(value: &TimeEntryUpdateTimeEntry) -> Self {
            value.clone()
        }
    }
    impl TimeEntryUpdateTimeEntry {
        pub fn builder() -> builder::TimeEntryUpdateTimeEntry {
            Default::default()
        }
    }
    ///Object describing a user associated with the account.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Object describing a user associated with the account.",
    ///  "examples": [
    ///    {
    ///      "created_at": "2025-02-17T10:44:56.902Z",
    ///      "email": "info@moneybird.nl",
    ///      "email_validated": true,
    ///      "id": "446241518578041973",
    ///      "language": "nl",
    ///      "name": "Mo Neybird",
    ///      "permissions": [
    ///        "sales_invoices",
    ///        "documents",
    ///        "estimates",
    ///        "bank",
    ///        "settings",
    ///        "ownership",
    ///        "time_entries"
    ///      ],
    ///      "time_zone": "Europe/Amsterdam",
    ///      "updated_at": "2025-02-17T10:44:57.128Z",
    ///      "user_type": "owner"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "properties": {
    ///    "created_at": {
    ///      "description": "The timestamp when the user was created.",
    ///      "type": "string"
    ///    },
    ///    "email": {
    ///      "description": "The email address of the user.",
    ///      "type": "string"
    ///    },
    ///    "email_validated": {
    ///      "description": "Indicates if the user’s email address has been validated.",
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "description": "The ID of the user.",
    ///      "type": "string"
    ///    },
    ///    "is_admin": {
    ///      "description": "Indicates if the user has administrative privileges.",
    ///      "type": "boolean"
    ///    },
    ///    "language": {
    ///      "description": "The language preference of the user (e.g., \"nl\").",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The name of the user.",
    ///      "type": "string"
    ///    },
    ///    "permissions": {
    ///      "description": "List of permissions granted to the user.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string",
    ///        "enum": [
    ///          "sales_invoices",
    ///          "documents",
    ///          "estimates",
    ///          "bank",
    ///          "settings",
    ///          "ownership",
    ///          "time_entries"
    ///        ]
    ///      }
    ///    },
    ///    "time_zone": {
    ///      "description": "The time zone of the user (e.g., \"Europe/Amsterdam\").",
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "description": "The timestamp when the user was last updated.",
    ///      "type": "string"
    ///    },
    ///    "user_type": {
    ///      "description": "The type of user (e.g., \"owner\", \"employee\", or \"accountant\").",
    ///      "type": "string",
    ///      "enum": [
    ///        "owner",
    ///        "employee",
    ///        "accountant"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct User {
        ///The timestamp when the user was created.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        ///The email address of the user.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email: ::std::option::Option<::std::string::String>,
        ///Indicates if the user’s email address has been validated.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email_validated: ::std::option::Option<bool>,
        ///The ID of the user.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        ///Indicates if the user has administrative privileges.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_admin: ::std::option::Option<bool>,
        ///The language preference of the user (e.g., "nl").
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub language: ::std::option::Option<::std::string::String>,
        ///The name of the user.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///List of permissions granted to the user.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub permissions: ::std::vec::Vec<UserPermissionsItem>,
        ///The time zone of the user (e.g., "Europe/Amsterdam").
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time_zone: ::std::option::Option<::std::string::String>,
        ///The timestamp when the user was last updated.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
        ///The type of user (e.g., "owner", "employee", or "accountant").
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user_type: ::std::option::Option<UserUserType>,
    }
    impl ::std::convert::From<&User> for User {
        fn from(value: &User) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for User {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                email: Default::default(),
                email_validated: Default::default(),
                id: Default::default(),
                is_admin: Default::default(),
                language: Default::default(),
                name: Default::default(),
                permissions: Default::default(),
                time_zone: Default::default(),
                updated_at: Default::default(),
                user_type: Default::default(),
            }
        }
    }
    impl User {
        pub fn builder() -> builder::User {
            Default::default()
        }
    }
    ///UserPermissionsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "sales_invoices",
    ///    "documents",
    ///    "estimates",
    ///    "bank",
    ///    "settings",
    ///    "ownership",
    ///    "time_entries"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UserPermissionsItem {
        #[serde(rename = "sales_invoices")]
        SalesInvoices,
        #[serde(rename = "documents")]
        Documents,
        #[serde(rename = "estimates")]
        Estimates,
        #[serde(rename = "bank")]
        Bank,
        #[serde(rename = "settings")]
        Settings,
        #[serde(rename = "ownership")]
        Ownership,
        #[serde(rename = "time_entries")]
        TimeEntries,
    }
    impl ::std::convert::From<&Self> for UserPermissionsItem {
        fn from(value: &UserPermissionsItem) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for UserPermissionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::SalesInvoices => write!(f, "sales_invoices"),
                Self::Documents => write!(f, "documents"),
                Self::Estimates => write!(f, "estimates"),
                Self::Bank => write!(f, "bank"),
                Self::Settings => write!(f, "settings"),
                Self::Ownership => write!(f, "ownership"),
                Self::TimeEntries => write!(f, "time_entries"),
            }
        }
    }
    impl ::std::str::FromStr for UserPermissionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "sales_invoices" => Ok(Self::SalesInvoices),
                "documents" => Ok(Self::Documents),
                "estimates" => Ok(Self::Estimates),
                "bank" => Ok(Self::Bank),
                "settings" => Ok(Self::Settings),
                "ownership" => Ok(Self::Ownership),
                "time_entries" => Ok(Self::TimeEntries),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UserPermissionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for UserPermissionsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UserPermissionsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///The type of user (e.g., "owner", "employee", or "accountant").
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The type of user (e.g., \"owner\", \"employee\", or \"accountant\").",
    ///  "type": "string",
    ///  "enum": [
    ///    "owner",
    ///    "employee",
    ///    "accountant"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UserUserType {
        #[serde(rename = "owner")]
        Owner,
        #[serde(rename = "employee")]
        Employee,
        #[serde(rename = "accountant")]
        Accountant,
    }
    impl ::std::convert::From<&Self> for UserUserType {
        fn from(value: &UserUserType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for UserUserType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Owner => write!(f, "owner"),
                Self::Employee => write!(f, "employee"),
                Self::Accountant => write!(f, "accountant"),
            }
        }
    }
    impl ::std::str::FromStr for UserUserType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "owner" => Ok(Self::Owner),
                "employee" => Ok(Self::Employee),
                "accountant" => Ok(Self::Accountant),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UserUserType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for UserUserType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UserUserType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct Administration {
            access: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            country: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            currency: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            language: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            time_zone: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for Administration {
            fn default() -> Self {
                Self {
                    access: Ok(Default::default()),
                    country: Ok(Default::default()),
                    currency: Ok(Default::default()),
                    id: Ok(Default::default()),
                    language: Ok(Default::default()),
                    name: Ok(Default::default()),
                    time_zone: Ok(Default::default()),
                }
            }
        }
        impl Administration {
            pub fn access<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.access = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for access: {}", e));
                self
            }
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn currency<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.currency = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for currency: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn language<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.language = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for language: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn time_zone<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.time_zone = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for time_zone: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<Administration> for super::Administration {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Administration,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    access: value.access?,
                    country: value.country?,
                    currency: value.currency?,
                    id: value.id?,
                    language: value.language?,
                    name: value.name?,
                    time_zone: value.time_zone?,
                })
            }
        }
        impl ::std::convert::From<super::Administration> for Administration {
            fn from(value: super::Administration) -> Self {
                Self {
                    access: Ok(value.access),
                    country: Ok(value.country),
                    currency: Ok(value.currency),
                    id: Ok(value.id),
                    language: Ok(value.language),
                    name: Ok(value.name),
                    time_zone: Ok(value.time_zone),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct Contact {
            address1: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            address2: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            bank_account: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            chamber_of_commerce: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            city: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            company_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            country: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            customer_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            delivery_method: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            direct_debit: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            email_ubl: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            estimate_workflow_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            firstname: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            invoice_workflow_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            lastname: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            phone: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_estimates_to_attention: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_estimates_to_email: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_invoices_to_attention: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_invoices_to_email: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_active: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            sepa_bic: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_iban: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_iban_account_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_mandate_date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_mandate_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_sequence_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            si_identifier: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            si_identifier_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            tax_number: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            zipcode: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for Contact {
            fn default() -> Self {
                Self {
                    address1: Ok(Default::default()),
                    address2: Ok(Default::default()),
                    bank_account: Ok(Default::default()),
                    chamber_of_commerce: Ok(Default::default()),
                    city: Ok(Default::default()),
                    company_name: Ok(Default::default()),
                    country: Ok(Default::default()),
                    customer_id: Ok(Default::default()),
                    delivery_method: Ok(Default::default()),
                    direct_debit: Ok(Default::default()),
                    email_ubl: Ok(Default::default()),
                    estimate_workflow_id: Ok(Default::default()),
                    firstname: Ok(Default::default()),
                    id: Ok(Default::default()),
                    invoice_workflow_id: Ok(Default::default()),
                    lastname: Ok(Default::default()),
                    phone: Ok(Default::default()),
                    send_estimates_to_attention: Ok(Default::default()),
                    send_estimates_to_email: Ok(Default::default()),
                    send_invoices_to_attention: Ok(Default::default()),
                    send_invoices_to_email: Ok(Default::default()),
                    sepa_active: Ok(Default::default()),
                    sepa_bic: Ok(Default::default()),
                    sepa_iban: Ok(Default::default()),
                    sepa_iban_account_name: Ok(Default::default()),
                    sepa_mandate_date: Ok(Default::default()),
                    sepa_mandate_id: Ok(Default::default()),
                    sepa_sequence_type: Ok(Default::default()),
                    si_identifier: Ok(Default::default()),
                    si_identifier_type: Ok(Default::default()),
                    tax_number: Ok(Default::default()),
                    zipcode: Ok(Default::default()),
                }
            }
        }
        impl Contact {
            pub fn address1<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.address1 = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address1: {}", e));
                self
            }
            pub fn address2<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.address2 = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address2: {}", e));
                self
            }
            pub fn bank_account<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.bank_account = value.try_into().map_err(|e| {
                    format!("error converting supplied value for bank_account: {}", e)
                });
                self
            }
            pub fn chamber_of_commerce<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.chamber_of_commerce = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for chamber_of_commerce: {}",
                        e
                    )
                });
                self
            }
            pub fn city<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.city = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city: {}", e));
                self
            }
            pub fn company_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.company_name = value.try_into().map_err(|e| {
                    format!("error converting supplied value for company_name: {}", e)
                });
                self
            }
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn customer_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.customer_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for customer_id: {}", e));
                self
            }
            pub fn delivery_method<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.delivery_method = value.try_into().map_err(|e| {
                    format!("error converting supplied value for delivery_method: {}", e)
                });
                self
            }
            pub fn direct_debit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.direct_debit = value.try_into().map_err(|e| {
                    format!("error converting supplied value for direct_debit: {}", e)
                });
                self
            }
            pub fn email_ubl<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.email_ubl = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email_ubl: {}", e));
                self
            }
            pub fn estimate_workflow_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.estimate_workflow_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for estimate_workflow_id: {}",
                        e
                    )
                });
                self
            }
            pub fn firstname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.firstname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for firstname: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn invoice_workflow_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.invoice_workflow_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for invoice_workflow_id: {}",
                        e
                    )
                });
                self
            }
            pub fn lastname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.lastname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lastname: {}", e));
                self
            }
            pub fn phone<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.phone = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for phone: {}", e));
                self
            }
            pub fn send_estimates_to_attention<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_estimates_to_attention = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_estimates_to_attention: {}",
                        e
                    )
                });
                self
            }
            pub fn send_estimates_to_email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_estimates_to_email = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_estimates_to_email: {}",
                        e
                    )
                });
                self
            }
            pub fn send_invoices_to_attention<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_invoices_to_attention = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_invoices_to_attention: {}",
                        e
                    )
                });
                self
            }
            pub fn send_invoices_to_email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_invoices_to_email = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_invoices_to_email: {}",
                        e
                    )
                });
                self
            }
            pub fn sepa_active<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_active = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sepa_active: {}", e));
                self
            }
            pub fn sepa_bic<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_bic = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sepa_bic: {}", e));
                self
            }
            pub fn sepa_iban<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_iban = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sepa_iban: {}", e));
                self
            }
            pub fn sepa_iban_account_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_iban_account_name = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for sepa_iban_account_name: {}",
                        e
                    )
                });
                self
            }
            pub fn sepa_mandate_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_mandate_date = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for sepa_mandate_date: {}",
                        e
                    )
                });
                self
            }
            pub fn sepa_mandate_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_mandate_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for sepa_mandate_id: {}", e)
                });
                self
            }
            pub fn sepa_sequence_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_sequence_type = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for sepa_sequence_type: {}",
                        e
                    )
                });
                self
            }
            pub fn si_identifier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.si_identifier = value.try_into().map_err(|e| {
                    format!("error converting supplied value for si_identifier: {}", e)
                });
                self
            }
            pub fn si_identifier_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.si_identifier_type = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for si_identifier_type: {}",
                        e
                    )
                });
                self
            }
            pub fn tax_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.tax_number = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tax_number: {}", e));
                self
            }
            pub fn zipcode<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.zipcode = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for zipcode: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<Contact> for super::Contact {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Contact,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    address1: value.address1?,
                    address2: value.address2?,
                    bank_account: value.bank_account?,
                    chamber_of_commerce: value.chamber_of_commerce?,
                    city: value.city?,
                    company_name: value.company_name?,
                    country: value.country?,
                    customer_id: value.customer_id?,
                    delivery_method: value.delivery_method?,
                    direct_debit: value.direct_debit?,
                    email_ubl: value.email_ubl?,
                    estimate_workflow_id: value.estimate_workflow_id?,
                    firstname: value.firstname?,
                    id: value.id?,
                    invoice_workflow_id: value.invoice_workflow_id?,
                    lastname: value.lastname?,
                    phone: value.phone?,
                    send_estimates_to_attention: value.send_estimates_to_attention?,
                    send_estimates_to_email: value.send_estimates_to_email?,
                    send_invoices_to_attention: value.send_invoices_to_attention?,
                    send_invoices_to_email: value.send_invoices_to_email?,
                    sepa_active: value.sepa_active?,
                    sepa_bic: value.sepa_bic?,
                    sepa_iban: value.sepa_iban?,
                    sepa_iban_account_name: value.sepa_iban_account_name?,
                    sepa_mandate_date: value.sepa_mandate_date?,
                    sepa_mandate_id: value.sepa_mandate_id?,
                    sepa_sequence_type: value.sepa_sequence_type?,
                    si_identifier: value.si_identifier?,
                    si_identifier_type: value.si_identifier_type?,
                    tax_number: value.tax_number?,
                    zipcode: value.zipcode?,
                })
            }
        }
        impl ::std::convert::From<super::Contact> for Contact {
            fn from(value: super::Contact) -> Self {
                Self {
                    address1: Ok(value.address1),
                    address2: Ok(value.address2),
                    bank_account: Ok(value.bank_account),
                    chamber_of_commerce: Ok(value.chamber_of_commerce),
                    city: Ok(value.city),
                    company_name: Ok(value.company_name),
                    country: Ok(value.country),
                    customer_id: Ok(value.customer_id),
                    delivery_method: Ok(value.delivery_method),
                    direct_debit: Ok(value.direct_debit),
                    email_ubl: Ok(value.email_ubl),
                    estimate_workflow_id: Ok(value.estimate_workflow_id),
                    firstname: Ok(value.firstname),
                    id: Ok(value.id),
                    invoice_workflow_id: Ok(value.invoice_workflow_id),
                    lastname: Ok(value.lastname),
                    phone: Ok(value.phone),
                    send_estimates_to_attention: Ok(value.send_estimates_to_attention),
                    send_estimates_to_email: Ok(value.send_estimates_to_email),
                    send_invoices_to_attention: Ok(value.send_invoices_to_attention),
                    send_invoices_to_email: Ok(value.send_invoices_to_email),
                    sepa_active: Ok(value.sepa_active),
                    sepa_bic: Ok(value.sepa_bic),
                    sepa_iban: Ok(value.sepa_iban),
                    sepa_iban_account_name: Ok(value.sepa_iban_account_name),
                    sepa_mandate_date: Ok(value.sepa_mandate_date),
                    sepa_mandate_id: Ok(value.sepa_mandate_id),
                    sepa_sequence_type: Ok(value.sepa_sequence_type),
                    si_identifier: Ok(value.si_identifier),
                    si_identifier_type: Ok(value.si_identifier_type),
                    tax_number: Ok(value.tax_number),
                    zipcode: Ok(value.zipcode),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ContactCreate {
            address1: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            address2: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            bank_account: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            chamber_of_commerce: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            city: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            company_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            contact_person: ::std::result::Result<
                ::std::vec::Vec<super::ContactCreateContactPersonItem>,
                ::std::string::String,
            >,
            country: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            custom_fields_attributes: ::std::result::Result<
                ::std::vec::Vec<super::ContactCreateCustomFieldsAttributesItem>,
                ::std::string::String,
            >,
            customer_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            delivery_method: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            direct_debit: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            email_ubl: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            estimate_workflow_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            firstname: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            invoice_workflow_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            lastname: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            phone: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_estimates_to_attention: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_estimates_to_email: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_invoices_to_attention: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_invoices_to_email: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_active: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            sepa_bic: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_iban: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_iban_account_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_mandate_date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_mandate_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_sequence_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            si_identifier: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            si_identifier_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            tax_number: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            zipcode: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ContactCreate {
            fn default() -> Self {
                Self {
                    address1: Ok(Default::default()),
                    address2: Ok(Default::default()),
                    bank_account: Ok(Default::default()),
                    chamber_of_commerce: Ok(Default::default()),
                    city: Ok(Default::default()),
                    company_name: Ok(Default::default()),
                    contact_person: Ok(Default::default()),
                    country: Ok(Default::default()),
                    custom_fields_attributes: Ok(Default::default()),
                    customer_id: Ok(Default::default()),
                    delivery_method: Ok(Default::default()),
                    direct_debit: Ok(Default::default()),
                    email_ubl: Ok(Default::default()),
                    estimate_workflow_id: Ok(Default::default()),
                    firstname: Ok(Default::default()),
                    id: Ok(Default::default()),
                    invoice_workflow_id: Ok(Default::default()),
                    lastname: Ok(Default::default()),
                    phone: Ok(Default::default()),
                    send_estimates_to_attention: Ok(Default::default()),
                    send_estimates_to_email: Ok(Default::default()),
                    send_invoices_to_attention: Ok(Default::default()),
                    send_invoices_to_email: Ok(Default::default()),
                    sepa_active: Ok(Default::default()),
                    sepa_bic: Ok(Default::default()),
                    sepa_iban: Ok(Default::default()),
                    sepa_iban_account_name: Ok(Default::default()),
                    sepa_mandate_date: Ok(Default::default()),
                    sepa_mandate_id: Ok(Default::default()),
                    sepa_sequence_type: Ok(Default::default()),
                    si_identifier: Ok(Default::default()),
                    si_identifier_type: Ok(Default::default()),
                    tax_number: Ok(Default::default()),
                    zipcode: Ok(Default::default()),
                }
            }
        }
        impl ContactCreate {
            pub fn address1<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.address1 = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address1: {}", e));
                self
            }
            pub fn address2<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.address2 = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address2: {}", e));
                self
            }
            pub fn bank_account<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.bank_account = value.try_into().map_err(|e| {
                    format!("error converting supplied value for bank_account: {}", e)
                });
                self
            }
            pub fn chamber_of_commerce<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.chamber_of_commerce = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for chamber_of_commerce: {}",
                        e
                    )
                });
                self
            }
            pub fn city<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.city = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city: {}", e));
                self
            }
            pub fn company_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.company_name = value.try_into().map_err(|e| {
                    format!("error converting supplied value for company_name: {}", e)
                });
                self
            }
            pub fn contact_person<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::ContactCreateContactPersonItem>>,
                T::Error: ::std::fmt::Display,
            {
                self.contact_person = value.try_into().map_err(|e| {
                    format!("error converting supplied value for contact_person: {}", e)
                });
                self
            }
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn custom_fields_attributes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::ContactCreateCustomFieldsAttributesItem>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.custom_fields_attributes = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for custom_fields_attributes: {}",
                        e
                    )
                });
                self
            }
            pub fn customer_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.customer_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for customer_id: {}", e));
                self
            }
            pub fn delivery_method<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.delivery_method = value.try_into().map_err(|e| {
                    format!("error converting supplied value for delivery_method: {}", e)
                });
                self
            }
            pub fn direct_debit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.direct_debit = value.try_into().map_err(|e| {
                    format!("error converting supplied value for direct_debit: {}", e)
                });
                self
            }
            pub fn email_ubl<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.email_ubl = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email_ubl: {}", e));
                self
            }
            pub fn estimate_workflow_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.estimate_workflow_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for estimate_workflow_id: {}",
                        e
                    )
                });
                self
            }
            pub fn firstname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.firstname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for firstname: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn invoice_workflow_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.invoice_workflow_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for invoice_workflow_id: {}",
                        e
                    )
                });
                self
            }
            pub fn lastname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.lastname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lastname: {}", e));
                self
            }
            pub fn phone<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.phone = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for phone: {}", e));
                self
            }
            pub fn send_estimates_to_attention<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_estimates_to_attention = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_estimates_to_attention: {}",
                        e
                    )
                });
                self
            }
            pub fn send_estimates_to_email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_estimates_to_email = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_estimates_to_email: {}",
                        e
                    )
                });
                self
            }
            pub fn send_invoices_to_attention<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_invoices_to_attention = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_invoices_to_attention: {}",
                        e
                    )
                });
                self
            }
            pub fn send_invoices_to_email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_invoices_to_email = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_invoices_to_email: {}",
                        e
                    )
                });
                self
            }
            pub fn sepa_active<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_active = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sepa_active: {}", e));
                self
            }
            pub fn sepa_bic<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_bic = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sepa_bic: {}", e));
                self
            }
            pub fn sepa_iban<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_iban = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sepa_iban: {}", e));
                self
            }
            pub fn sepa_iban_account_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_iban_account_name = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for sepa_iban_account_name: {}",
                        e
                    )
                });
                self
            }
            pub fn sepa_mandate_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_mandate_date = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for sepa_mandate_date: {}",
                        e
                    )
                });
                self
            }
            pub fn sepa_mandate_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_mandate_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for sepa_mandate_id: {}", e)
                });
                self
            }
            pub fn sepa_sequence_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_sequence_type = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for sepa_sequence_type: {}",
                        e
                    )
                });
                self
            }
            pub fn si_identifier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.si_identifier = value.try_into().map_err(|e| {
                    format!("error converting supplied value for si_identifier: {}", e)
                });
                self
            }
            pub fn si_identifier_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.si_identifier_type = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for si_identifier_type: {}",
                        e
                    )
                });
                self
            }
            pub fn tax_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.tax_number = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tax_number: {}", e));
                self
            }
            pub fn zipcode<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.zipcode = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for zipcode: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ContactCreate> for super::ContactCreate {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ContactCreate,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    address1: value.address1?,
                    address2: value.address2?,
                    bank_account: value.bank_account?,
                    chamber_of_commerce: value.chamber_of_commerce?,
                    city: value.city?,
                    company_name: value.company_name?,
                    contact_person: value.contact_person?,
                    country: value.country?,
                    custom_fields_attributes: value.custom_fields_attributes?,
                    customer_id: value.customer_id?,
                    delivery_method: value.delivery_method?,
                    direct_debit: value.direct_debit?,
                    email_ubl: value.email_ubl?,
                    estimate_workflow_id: value.estimate_workflow_id?,
                    firstname: value.firstname?,
                    id: value.id?,
                    invoice_workflow_id: value.invoice_workflow_id?,
                    lastname: value.lastname?,
                    phone: value.phone?,
                    send_estimates_to_attention: value.send_estimates_to_attention?,
                    send_estimates_to_email: value.send_estimates_to_email?,
                    send_invoices_to_attention: value.send_invoices_to_attention?,
                    send_invoices_to_email: value.send_invoices_to_email?,
                    sepa_active: value.sepa_active?,
                    sepa_bic: value.sepa_bic?,
                    sepa_iban: value.sepa_iban?,
                    sepa_iban_account_name: value.sepa_iban_account_name?,
                    sepa_mandate_date: value.sepa_mandate_date?,
                    sepa_mandate_id: value.sepa_mandate_id?,
                    sepa_sequence_type: value.sepa_sequence_type?,
                    si_identifier: value.si_identifier?,
                    si_identifier_type: value.si_identifier_type?,
                    tax_number: value.tax_number?,
                    zipcode: value.zipcode?,
                })
            }
        }
        impl ::std::convert::From<super::ContactCreate> for ContactCreate {
            fn from(value: super::ContactCreate) -> Self {
                Self {
                    address1: Ok(value.address1),
                    address2: Ok(value.address2),
                    bank_account: Ok(value.bank_account),
                    chamber_of_commerce: Ok(value.chamber_of_commerce),
                    city: Ok(value.city),
                    company_name: Ok(value.company_name),
                    contact_person: Ok(value.contact_person),
                    country: Ok(value.country),
                    custom_fields_attributes: Ok(value.custom_fields_attributes),
                    customer_id: Ok(value.customer_id),
                    delivery_method: Ok(value.delivery_method),
                    direct_debit: Ok(value.direct_debit),
                    email_ubl: Ok(value.email_ubl),
                    estimate_workflow_id: Ok(value.estimate_workflow_id),
                    firstname: Ok(value.firstname),
                    id: Ok(value.id),
                    invoice_workflow_id: Ok(value.invoice_workflow_id),
                    lastname: Ok(value.lastname),
                    phone: Ok(value.phone),
                    send_estimates_to_attention: Ok(value.send_estimates_to_attention),
                    send_estimates_to_email: Ok(value.send_estimates_to_email),
                    send_invoices_to_attention: Ok(value.send_invoices_to_attention),
                    send_invoices_to_email: Ok(value.send_invoices_to_email),
                    sepa_active: Ok(value.sepa_active),
                    sepa_bic: Ok(value.sepa_bic),
                    sepa_iban: Ok(value.sepa_iban),
                    sepa_iban_account_name: Ok(value.sepa_iban_account_name),
                    sepa_mandate_date: Ok(value.sepa_mandate_date),
                    sepa_mandate_id: Ok(value.sepa_mandate_id),
                    sepa_sequence_type: Ok(value.sepa_sequence_type),
                    si_identifier: Ok(value.si_identifier),
                    si_identifier_type: Ok(value.si_identifier_type),
                    tax_number: Ok(value.tax_number),
                    zipcode: Ok(value.zipcode),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ContactCreateContactPersonItem {
            firstname: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            lastname: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ContactCreateContactPersonItem {
            fn default() -> Self {
                Self {
                    firstname: Ok(Default::default()),
                    lastname: Ok(Default::default()),
                }
            }
        }
        impl ContactCreateContactPersonItem {
            pub fn firstname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.firstname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for firstname: {}", e));
                self
            }
            pub fn lastname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.lastname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lastname: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ContactCreateContactPersonItem>
            for super::ContactCreateContactPersonItem
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ContactCreateContactPersonItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    firstname: value.firstname?,
                    lastname: value.lastname?,
                })
            }
        }
        impl ::std::convert::From<super::ContactCreateContactPersonItem>
            for ContactCreateContactPersonItem
        {
            fn from(value: super::ContactCreateContactPersonItem) -> Self {
                Self {
                    firstname: Ok(value.firstname),
                    lastname: Ok(value.lastname),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ContactCreateCustomFieldsAttributesItem {
            id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            value: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ContactCreateCustomFieldsAttributesItem {
            fn default() -> Self {
                Self {
                    id: Ok(Default::default()),
                    value: Ok(Default::default()),
                }
            }
        }
        impl ContactCreateCustomFieldsAttributesItem {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for value: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ContactCreateCustomFieldsAttributesItem>
            for super::ContactCreateCustomFieldsAttributesItem
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ContactCreateCustomFieldsAttributesItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    value: value.value?,
                })
            }
        }
        impl ::std::convert::From<super::ContactCreateCustomFieldsAttributesItem>
            for ContactCreateCustomFieldsAttributesItem
        {
            fn from(value: super::ContactCreateCustomFieldsAttributesItem) -> Self {
                Self {
                    id: Ok(value.id),
                    value: Ok(value.value),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ContactRead {
            address1: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            address2: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            administration_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            archived: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            bank_account: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            chamber_of_commerce: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            city: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            company_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            contact_people: ::std::result::Result<
                ::std::vec::Vec<super::ContactReadContactPeopleItem>,
                ::std::string::String,
            >,
            country: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            custom_fields:
                ::std::result::Result<::std::vec::Vec<super::CustomField>, ::std::string::String>,
            customer_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            delivery_method: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            direct_debit: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            email_ubl: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            estimate_workflow_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            events: ::std::result::Result<::std::vec::Vec<super::Event>, ::std::string::String>,
            firstname: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            invoice_workflow_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            lastname: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            notes: ::std::result::Result<::std::vec::Vec<super::Note>, ::std::string::String>,
            phone: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_estimates_to_attention: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_estimates_to_email: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_invoices_to_attention: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_invoices_to_email: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_active: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            sepa_bic: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_iban: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_iban_account_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_mandate_date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_mandate_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_sequence_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            si_identifier: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            si_identifier_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            tax_number: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            zipcode: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ContactRead {
            fn default() -> Self {
                Self {
                    address1: Ok(Default::default()),
                    address2: Ok(Default::default()),
                    administration_id: Ok(Default::default()),
                    archived: Ok(Default::default()),
                    bank_account: Ok(Default::default()),
                    chamber_of_commerce: Ok(Default::default()),
                    city: Ok(Default::default()),
                    company_name: Ok(Default::default()),
                    contact_people: Ok(Default::default()),
                    country: Ok(Default::default()),
                    custom_fields: Ok(Default::default()),
                    customer_id: Ok(Default::default()),
                    delivery_method: Ok(Default::default()),
                    direct_debit: Ok(Default::default()),
                    email_ubl: Ok(Default::default()),
                    estimate_workflow_id: Ok(Default::default()),
                    events: Ok(Default::default()),
                    firstname: Ok(Default::default()),
                    id: Ok(Default::default()),
                    invoice_workflow_id: Ok(Default::default()),
                    lastname: Ok(Default::default()),
                    notes: Ok(Default::default()),
                    phone: Ok(Default::default()),
                    send_estimates_to_attention: Ok(Default::default()),
                    send_estimates_to_email: Ok(Default::default()),
                    send_invoices_to_attention: Ok(Default::default()),
                    send_invoices_to_email: Ok(Default::default()),
                    sepa_active: Ok(Default::default()),
                    sepa_bic: Ok(Default::default()),
                    sepa_iban: Ok(Default::default()),
                    sepa_iban_account_name: Ok(Default::default()),
                    sepa_mandate_date: Ok(Default::default()),
                    sepa_mandate_id: Ok(Default::default()),
                    sepa_sequence_type: Ok(Default::default()),
                    si_identifier: Ok(Default::default()),
                    si_identifier_type: Ok(Default::default()),
                    tax_number: Ok(Default::default()),
                    zipcode: Ok(Default::default()),
                }
            }
        }
        impl ContactRead {
            pub fn address1<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.address1 = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address1: {}", e));
                self
            }
            pub fn address2<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.address2 = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address2: {}", e));
                self
            }
            pub fn administration_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.administration_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for administration_id: {}",
                        e
                    )
                });
                self
            }
            pub fn archived<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.archived = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for archived: {}", e));
                self
            }
            pub fn bank_account<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.bank_account = value.try_into().map_err(|e| {
                    format!("error converting supplied value for bank_account: {}", e)
                });
                self
            }
            pub fn chamber_of_commerce<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.chamber_of_commerce = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for chamber_of_commerce: {}",
                        e
                    )
                });
                self
            }
            pub fn city<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.city = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city: {}", e));
                self
            }
            pub fn company_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.company_name = value.try_into().map_err(|e| {
                    format!("error converting supplied value for company_name: {}", e)
                });
                self
            }
            pub fn contact_people<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::ContactReadContactPeopleItem>>,
                T::Error: ::std::fmt::Display,
            {
                self.contact_people = value.try_into().map_err(|e| {
                    format!("error converting supplied value for contact_people: {}", e)
                });
                self
            }
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn custom_fields<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::CustomField>>,
                T::Error: ::std::fmt::Display,
            {
                self.custom_fields = value.try_into().map_err(|e| {
                    format!("error converting supplied value for custom_fields: {}", e)
                });
                self
            }
            pub fn customer_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.customer_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for customer_id: {}", e));
                self
            }
            pub fn delivery_method<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.delivery_method = value.try_into().map_err(|e| {
                    format!("error converting supplied value for delivery_method: {}", e)
                });
                self
            }
            pub fn direct_debit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.direct_debit = value.try_into().map_err(|e| {
                    format!("error converting supplied value for direct_debit: {}", e)
                });
                self
            }
            pub fn email_ubl<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.email_ubl = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email_ubl: {}", e));
                self
            }
            pub fn estimate_workflow_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.estimate_workflow_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for estimate_workflow_id: {}",
                        e
                    )
                });
                self
            }
            pub fn events<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Event>>,
                T::Error: ::std::fmt::Display,
            {
                self.events = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for events: {}", e));
                self
            }
            pub fn firstname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.firstname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for firstname: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn invoice_workflow_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.invoice_workflow_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for invoice_workflow_id: {}",
                        e
                    )
                });
                self
            }
            pub fn lastname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.lastname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lastname: {}", e));
                self
            }
            pub fn notes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Note>>,
                T::Error: ::std::fmt::Display,
            {
                self.notes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for notes: {}", e));
                self
            }
            pub fn phone<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.phone = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for phone: {}", e));
                self
            }
            pub fn send_estimates_to_attention<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_estimates_to_attention = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_estimates_to_attention: {}",
                        e
                    )
                });
                self
            }
            pub fn send_estimates_to_email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_estimates_to_email = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_estimates_to_email: {}",
                        e
                    )
                });
                self
            }
            pub fn send_invoices_to_attention<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_invoices_to_attention = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_invoices_to_attention: {}",
                        e
                    )
                });
                self
            }
            pub fn send_invoices_to_email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_invoices_to_email = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_invoices_to_email: {}",
                        e
                    )
                });
                self
            }
            pub fn sepa_active<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_active = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sepa_active: {}", e));
                self
            }
            pub fn sepa_bic<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_bic = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sepa_bic: {}", e));
                self
            }
            pub fn sepa_iban<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_iban = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sepa_iban: {}", e));
                self
            }
            pub fn sepa_iban_account_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_iban_account_name = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for sepa_iban_account_name: {}",
                        e
                    )
                });
                self
            }
            pub fn sepa_mandate_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_mandate_date = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for sepa_mandate_date: {}",
                        e
                    )
                });
                self
            }
            pub fn sepa_mandate_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_mandate_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for sepa_mandate_id: {}", e)
                });
                self
            }
            pub fn sepa_sequence_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_sequence_type = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for sepa_sequence_type: {}",
                        e
                    )
                });
                self
            }
            pub fn si_identifier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.si_identifier = value.try_into().map_err(|e| {
                    format!("error converting supplied value for si_identifier: {}", e)
                });
                self
            }
            pub fn si_identifier_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.si_identifier_type = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for si_identifier_type: {}",
                        e
                    )
                });
                self
            }
            pub fn tax_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.tax_number = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tax_number: {}", e));
                self
            }
            pub fn zipcode<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.zipcode = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for zipcode: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ContactRead> for super::ContactRead {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ContactRead,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    address1: value.address1?,
                    address2: value.address2?,
                    administration_id: value.administration_id?,
                    archived: value.archived?,
                    bank_account: value.bank_account?,
                    chamber_of_commerce: value.chamber_of_commerce?,
                    city: value.city?,
                    company_name: value.company_name?,
                    contact_people: value.contact_people?,
                    country: value.country?,
                    custom_fields: value.custom_fields?,
                    customer_id: value.customer_id?,
                    delivery_method: value.delivery_method?,
                    direct_debit: value.direct_debit?,
                    email_ubl: value.email_ubl?,
                    estimate_workflow_id: value.estimate_workflow_id?,
                    events: value.events?,
                    firstname: value.firstname?,
                    id: value.id?,
                    invoice_workflow_id: value.invoice_workflow_id?,
                    lastname: value.lastname?,
                    notes: value.notes?,
                    phone: value.phone?,
                    send_estimates_to_attention: value.send_estimates_to_attention?,
                    send_estimates_to_email: value.send_estimates_to_email?,
                    send_invoices_to_attention: value.send_invoices_to_attention?,
                    send_invoices_to_email: value.send_invoices_to_email?,
                    sepa_active: value.sepa_active?,
                    sepa_bic: value.sepa_bic?,
                    sepa_iban: value.sepa_iban?,
                    sepa_iban_account_name: value.sepa_iban_account_name?,
                    sepa_mandate_date: value.sepa_mandate_date?,
                    sepa_mandate_id: value.sepa_mandate_id?,
                    sepa_sequence_type: value.sepa_sequence_type?,
                    si_identifier: value.si_identifier?,
                    si_identifier_type: value.si_identifier_type?,
                    tax_number: value.tax_number?,
                    zipcode: value.zipcode?,
                })
            }
        }
        impl ::std::convert::From<super::ContactRead> for ContactRead {
            fn from(value: super::ContactRead) -> Self {
                Self {
                    address1: Ok(value.address1),
                    address2: Ok(value.address2),
                    administration_id: Ok(value.administration_id),
                    archived: Ok(value.archived),
                    bank_account: Ok(value.bank_account),
                    chamber_of_commerce: Ok(value.chamber_of_commerce),
                    city: Ok(value.city),
                    company_name: Ok(value.company_name),
                    contact_people: Ok(value.contact_people),
                    country: Ok(value.country),
                    custom_fields: Ok(value.custom_fields),
                    customer_id: Ok(value.customer_id),
                    delivery_method: Ok(value.delivery_method),
                    direct_debit: Ok(value.direct_debit),
                    email_ubl: Ok(value.email_ubl),
                    estimate_workflow_id: Ok(value.estimate_workflow_id),
                    events: Ok(value.events),
                    firstname: Ok(value.firstname),
                    id: Ok(value.id),
                    invoice_workflow_id: Ok(value.invoice_workflow_id),
                    lastname: Ok(value.lastname),
                    notes: Ok(value.notes),
                    phone: Ok(value.phone),
                    send_estimates_to_attention: Ok(value.send_estimates_to_attention),
                    send_estimates_to_email: Ok(value.send_estimates_to_email),
                    send_invoices_to_attention: Ok(value.send_invoices_to_attention),
                    send_invoices_to_email: Ok(value.send_invoices_to_email),
                    sepa_active: Ok(value.sepa_active),
                    sepa_bic: Ok(value.sepa_bic),
                    sepa_iban: Ok(value.sepa_iban),
                    sepa_iban_account_name: Ok(value.sepa_iban_account_name),
                    sepa_mandate_date: Ok(value.sepa_mandate_date),
                    sepa_mandate_id: Ok(value.sepa_mandate_id),
                    sepa_sequence_type: Ok(value.sepa_sequence_type),
                    si_identifier: Ok(value.si_identifier),
                    si_identifier_type: Ok(value.si_identifier_type),
                    tax_number: Ok(value.tax_number),
                    zipcode: Ok(value.zipcode),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ContactReadContactPeopleItem {
            administration_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            created_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            department: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            email: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            firstname: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            lastname: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            phone: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            updated_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            version: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        }
        impl ::std::default::Default for ContactReadContactPeopleItem {
            fn default() -> Self {
                Self {
                    administration_id: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    department: Ok(Default::default()),
                    email: Ok(Default::default()),
                    firstname: Ok(Default::default()),
                    id: Ok(Default::default()),
                    lastname: Ok(Default::default()),
                    phone: Ok(Default::default()),
                    updated_at: Ok(Default::default()),
                    version: Ok(Default::default()),
                }
            }
        }
        impl ContactReadContactPeopleItem {
            pub fn administration_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.administration_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for administration_id: {}",
                        e
                    )
                });
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn department<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.department = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for department: {}", e));
                self
            }
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {}", e));
                self
            }
            pub fn firstname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.firstname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for firstname: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn lastname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.lastname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lastname: {}", e));
                self
            }
            pub fn phone<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.phone = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for phone: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
            pub fn version<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.version = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for version: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ContactReadContactPeopleItem> for super::ContactReadContactPeopleItem {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ContactReadContactPeopleItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    administration_id: value.administration_id?,
                    created_at: value.created_at?,
                    department: value.department?,
                    email: value.email?,
                    firstname: value.firstname?,
                    id: value.id?,
                    lastname: value.lastname?,
                    phone: value.phone?,
                    updated_at: value.updated_at?,
                    version: value.version?,
                })
            }
        }
        impl ::std::convert::From<super::ContactReadContactPeopleItem> for ContactReadContactPeopleItem {
            fn from(value: super::ContactReadContactPeopleItem) -> Self {
                Self {
                    administration_id: Ok(value.administration_id),
                    created_at: Ok(value.created_at),
                    department: Ok(value.department),
                    email: Ok(value.email),
                    firstname: Ok(value.firstname),
                    id: Ok(value.id),
                    lastname: Ok(value.lastname),
                    phone: Ok(value.phone),
                    updated_at: Ok(value.updated_at),
                    version: Ok(value.version),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ContactUpdate {
            address1: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            address2: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            bank_account: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            chamber_of_commerce: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            city: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            company_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            country: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            custom_fields_attributes: ::std::result::Result<
                ::std::vec::Vec<super::ContactUpdateCustomFieldsAttributesItem>,
                ::std::string::String,
            >,
            customer_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            delivery_method: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            direct_debit: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            email_ubl: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            estimate_workflow_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            firstname: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            invoice_workflow_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            lastname: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            phone: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_estimates_to_attention: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_estimates_to_email: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_invoices_to_attention: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            send_invoices_to_email: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_active: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            sepa_bic: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_iban: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_iban_account_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_mandate_date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_mandate_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sepa_sequence_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            si_identifier: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            si_identifier_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            tax_number: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            zipcode: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ContactUpdate {
            fn default() -> Self {
                Self {
                    address1: Ok(Default::default()),
                    address2: Ok(Default::default()),
                    bank_account: Ok(Default::default()),
                    chamber_of_commerce: Ok(Default::default()),
                    city: Ok(Default::default()),
                    company_name: Ok(Default::default()),
                    country: Ok(Default::default()),
                    custom_fields_attributes: Ok(Default::default()),
                    customer_id: Ok(Default::default()),
                    delivery_method: Ok(Default::default()),
                    direct_debit: Ok(Default::default()),
                    email_ubl: Ok(Default::default()),
                    estimate_workflow_id: Ok(Default::default()),
                    firstname: Ok(Default::default()),
                    id: Ok(Default::default()),
                    invoice_workflow_id: Ok(Default::default()),
                    lastname: Ok(Default::default()),
                    phone: Ok(Default::default()),
                    send_estimates_to_attention: Ok(Default::default()),
                    send_estimates_to_email: Ok(Default::default()),
                    send_invoices_to_attention: Ok(Default::default()),
                    send_invoices_to_email: Ok(Default::default()),
                    sepa_active: Ok(Default::default()),
                    sepa_bic: Ok(Default::default()),
                    sepa_iban: Ok(Default::default()),
                    sepa_iban_account_name: Ok(Default::default()),
                    sepa_mandate_date: Ok(Default::default()),
                    sepa_mandate_id: Ok(Default::default()),
                    sepa_sequence_type: Ok(Default::default()),
                    si_identifier: Ok(Default::default()),
                    si_identifier_type: Ok(Default::default()),
                    tax_number: Ok(Default::default()),
                    zipcode: Ok(Default::default()),
                }
            }
        }
        impl ContactUpdate {
            pub fn address1<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.address1 = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address1: {}", e));
                self
            }
            pub fn address2<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.address2 = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address2: {}", e));
                self
            }
            pub fn bank_account<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.bank_account = value.try_into().map_err(|e| {
                    format!("error converting supplied value for bank_account: {}", e)
                });
                self
            }
            pub fn chamber_of_commerce<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.chamber_of_commerce = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for chamber_of_commerce: {}",
                        e
                    )
                });
                self
            }
            pub fn city<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.city = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city: {}", e));
                self
            }
            pub fn company_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.company_name = value.try_into().map_err(|e| {
                    format!("error converting supplied value for company_name: {}", e)
                });
                self
            }
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn custom_fields_attributes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::ContactUpdateCustomFieldsAttributesItem>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.custom_fields_attributes = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for custom_fields_attributes: {}",
                        e
                    )
                });
                self
            }
            pub fn customer_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.customer_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for customer_id: {}", e));
                self
            }
            pub fn delivery_method<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.delivery_method = value.try_into().map_err(|e| {
                    format!("error converting supplied value for delivery_method: {}", e)
                });
                self
            }
            pub fn direct_debit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.direct_debit = value.try_into().map_err(|e| {
                    format!("error converting supplied value for direct_debit: {}", e)
                });
                self
            }
            pub fn email_ubl<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.email_ubl = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email_ubl: {}", e));
                self
            }
            pub fn estimate_workflow_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.estimate_workflow_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for estimate_workflow_id: {}",
                        e
                    )
                });
                self
            }
            pub fn firstname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.firstname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for firstname: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn invoice_workflow_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.invoice_workflow_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for invoice_workflow_id: {}",
                        e
                    )
                });
                self
            }
            pub fn lastname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.lastname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lastname: {}", e));
                self
            }
            pub fn phone<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.phone = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for phone: {}", e));
                self
            }
            pub fn send_estimates_to_attention<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_estimates_to_attention = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_estimates_to_attention: {}",
                        e
                    )
                });
                self
            }
            pub fn send_estimates_to_email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_estimates_to_email = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_estimates_to_email: {}",
                        e
                    )
                });
                self
            }
            pub fn send_invoices_to_attention<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_invoices_to_attention = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_invoices_to_attention: {}",
                        e
                    )
                });
                self
            }
            pub fn send_invoices_to_email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.send_invoices_to_email = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for send_invoices_to_email: {}",
                        e
                    )
                });
                self
            }
            pub fn sepa_active<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_active = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sepa_active: {}", e));
                self
            }
            pub fn sepa_bic<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_bic = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sepa_bic: {}", e));
                self
            }
            pub fn sepa_iban<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_iban = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sepa_iban: {}", e));
                self
            }
            pub fn sepa_iban_account_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_iban_account_name = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for sepa_iban_account_name: {}",
                        e
                    )
                });
                self
            }
            pub fn sepa_mandate_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_mandate_date = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for sepa_mandate_date: {}",
                        e
                    )
                });
                self
            }
            pub fn sepa_mandate_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_mandate_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for sepa_mandate_id: {}", e)
                });
                self
            }
            pub fn sepa_sequence_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sepa_sequence_type = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for sepa_sequence_type: {}",
                        e
                    )
                });
                self
            }
            pub fn si_identifier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.si_identifier = value.try_into().map_err(|e| {
                    format!("error converting supplied value for si_identifier: {}", e)
                });
                self
            }
            pub fn si_identifier_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.si_identifier_type = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for si_identifier_type: {}",
                        e
                    )
                });
                self
            }
            pub fn tax_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.tax_number = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tax_number: {}", e));
                self
            }
            pub fn zipcode<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.zipcode = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for zipcode: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ContactUpdate> for super::ContactUpdate {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ContactUpdate,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    address1: value.address1?,
                    address2: value.address2?,
                    bank_account: value.bank_account?,
                    chamber_of_commerce: value.chamber_of_commerce?,
                    city: value.city?,
                    company_name: value.company_name?,
                    country: value.country?,
                    custom_fields_attributes: value.custom_fields_attributes?,
                    customer_id: value.customer_id?,
                    delivery_method: value.delivery_method?,
                    direct_debit: value.direct_debit?,
                    email_ubl: value.email_ubl?,
                    estimate_workflow_id: value.estimate_workflow_id?,
                    firstname: value.firstname?,
                    id: value.id?,
                    invoice_workflow_id: value.invoice_workflow_id?,
                    lastname: value.lastname?,
                    phone: value.phone?,
                    send_estimates_to_attention: value.send_estimates_to_attention?,
                    send_estimates_to_email: value.send_estimates_to_email?,
                    send_invoices_to_attention: value.send_invoices_to_attention?,
                    send_invoices_to_email: value.send_invoices_to_email?,
                    sepa_active: value.sepa_active?,
                    sepa_bic: value.sepa_bic?,
                    sepa_iban: value.sepa_iban?,
                    sepa_iban_account_name: value.sepa_iban_account_name?,
                    sepa_mandate_date: value.sepa_mandate_date?,
                    sepa_mandate_id: value.sepa_mandate_id?,
                    sepa_sequence_type: value.sepa_sequence_type?,
                    si_identifier: value.si_identifier?,
                    si_identifier_type: value.si_identifier_type?,
                    tax_number: value.tax_number?,
                    zipcode: value.zipcode?,
                })
            }
        }
        impl ::std::convert::From<super::ContactUpdate> for ContactUpdate {
            fn from(value: super::ContactUpdate) -> Self {
                Self {
                    address1: Ok(value.address1),
                    address2: Ok(value.address2),
                    bank_account: Ok(value.bank_account),
                    chamber_of_commerce: Ok(value.chamber_of_commerce),
                    city: Ok(value.city),
                    company_name: Ok(value.company_name),
                    country: Ok(value.country),
                    custom_fields_attributes: Ok(value.custom_fields_attributes),
                    customer_id: Ok(value.customer_id),
                    delivery_method: Ok(value.delivery_method),
                    direct_debit: Ok(value.direct_debit),
                    email_ubl: Ok(value.email_ubl),
                    estimate_workflow_id: Ok(value.estimate_workflow_id),
                    firstname: Ok(value.firstname),
                    id: Ok(value.id),
                    invoice_workflow_id: Ok(value.invoice_workflow_id),
                    lastname: Ok(value.lastname),
                    phone: Ok(value.phone),
                    send_estimates_to_attention: Ok(value.send_estimates_to_attention),
                    send_estimates_to_email: Ok(value.send_estimates_to_email),
                    send_invoices_to_attention: Ok(value.send_invoices_to_attention),
                    send_invoices_to_email: Ok(value.send_invoices_to_email),
                    sepa_active: Ok(value.sepa_active),
                    sepa_bic: Ok(value.sepa_bic),
                    sepa_iban: Ok(value.sepa_iban),
                    sepa_iban_account_name: Ok(value.sepa_iban_account_name),
                    sepa_mandate_date: Ok(value.sepa_mandate_date),
                    sepa_mandate_id: Ok(value.sepa_mandate_id),
                    sepa_sequence_type: Ok(value.sepa_sequence_type),
                    si_identifier: Ok(value.si_identifier),
                    si_identifier_type: Ok(value.si_identifier_type),
                    tax_number: Ok(value.tax_number),
                    zipcode: Ok(value.zipcode),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ContactUpdateCustomFieldsAttributesItem {
            id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            value: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ContactUpdateCustomFieldsAttributesItem {
            fn default() -> Self {
                Self {
                    id: Ok(Default::default()),
                    value: Ok(Default::default()),
                }
            }
        }
        impl ContactUpdateCustomFieldsAttributesItem {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for value: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ContactUpdateCustomFieldsAttributesItem>
            for super::ContactUpdateCustomFieldsAttributesItem
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ContactUpdateCustomFieldsAttributesItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    value: value.value?,
                })
            }
        }
        impl ::std::convert::From<super::ContactUpdateCustomFieldsAttributesItem>
            for ContactUpdateCustomFieldsAttributesItem
        {
            fn from(value: super::ContactUpdateCustomFieldsAttributesItem) -> Self {
                Self {
                    id: Ok(value.id),
                    value: Ok(value.value),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct CustomField {
            administration_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            source: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for CustomField {
            fn default() -> Self {
                Self {
                    administration_id: Ok(Default::default()),
                    id: Ok(Default::default()),
                    name: Ok(Default::default()),
                    source: Ok(Default::default()),
                }
            }
        }
        impl CustomField {
            pub fn administration_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.administration_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for administration_id: {}",
                        e
                    )
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn source<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.source = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for source: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<CustomField> for super::CustomField {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CustomField,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    administration_id: value.administration_id?,
                    id: value.id?,
                    name: value.name?,
                    source: value.source?,
                })
            }
        }
        impl ::std::convert::From<super::CustomField> for CustomField {
            fn from(value: super::CustomField) -> Self {
                Self {
                    administration_id: Ok(value.administration_id),
                    id: Ok(value.id),
                    name: Ok(value.name),
                    source: Ok(value.source),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct Event {
            action: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            administration_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            created_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            data: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            link_entity_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            link_entity_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            updated_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            user_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for Event {
            fn default() -> Self {
                Self {
                    action: Ok(Default::default()),
                    administration_id: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    data: Ok(Default::default()),
                    link_entity_id: Ok(Default::default()),
                    link_entity_type: Ok(Default::default()),
                    updated_at: Ok(Default::default()),
                    user_id: Ok(Default::default()),
                }
            }
        }
        impl Event {
            pub fn action<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.action = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for action: {}", e));
                self
            }
            pub fn administration_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.administration_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for administration_id: {}",
                        e
                    )
                });
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn data<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.data = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for data: {}", e));
                self
            }
            pub fn link_entity_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.link_entity_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for link_entity_id: {}", e)
                });
                self
            }
            pub fn link_entity_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.link_entity_type = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for link_entity_type: {}",
                        e
                    )
                });
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
            pub fn user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.user_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user_id: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<Event> for super::Event {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Event,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    action: value.action?,
                    administration_id: value.administration_id?,
                    created_at: value.created_at?,
                    data: value.data?,
                    link_entity_id: value.link_entity_id?,
                    link_entity_type: value.link_entity_type?,
                    updated_at: value.updated_at?,
                    user_id: value.user_id?,
                })
            }
        }
        impl ::std::convert::From<super::Event> for Event {
            fn from(value: super::Event) -> Self {
                Self {
                    action: Ok(value.action),
                    administration_id: Ok(value.administration_id),
                    created_at: Ok(value.created_at),
                    data: Ok(value.data),
                    link_entity_id: Ok(value.link_entity_id),
                    link_entity_type: Ok(value.link_entity_type),
                    updated_at: Ok(value.updated_at),
                    user_id: Ok(value.user_id),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct Note {
            administration_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            assignee_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            completed_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            completed_by_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            created_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            data: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            entity_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            entity_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            note: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            todo: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            todo_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            updated_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            user_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for Note {
            fn default() -> Self {
                Self {
                    administration_id: Ok(Default::default()),
                    assignee_id: Ok(Default::default()),
                    completed_at: Ok(Default::default()),
                    completed_by_id: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    data: Ok(Default::default()),
                    entity_id: Ok(Default::default()),
                    entity_type: Ok(Default::default()),
                    id: Ok(Default::default()),
                    note: Ok(Default::default()),
                    todo: Ok(Default::default()),
                    todo_type: Ok(Default::default()),
                    updated_at: Ok(Default::default()),
                    user_id: Ok(Default::default()),
                }
            }
        }
        impl Note {
            pub fn administration_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.administration_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for administration_id: {}",
                        e
                    )
                });
                self
            }
            pub fn assignee_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.assignee_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for assignee_id: {}", e));
                self
            }
            pub fn completed_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.completed_at = value.try_into().map_err(|e| {
                    format!("error converting supplied value for completed_at: {}", e)
                });
                self
            }
            pub fn completed_by_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.completed_by_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for completed_by_id: {}", e)
                });
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn data<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.data = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for data: {}", e));
                self
            }
            pub fn entity_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.entity_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for entity_id: {}", e));
                self
            }
            pub fn entity_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.entity_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for entity_type: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn note<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.note = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for note: {}", e));
                self
            }
            pub fn todo<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.todo = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for todo: {}", e));
                self
            }
            pub fn todo_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.todo_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for todo_type: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
            pub fn user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.user_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user_id: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<Note> for super::Note {
            type Error = super::error::ConversionError;
            fn try_from(value: Note) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    administration_id: value.administration_id?,
                    assignee_id: value.assignee_id?,
                    completed_at: value.completed_at?,
                    completed_by_id: value.completed_by_id?,
                    created_at: value.created_at?,
                    data: value.data?,
                    entity_id: value.entity_id?,
                    entity_type: value.entity_type?,
                    id: value.id?,
                    note: value.note?,
                    todo: value.todo?,
                    todo_type: value.todo_type?,
                    updated_at: value.updated_at?,
                    user_id: value.user_id?,
                })
            }
        }
        impl ::std::convert::From<super::Note> for Note {
            fn from(value: super::Note) -> Self {
                Self {
                    administration_id: Ok(value.administration_id),
                    assignee_id: Ok(value.assignee_id),
                    completed_at: Ok(value.completed_at),
                    completed_by_id: Ok(value.completed_by_id),
                    created_at: Ok(value.created_at),
                    data: Ok(value.data),
                    entity_id: Ok(value.entity_id),
                    entity_type: Ok(value.entity_type),
                    id: Ok(value.id),
                    note: Ok(value.note),
                    todo: Ok(value.todo),
                    todo_type: Ok(value.todo_type),
                    updated_at: Ok(value.updated_at),
                    user_id: Ok(value.user_id),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct Project {
            budget: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            state: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for Project {
            fn default() -> Self {
                Self {
                    budget: Ok(Default::default()),
                    id: Ok(Default::default()),
                    name: Ok(Default::default()),
                    state: Ok(Default::default()),
                }
            }
        }
        impl Project {
            pub fn budget<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.budget = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for budget: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<Project> for super::Project {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Project,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    budget: value.budget?,
                    id: value.id?,
                    name: value.name?,
                    state: value.state?,
                })
            }
        }
        impl ::std::convert::From<super::Project> for Project {
            fn from(value: super::Project) -> Self {
                Self {
                    budget: Ok(value.budget),
                    id: Ok(value.id),
                    name: Ok(value.name),
                    state: Ok(value.state),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ProjectCreate {
            project: ::std::result::Result<super::ProjectCreateProject, ::std::string::String>,
        }
        impl ::std::default::Default for ProjectCreate {
            fn default() -> Self {
                Self {
                    project: Err("no value supplied for project".to_string()),
                }
            }
        }
        impl ProjectCreate {
            pub fn project<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ProjectCreateProject>,
                T::Error: ::std::fmt::Display,
            {
                self.project = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for project: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ProjectCreate> for super::ProjectCreate {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ProjectCreate,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    project: value.project?,
                })
            }
        }
        impl ::std::convert::From<super::ProjectCreate> for ProjectCreate {
            fn from(value: super::ProjectCreate) -> Self {
                Self {
                    project: Ok(value.project),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ProjectCreateProject {
            budget: ::std::result::Result<f64, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for ProjectCreateProject {
            fn default() -> Self {
                Self {
                    budget: Err("no value supplied for budget".to_string()),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }
        impl ProjectCreateProject {
            pub fn budget<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.budget = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for budget: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ProjectCreateProject> for super::ProjectCreateProject {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ProjectCreateProject,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    budget: value.budget?,
                    name: value.name?,
                })
            }
        }
        impl ::std::convert::From<super::ProjectCreateProject> for ProjectCreateProject {
            fn from(value: super::ProjectCreateProject) -> Self {
                Self {
                    budget: Ok(value.budget),
                    name: Ok(value.name),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct TimeEntry {
            administration_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            billable: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            contact:
                ::std::result::Result<::std::option::Option<super::Contact>, ::std::string::String>,
            contact_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            created_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ended_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            events: ::std::result::Result<::std::vec::Vec<super::Event>, ::std::string::String>,
            id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            notes: ::std::result::Result<::std::vec::Vec<super::Note>, ::std::string::String>,
            paused_duration:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            project:
                ::std::result::Result<::std::option::Option<super::Project>, ::std::string::String>,
            project_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            started_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            updated_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            user_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for TimeEntry {
            fn default() -> Self {
                Self {
                    administration_id: Ok(Default::default()),
                    billable: Ok(Default::default()),
                    contact: Ok(Default::default()),
                    contact_id: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    description: Ok(Default::default()),
                    ended_at: Ok(Default::default()),
                    events: Ok(Default::default()),
                    id: Ok(Default::default()),
                    notes: Ok(Default::default()),
                    paused_duration: Ok(Default::default()),
                    project: Ok(Default::default()),
                    project_id: Ok(Default::default()),
                    started_at: Ok(Default::default()),
                    updated_at: Ok(Default::default()),
                    user_id: Ok(Default::default()),
                }
            }
        }
        impl TimeEntry {
            pub fn administration_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.administration_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for administration_id: {}",
                        e
                    )
                });
                self
            }
            pub fn billable<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.billable = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for billable: {}", e));
                self
            }
            pub fn contact<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Contact>>,
                T::Error: ::std::fmt::Display,
            {
                self.contact = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for contact: {}", e));
                self
            }
            pub fn contact_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.contact_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for contact_id: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn ended_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ended_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ended_at: {}", e));
                self
            }
            pub fn events<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Event>>,
                T::Error: ::std::fmt::Display,
            {
                self.events = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for events: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn notes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Note>>,
                T::Error: ::std::fmt::Display,
            {
                self.notes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for notes: {}", e));
                self
            }
            pub fn paused_duration<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.paused_duration = value.try_into().map_err(|e| {
                    format!("error converting supplied value for paused_duration: {}", e)
                });
                self
            }
            pub fn project<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Project>>,
                T::Error: ::std::fmt::Display,
            {
                self.project = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for project: {}", e));
                self
            }
            pub fn project_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.project_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for project_id: {}", e));
                self
            }
            pub fn started_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.started_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for started_at: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
            pub fn user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.user_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user_id: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<TimeEntry> for super::TimeEntry {
            type Error = super::error::ConversionError;
            fn try_from(
                value: TimeEntry,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    administration_id: value.administration_id?,
                    billable: value.billable?,
                    contact: value.contact?,
                    contact_id: value.contact_id?,
                    created_at: value.created_at?,
                    description: value.description?,
                    ended_at: value.ended_at?,
                    events: value.events?,
                    id: value.id?,
                    notes: value.notes?,
                    paused_duration: value.paused_duration?,
                    project: value.project?,
                    project_id: value.project_id?,
                    started_at: value.started_at?,
                    updated_at: value.updated_at?,
                    user_id: value.user_id?,
                })
            }
        }
        impl ::std::convert::From<super::TimeEntry> for TimeEntry {
            fn from(value: super::TimeEntry) -> Self {
                Self {
                    administration_id: Ok(value.administration_id),
                    billable: Ok(value.billable),
                    contact: Ok(value.contact),
                    contact_id: Ok(value.contact_id),
                    created_at: Ok(value.created_at),
                    description: Ok(value.description),
                    ended_at: Ok(value.ended_at),
                    events: Ok(value.events),
                    id: Ok(value.id),
                    notes: Ok(value.notes),
                    paused_duration: Ok(value.paused_duration),
                    project: Ok(value.project),
                    project_id: Ok(value.project_id),
                    started_at: Ok(value.started_at),
                    updated_at: Ok(value.updated_at),
                    user_id: Ok(value.user_id),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct TimeEntryCreate {
            time_entry:
                ::std::result::Result<super::TimeEntryCreateTimeEntry, ::std::string::String>,
        }
        impl ::std::default::Default for TimeEntryCreate {
            fn default() -> Self {
                Self {
                    time_entry: Err("no value supplied for time_entry".to_string()),
                }
            }
        }
        impl TimeEntryCreate {
            pub fn time_entry<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TimeEntryCreateTimeEntry>,
                T::Error: ::std::fmt::Display,
            {
                self.time_entry = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for time_entry: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<TimeEntryCreate> for super::TimeEntryCreate {
            type Error = super::error::ConversionError;
            fn try_from(
                value: TimeEntryCreate,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    time_entry: value.time_entry?,
                })
            }
        }
        impl ::std::convert::From<super::TimeEntryCreate> for TimeEntryCreate {
            fn from(value: super::TimeEntryCreate) -> Self {
                Self {
                    time_entry: Ok(value.time_entry),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct TimeEntryCreateTimeEntry {
            billable: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            contact_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<::std::string::String, ::std::string::String>,
            detail_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ended_at: ::std::result::Result<::std::string::String, ::std::string::String>,
            paused_duration:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            project_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            started_at: ::std::result::Result<::std::string::String, ::std::string::String>,
            user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for TimeEntryCreateTimeEntry {
            fn default() -> Self {
                Self {
                    billable: Ok(Default::default()),
                    contact_id: Ok(Default::default()),
                    description: Err("no value supplied for description".to_string()),
                    detail_id: Ok(Default::default()),
                    ended_at: Err("no value supplied for ended_at".to_string()),
                    paused_duration: Ok(Default::default()),
                    project_id: Ok(Default::default()),
                    started_at: Err("no value supplied for started_at".to_string()),
                    user_id: Err("no value supplied for user_id".to_string()),
                }
            }
        }
        impl TimeEntryCreateTimeEntry {
            pub fn billable<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.billable = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for billable: {}", e));
                self
            }
            pub fn contact_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.contact_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for contact_id: {}", e));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn detail_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.detail_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for detail_id: {}", e));
                self
            }
            pub fn ended_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.ended_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ended_at: {}", e));
                self
            }
            pub fn paused_duration<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.paused_duration = value.try_into().map_err(|e| {
                    format!("error converting supplied value for paused_duration: {}", e)
                });
                self
            }
            pub fn project_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.project_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for project_id: {}", e));
                self
            }
            pub fn started_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.started_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for started_at: {}", e));
                self
            }
            pub fn user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.user_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user_id: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<TimeEntryCreateTimeEntry> for super::TimeEntryCreateTimeEntry {
            type Error = super::error::ConversionError;
            fn try_from(
                value: TimeEntryCreateTimeEntry,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    billable: value.billable?,
                    contact_id: value.contact_id?,
                    description: value.description?,
                    detail_id: value.detail_id?,
                    ended_at: value.ended_at?,
                    paused_duration: value.paused_duration?,
                    project_id: value.project_id?,
                    started_at: value.started_at?,
                    user_id: value.user_id?,
                })
            }
        }
        impl ::std::convert::From<super::TimeEntryCreateTimeEntry> for TimeEntryCreateTimeEntry {
            fn from(value: super::TimeEntryCreateTimeEntry) -> Self {
                Self {
                    billable: Ok(value.billable),
                    contact_id: Ok(value.contact_id),
                    description: Ok(value.description),
                    detail_id: Ok(value.detail_id),
                    ended_at: Ok(value.ended_at),
                    paused_duration: Ok(value.paused_duration),
                    project_id: Ok(value.project_id),
                    started_at: Ok(value.started_at),
                    user_id: Ok(value.user_id),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct TimeEntryUpdate {
            time_entry:
                ::std::result::Result<super::TimeEntryUpdateTimeEntry, ::std::string::String>,
        }
        impl ::std::default::Default for TimeEntryUpdate {
            fn default() -> Self {
                Self {
                    time_entry: Err("no value supplied for time_entry".to_string()),
                }
            }
        }
        impl TimeEntryUpdate {
            pub fn time_entry<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TimeEntryUpdateTimeEntry>,
                T::Error: ::std::fmt::Display,
            {
                self.time_entry = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for time_entry: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<TimeEntryUpdate> for super::TimeEntryUpdate {
            type Error = super::error::ConversionError;
            fn try_from(
                value: TimeEntryUpdate,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    time_entry: value.time_entry?,
                })
            }
        }
        impl ::std::convert::From<super::TimeEntryUpdate> for TimeEntryUpdate {
            fn from(value: super::TimeEntryUpdate) -> Self {
                Self {
                    time_entry: Ok(value.time_entry),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct TimeEntryUpdateTimeEntry {
            administration_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            billable: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            contact:
                ::std::result::Result<::std::option::Option<super::Contact>, ::std::string::String>,
            contact_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            created_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            detail_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ended_at: ::std::result::Result<::std::string::String, ::std::string::String>,
            events: ::std::result::Result<::std::vec::Vec<super::Event>, ::std::string::String>,
            id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            notes: ::std::result::Result<::std::vec::Vec<super::Note>, ::std::string::String>,
            paused_duration:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            project:
                ::std::result::Result<::std::option::Option<super::Project>, ::std::string::String>,
            project_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            started_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            updated_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            user_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for TimeEntryUpdateTimeEntry {
            fn default() -> Self {
                Self {
                    administration_id: Ok(Default::default()),
                    billable: Ok(Default::default()),
                    contact: Ok(Default::default()),
                    contact_id: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    description: Ok(Default::default()),
                    detail_id: Ok(Default::default()),
                    ended_at: Err("no value supplied for ended_at".to_string()),
                    events: Ok(Default::default()),
                    id: Ok(Default::default()),
                    notes: Ok(Default::default()),
                    paused_duration: Ok(Default::default()),
                    project: Ok(Default::default()),
                    project_id: Ok(Default::default()),
                    started_at: Ok(Default::default()),
                    updated_at: Ok(Default::default()),
                    user_id: Ok(Default::default()),
                }
            }
        }
        impl TimeEntryUpdateTimeEntry {
            pub fn administration_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.administration_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for administration_id: {}",
                        e
                    )
                });
                self
            }
            pub fn billable<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.billable = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for billable: {}", e));
                self
            }
            pub fn contact<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Contact>>,
                T::Error: ::std::fmt::Display,
            {
                self.contact = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for contact: {}", e));
                self
            }
            pub fn contact_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.contact_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for contact_id: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn detail_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.detail_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for detail_id: {}", e));
                self
            }
            pub fn ended_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.ended_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ended_at: {}", e));
                self
            }
            pub fn events<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Event>>,
                T::Error: ::std::fmt::Display,
            {
                self.events = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for events: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn notes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Note>>,
                T::Error: ::std::fmt::Display,
            {
                self.notes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for notes: {}", e));
                self
            }
            pub fn paused_duration<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.paused_duration = value.try_into().map_err(|e| {
                    format!("error converting supplied value for paused_duration: {}", e)
                });
                self
            }
            pub fn project<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Project>>,
                T::Error: ::std::fmt::Display,
            {
                self.project = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for project: {}", e));
                self
            }
            pub fn project_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.project_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for project_id: {}", e));
                self
            }
            pub fn started_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.started_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for started_at: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
            pub fn user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.user_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user_id: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<TimeEntryUpdateTimeEntry> for super::TimeEntryUpdateTimeEntry {
            type Error = super::error::ConversionError;
            fn try_from(
                value: TimeEntryUpdateTimeEntry,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    administration_id: value.administration_id?,
                    billable: value.billable?,
                    contact: value.contact?,
                    contact_id: value.contact_id?,
                    created_at: value.created_at?,
                    description: value.description?,
                    detail_id: value.detail_id?,
                    ended_at: value.ended_at?,
                    events: value.events?,
                    id: value.id?,
                    notes: value.notes?,
                    paused_duration: value.paused_duration?,
                    project: value.project?,
                    project_id: value.project_id?,
                    started_at: value.started_at?,
                    updated_at: value.updated_at?,
                    user_id: value.user_id?,
                })
            }
        }
        impl ::std::convert::From<super::TimeEntryUpdateTimeEntry> for TimeEntryUpdateTimeEntry {
            fn from(value: super::TimeEntryUpdateTimeEntry) -> Self {
                Self {
                    administration_id: Ok(value.administration_id),
                    billable: Ok(value.billable),
                    contact: Ok(value.contact),
                    contact_id: Ok(value.contact_id),
                    created_at: Ok(value.created_at),
                    description: Ok(value.description),
                    detail_id: Ok(value.detail_id),
                    ended_at: Ok(value.ended_at),
                    events: Ok(value.events),
                    id: Ok(value.id),
                    notes: Ok(value.notes),
                    paused_duration: Ok(value.paused_duration),
                    project: Ok(value.project),
                    project_id: Ok(value.project_id),
                    started_at: Ok(value.started_at),
                    updated_at: Ok(value.updated_at),
                    user_id: Ok(value.user_id),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct User {
            created_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            email: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            email_validated:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            is_admin: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            language: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            permissions: ::std::result::Result<
                ::std::vec::Vec<super::UserPermissionsItem>,
                ::std::string::String,
            >,
            time_zone: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            updated_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            user_type: ::std::result::Result<
                ::std::option::Option<super::UserUserType>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for User {
            fn default() -> Self {
                Self {
                    created_at: Ok(Default::default()),
                    email: Ok(Default::default()),
                    email_validated: Ok(Default::default()),
                    id: Ok(Default::default()),
                    is_admin: Ok(Default::default()),
                    language: Ok(Default::default()),
                    name: Ok(Default::default()),
                    permissions: Ok(Default::default()),
                    time_zone: Ok(Default::default()),
                    updated_at: Ok(Default::default()),
                    user_type: Ok(Default::default()),
                }
            }
        }
        impl User {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {}", e));
                self
            }
            pub fn email_validated<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.email_validated = value.try_into().map_err(|e| {
                    format!("error converting supplied value for email_validated: {}", e)
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn is_admin<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_admin = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_admin: {}", e));
                self
            }
            pub fn language<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.language = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for language: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::UserPermissionsItem>>,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
            pub fn time_zone<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.time_zone = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for time_zone: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
            pub fn user_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::UserUserType>>,
                T::Error: ::std::fmt::Display,
            {
                self.user_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user_type: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<User> for super::User {
            type Error = super::error::ConversionError;
            fn try_from(value: User) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    email: value.email?,
                    email_validated: value.email_validated?,
                    id: value.id?,
                    is_admin: value.is_admin?,
                    language: value.language?,
                    name: value.name?,
                    permissions: value.permissions?,
                    time_zone: value.time_zone?,
                    updated_at: value.updated_at?,
                    user_type: value.user_type?,
                })
            }
        }
        impl ::std::convert::From<super::User> for User {
            fn from(value: super::User) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    email: Ok(value.email),
                    email_validated: Ok(value.email_validated),
                    id: Ok(value.id),
                    is_admin: Ok(value.is_admin),
                    language: Ok(value.language),
                    name: Ok(value.name),
                    permissions: Ok(value.permissions),
                    time_zone: Ok(value.time_zone),
                    updated_at: Ok(value.updated_at),
                    user_type: Ok(value.user_type),
                }
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for Moneybird OpenAPI spec

OpenAPI spec for Moneybird: https://developer.moneybird.com/

Version: v2*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }
    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "v2"
    }
}
impl Client {
    /**List all administrations

    This endpoint returns all administrations associated with the account

    Sends a `GET` request to `/administrations`

    ```ignore
    let response = client.get_administrations()
        .send()
        .await;
    ```*/
    pub fn get_administrations(&self) -> builder::GetAdministrations {
        builder::GetAdministrations::new(self)
    }
    /**Retrieve all contacts

    Sends a `GET` request to `/{administrationId}/contacts`

    Arguments:
    - `administration_id`: The administration you want to access
    - `include_archived`: Include archived contacts
    - `page`: The page number to retrieve (for pagination)
    - `per_page`: The number of contacts per page (max 100)
    - `query`: A search query to filter contacts by name, email, or other searchable fields
    ```ignore
    let response = client.get_contacts()
        .administration_id(administration_id)
        .include_archived(include_archived)
        .page(page)
        .per_page(per_page)
        .query(query)
        .send()
        .await;
    ```*/
    pub fn get_contacts(&self) -> builder::GetContacts {
        builder::GetContacts::new(self)
    }
    /**Create a new contact

    Sends a `POST` request to `/{administrationId}/contacts`

    Arguments:
    - `administration_id`: The administration you want to access
    - `body`
    ```ignore
    let response = client.create_contact()
        .administration_id(administration_id)
        .body(body)
        .send()
        .await;
    ```*/
    pub fn create_contact(&self) -> builder::CreateContact {
        builder::CreateContact::new(self)
    }
    /**Get contact

    Sends a `GET` request to `/{administrationId}/contacts/{contactId}`

    Arguments:
    - `administration_id`: The administration you want to access
    - `contact_id`: The contact you want to retrieve
    ```ignore
    let response = client.get_contact()
        .administration_id(administration_id)
        .contact_id(contact_id)
        .send()
        .await;
    ```*/
    pub fn get_contact(&self) -> builder::GetContact {
        builder::GetContact::new(self)
    }
    /**Delete a contact

    Sends a `DELETE` request to `/{administrationId}/contacts/{contactId}`

    Arguments:
    - `administration_id`: The administration you want to access
    - `contact_id`: The contact you want to retrieve
    ```ignore
    let response = client.delete_contact()
        .administration_id(administration_id)
        .contact_id(contact_id)
        .send()
        .await;
    ```*/
    pub fn delete_contact(&self) -> builder::DeleteContact {
        builder::DeleteContact::new(self)
    }
    /**Update a contact

    Sends a `PATCH` request to `/{administrationId}/contacts/{contactId}`

    Arguments:
    - `administration_id`: The administration you want to access
    - `contact_id`: The contact you want to retrieve
    - `body`
    ```ignore
    let response = client.update_contact()
        .administration_id(administration_id)
        .contact_id(contact_id)
        .body(body)
        .send()
        .await;
    ```*/
    pub fn update_contact(&self) -> builder::UpdateContact {
        builder::UpdateContact::new(self)
    }
    /**Retrieve all projects

    Sends a `GET` request to `/{administrationId}/projects`

    Arguments:
    - `administration_id`: The administration you want to access
    - `filter`: The filter argument allows you to filter on the list of documents. Filters are a combination of keys and values, separated by a comma: key:value,key2:value2. The most common filter method will be period: period:this_month. Filtering works the same as in the web application, for more advanced examples, change the filtering in the web application and learn from the resulting URI.
    - `page`: The page number to retrieve (for pagination)
    - `per_page`: The number of projects per page (max 100)
    ```ignore
    let response = client.get_projects()
        .administration_id(administration_id)
        .filter(filter)
        .page(page)
        .per_page(per_page)
        .send()
        .await;
    ```*/
    pub fn get_projects(&self) -> builder::GetProjects {
        builder::GetProjects::new(self)
    }
    /**Create a new project

    Sends a `POST` request to `/{administrationId}/projects`

    Arguments:
    - `administration_id`: The administration you want to access
    - `body`
    ```ignore
    let response = client.create_project()
        .administration_id(administration_id)
        .body(body)
        .send()
        .await;
    ```*/
    pub fn create_project(&self) -> builder::CreateProject {
        builder::CreateProject::new(self)
    }
    /**Get project

    Sends a `GET` request to `/{administrationId}/projects/{projectId}`

    Arguments:
    - `administration_id`: The administration you want to access
    - `project_id`: The id of the project you want to retrieve
    ```ignore
    let response = client.get_project()
        .administration_id(administration_id)
        .project_id(project_id)
        .send()
        .await;
    ```*/
    pub fn get_project(&self) -> builder::GetProject {
        builder::GetProject::new(self)
    }
    /**Delete a project

    Sends a `DELETE` request to `/{administrationId}/projects/{projectId}`

    Arguments:
    - `administration_id`: The administration you want to access
    - `project_id`: The id of the project you want to retrieve
    ```ignore
    let response = client.delete_project()
        .administration_id(administration_id)
        .project_id(project_id)
        .send()
        .await;
    ```*/
    pub fn delete_project(&self) -> builder::DeleteProject {
        builder::DeleteProject::new(self)
    }
    /**Update a project

    Sends a `PATCH` request to `/{administrationId}/projects/{projectId}`

    Arguments:
    - `administration_id`: The administration you want to access
    - `project_id`: The id of the project you want to retrieve
    - `body`
    ```ignore
    let response = client.update_project()
        .administration_id(administration_id)
        .project_id(project_id)
        .body(body)
        .send()
        .await;
    ```*/
    pub fn update_project(&self) -> builder::UpdateProject {
        builder::UpdateProject::new(self)
    }
    /**Retrieve all time entries

    Sends a `GET` request to `/{administrationId}/time_entries`

    Arguments:
    - `administration_id`: The administration you want to access
    - `filter`: The filter argument allows you to filter the list of time entries. Filters are a combination of keys and values, separated by a comma: period:this_year,state:all. See API documentation for more info
    - `page`: The page number to retrieve (for pagination)
    - `per_page`: The number of time_entries per page (max 100)
    - `query`: Allows filtering by time entry description.
    ```ignore
    let response = client.get_time_entries()
        .administration_id(administration_id)
        .filter(filter)
        .page(page)
        .per_page(per_page)
        .query(query)
        .send()
        .await;
    ```*/
    pub fn get_time_entries(&self) -> builder::GetTimeEntries {
        builder::GetTimeEntries::new(self)
    }
    /**Create a new time entry

    Sends a `POST` request to `/{administrationId}/time_entries`

    Arguments:
    - `administration_id`: The administration you want to access
    - `body`
    ```ignore
    let response = client.create_time_entry()
        .administration_id(administration_id)
        .body(body)
        .send()
        .await;
    ```*/
    pub fn create_time_entry(&self) -> builder::CreateTimeEntry {
        builder::CreateTimeEntry::new(self)
    }
    /**Get time entry

    Sends a `GET` request to `/{administrationId}/time_entries/{timeEntryId}`

    Arguments:
    - `administration_id`: The administration you want to access
    - `time_entry_id`: The id of the time entry you want to retrieve
    ```ignore
    let response = client.get_time_entry()
        .administration_id(administration_id)
        .time_entry_id(time_entry_id)
        .send()
        .await;
    ```*/
    pub fn get_time_entry(&self) -> builder::GetTimeEntry {
        builder::GetTimeEntry::new(self)
    }
    /**Delete a time entry

    Sends a `DELETE` request to `/{administrationId}/time_entries/{timeEntryId}`

    Arguments:
    - `administration_id`: The administration you want to access
    - `time_entry_id`: The id of the time entry you want to retrieve
    ```ignore
    let response = client.delete_time_entry()
        .administration_id(administration_id)
        .time_entry_id(time_entry_id)
        .send()
        .await;
    ```*/
    pub fn delete_time_entry(&self) -> builder::DeleteTimeEntry {
        builder::DeleteTimeEntry::new(self)
    }
    /**Update a time entry

    Sends a `PATCH` request to `/{administrationId}/time_entries/{timeEntryId}`

    Arguments:
    - `administration_id`: The administration you want to access
    - `time_entry_id`: The id of the time entry you want to retrieve
    - `body`
    ```ignore
    let response = client.update_time_entry()
        .administration_id(administration_id)
        .time_entry_id(time_entry_id)
        .body(body)
        .send()
        .await;
    ```*/
    pub fn update_time_entry(&self) -> builder::UpdateTimeEntry {
        builder::UpdateTimeEntry::new(self)
    }
    /**List all users

    This endpoint returns all users associated with the account. Optionally, accountants can be included by setting the include_accountants parameter to true.

    Sends a `GET` request to `/{administrationId}/users`

    Arguments:
    - `administration_id`: The administration you want to access
    - `include_accountants`: Include accountants in the list of users if set to true.
    ```ignore
    let response = client.get_users()
        .administration_id(administration_id)
        .include_accountants(include_accountants)
        .send()
        .await;
    ```*/
    pub fn get_users(&self) -> builder::GetUsers {
        builder::GetUsers::new(self)
    }
}
/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt, ResponseValue,
    };
    /**Builder for [`Client::get_administrations`]

    [`Client::get_administrations`]: super::Client::get_administrations*/
    #[derive(Debug, Clone)]
    pub struct GetAdministrations<'a> {
        client: &'a super::Client,
    }
    impl<'a> GetAdministrations<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }
        ///Sends a `GET` request to `/administrations`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::Administration>>, Error<()>> {
            let Self { client } = self;
            let url = format!("{}/administrations", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_contacts`]

    [`Client::get_contacts`]: super::Client::get_contacts*/
    #[derive(Debug, Clone)]
    pub struct GetContacts<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        include_archived: Result<Option<bool>, String>,
        page: Result<Option<std::num::NonZeroU64>, String>,
        per_page: Result<Option<i64>, String>,
        query: Result<Option<::std::string::String>, String>,
    }
    impl<'a> GetContacts<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                include_archived: Ok(None),
                page: Ok(None),
                per_page: Ok(None),
                query: Ok(None),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn include_archived<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.include_archived = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for include_archived failed".to_string());
            self
        }
        pub fn page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<std::num::NonZeroU64>,
        {
            self.page = value.try_into().map(Some).map_err(|_| {
                "conversion to `std :: num :: NonZeroU64` for page failed".to_string()
            });
            self
        }
        pub fn per_page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.per_page = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for per_page failed".to_string());
            self
        }
        pub fn query<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.query = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for query failed".to_string()
            });
            self
        }
        ///Sends a `GET` request to `/{administrationId}/contacts`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::ContactRead>>, Error<()>> {
            let Self {
                client,
                administration_id,
                include_archived,
                page,
                per_page,
                query,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let include_archived = include_archived.map_err(Error::InvalidRequest)?;
            let page = page.map_err(Error::InvalidRequest)?;
            let per_page = per_page.map_err(Error::InvalidRequest)?;
            let query = query.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/contacts",
                client.baseurl,
                encode_path(&administration_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new(
                    "include_archived",
                    &include_archived,
                ))
                .query(&progenitor_client::QueryParam::new("page", &page))
                .query(&progenitor_client::QueryParam::new("per_page", &per_page))
                .query(&progenitor_client::QueryParam::new("query", &query))
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::create_contact`]

    [`Client::create_contact`]: super::Client::create_contact*/
    #[derive(Debug, Clone)]
    pub struct CreateContact<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        body: Result<types::builder::ContactCreate, String>,
    }
    impl<'a> CreateContact<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ContactCreate>,
            <V as std::convert::TryInto<types::ContactCreate>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `ContactCreate` for body failed: {}", s));
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::ContactCreate) -> types::builder::ContactCreate,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/{administrationId}/contacts`
        pub async fn send(self) -> Result<ResponseValue<types::ContactRead>, Error<()>> {
            let Self {
                client,
                administration_id,
                body,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::ContactCreate::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/contacts",
                client.baseurl,
                encode_path(&administration_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_contact`]

    [`Client::get_contact`]: super::Client::get_contact*/
    #[derive(Debug, Clone)]
    pub struct GetContact<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        contact_id: Result<::std::string::String, String>,
    }
    impl<'a> GetContact<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                contact_id: Err("contact_id was not initialized".to_string()),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn contact_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.contact_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for contact_id failed".to_string()
            });
            self
        }
        ///Sends a `GET` request to `/{administrationId}/contacts/{contactId}`
        pub async fn send(self) -> Result<ResponseValue<types::ContactRead>, Error<()>> {
            let Self {
                client,
                administration_id,
                contact_id,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let contact_id = contact_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/contacts/{}",
                client.baseurl,
                encode_path(&administration_id.to_string()),
                encode_path(&contact_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::delete_contact`]

    [`Client::delete_contact`]: super::Client::delete_contact*/
    #[derive(Debug, Clone)]
    pub struct DeleteContact<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        contact_id: Result<::std::string::String, String>,
    }
    impl<'a> DeleteContact<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                contact_id: Err("contact_id was not initialized".to_string()),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn contact_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.contact_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for contact_id failed".to_string()
            });
            self
        }
        ///Sends a `DELETE` request to `/{administrationId}/contacts/{contactId}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                administration_id,
                contact_id,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let contact_id = contact_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/contacts/{}",
                client.baseurl,
                encode_path(&administration_id.to_string()),
                encode_path(&contact_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.delete(url).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                204u16 => Ok(ResponseValue::empty(response)),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::update_contact`]

    [`Client::update_contact`]: super::Client::update_contact*/
    #[derive(Debug, Clone)]
    pub struct UpdateContact<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        contact_id: Result<::std::string::String, String>,
        body: Result<types::builder::ContactUpdate, String>,
    }
    impl<'a> UpdateContact<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                contact_id: Err("contact_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn contact_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.contact_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for contact_id failed".to_string()
            });
            self
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ContactUpdate>,
            <V as std::convert::TryInto<types::ContactUpdate>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `ContactUpdate` for body failed: {}", s));
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::ContactUpdate) -> types::builder::ContactUpdate,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `PATCH` request to `/{administrationId}/contacts/{contactId}`
        pub async fn send(self) -> Result<ResponseValue<types::ContactRead>, Error<()>> {
            let Self {
                client,
                administration_id,
                contact_id,
                body,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let contact_id = contact_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::ContactUpdate::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/contacts/{}",
                client.baseurl,
                encode_path(&administration_id.to_string()),
                encode_path(&contact_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .patch(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_projects`]

    [`Client::get_projects`]: super::Client::get_projects*/
    #[derive(Debug, Clone)]
    pub struct GetProjects<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        filter: Result<Option<types::GetProjectsFilter>, String>,
        page: Result<Option<std::num::NonZeroU64>, String>,
        per_page: Result<Option<i64>, String>,
    }
    impl<'a> GetProjects<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                filter: Ok(None),
                page: Ok(None),
                per_page: Ok(None),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn filter<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetProjectsFilter>,
        {
            self.filter = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `GetProjectsFilter` for filter failed".to_string());
            self
        }
        pub fn page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<std::num::NonZeroU64>,
        {
            self.page = value.try_into().map(Some).map_err(|_| {
                "conversion to `std :: num :: NonZeroU64` for page failed".to_string()
            });
            self
        }
        pub fn per_page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.per_page = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for per_page failed".to_string());
            self
        }
        ///Sends a `GET` request to `/{administrationId}/projects`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::ProjectRead>>, Error<()>> {
            let Self {
                client,
                administration_id,
                filter,
                page,
                per_page,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let filter = filter.map_err(Error::InvalidRequest)?;
            let page = page.map_err(Error::InvalidRequest)?;
            let per_page = per_page.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/projects",
                client.baseurl,
                encode_path(&administration_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("filter", &filter))
                .query(&progenitor_client::QueryParam::new("page", &page))
                .query(&progenitor_client::QueryParam::new("per_page", &per_page))
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::create_project`]

    [`Client::create_project`]: super::Client::create_project*/
    #[derive(Debug, Clone)]
    pub struct CreateProject<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        body: Result<types::builder::ProjectCreate, String>,
    }
    impl<'a> CreateProject<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ProjectCreate>,
            <V as std::convert::TryInto<types::ProjectCreate>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `ProjectCreate` for body failed: {}", s));
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::ProjectCreate) -> types::builder::ProjectCreate,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/{administrationId}/projects`
        pub async fn send(self) -> Result<ResponseValue<types::ProjectRead>, Error<()>> {
            let Self {
                client,
                administration_id,
                body,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::ProjectCreate::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/projects",
                client.baseurl,
                encode_path(&administration_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_project`]

    [`Client::get_project`]: super::Client::get_project*/
    #[derive(Debug, Clone)]
    pub struct GetProject<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        project_id: Result<::std::string::String, String>,
    }
    impl<'a> GetProject<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                project_id: Err("project_id was not initialized".to_string()),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn project_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.project_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for project_id failed".to_string()
            });
            self
        }
        ///Sends a `GET` request to `/{administrationId}/projects/{projectId}`
        pub async fn send(self) -> Result<ResponseValue<types::ProjectRead>, Error<()>> {
            let Self {
                client,
                administration_id,
                project_id,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let project_id = project_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/projects/{}",
                client.baseurl,
                encode_path(&administration_id.to_string()),
                encode_path(&project_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::delete_project`]

    [`Client::delete_project`]: super::Client::delete_project*/
    #[derive(Debug, Clone)]
    pub struct DeleteProject<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        project_id: Result<::std::string::String, String>,
    }
    impl<'a> DeleteProject<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                project_id: Err("project_id was not initialized".to_string()),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn project_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.project_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for project_id failed".to_string()
            });
            self
        }
        ///Sends a `DELETE` request to `/{administrationId}/projects/{projectId}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                administration_id,
                project_id,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let project_id = project_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/projects/{}",
                client.baseurl,
                encode_path(&administration_id.to_string()),
                encode_path(&project_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.delete(url).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                204u16 => Ok(ResponseValue::empty(response)),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::update_project`]

    [`Client::update_project`]: super::Client::update_project*/
    #[derive(Debug, Clone)]
    pub struct UpdateProject<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        project_id: Result<::std::string::String, String>,
        body: Result<types::ProjectUpdate, String>,
    }
    impl<'a> UpdateProject<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                project_id: Err("project_id was not initialized".to_string()),
                body: Err("body was not initialized".to_string()),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn project_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.project_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for project_id failed".to_string()
            });
            self
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ProjectUpdate>,
        {
            self.body = value
                .try_into()
                .map_err(|_| "conversion to `ProjectUpdate` for body failed".to_string());
            self
        }
        ///Sends a `PATCH` request to `/{administrationId}/projects/{projectId}`
        pub async fn send(self) -> Result<ResponseValue<types::ProjectRead>, Error<()>> {
            let Self {
                client,
                administration_id,
                project_id,
                body,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let project_id = project_id.map_err(Error::InvalidRequest)?;
            let body = body.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/projects/{}",
                client.baseurl,
                encode_path(&administration_id.to_string()),
                encode_path(&project_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .patch(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_time_entries`]

    [`Client::get_time_entries`]: super::Client::get_time_entries*/
    #[derive(Debug, Clone)]
    pub struct GetTimeEntries<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        filter: Result<Option<::std::string::String>, String>,
        page: Result<Option<std::num::NonZeroU64>, String>,
        per_page: Result<Option<i64>, String>,
        query: Result<Option<::std::string::String>, String>,
    }
    impl<'a> GetTimeEntries<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                filter: Ok(None),
                page: Ok(None),
                per_page: Ok(None),
                query: Ok(None),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn filter<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.filter = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for filter failed".to_string()
            });
            self
        }
        pub fn page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<std::num::NonZeroU64>,
        {
            self.page = value.try_into().map(Some).map_err(|_| {
                "conversion to `std :: num :: NonZeroU64` for page failed".to_string()
            });
            self
        }
        pub fn per_page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.per_page = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for per_page failed".to_string());
            self
        }
        pub fn query<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.query = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for query failed".to_string()
            });
            self
        }
        ///Sends a `GET` request to `/{administrationId}/time_entries`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::TimeEntryRead>>, Error<()>> {
            let Self {
                client,
                administration_id,
                filter,
                page,
                per_page,
                query,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let filter = filter.map_err(Error::InvalidRequest)?;
            let page = page.map_err(Error::InvalidRequest)?;
            let per_page = per_page.map_err(Error::InvalidRequest)?;
            let query = query.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/time_entries",
                client.baseurl,
                encode_path(&administration_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("filter", &filter))
                .query(&progenitor_client::QueryParam::new("page", &page))
                .query(&progenitor_client::QueryParam::new("per_page", &per_page))
                .query(&progenitor_client::QueryParam::new("query", &query))
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::create_time_entry`]

    [`Client::create_time_entry`]: super::Client::create_time_entry*/
    #[derive(Debug, Clone)]
    pub struct CreateTimeEntry<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        body: Result<types::builder::TimeEntryCreate, String>,
    }
    impl<'a> CreateTimeEntry<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TimeEntryCreate>,
            <V as std::convert::TryInto<types::TimeEntryCreate>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `TimeEntryCreate` for body failed: {}", s));
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::TimeEntryCreate) -> types::builder::TimeEntryCreate,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/{administrationId}/time_entries`
        pub async fn send(self) -> Result<ResponseValue<types::TimeEntryRead>, Error<()>> {
            let Self {
                client,
                administration_id,
                body,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::TimeEntryCreate::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/time_entries",
                client.baseurl,
                encode_path(&administration_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_time_entry`]

    [`Client::get_time_entry`]: super::Client::get_time_entry*/
    #[derive(Debug, Clone)]
    pub struct GetTimeEntry<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        time_entry_id: Result<::std::string::String, String>,
    }
    impl<'a> GetTimeEntry<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                time_entry_id: Err("time_entry_id was not initialized".to_string()),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn time_entry_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.time_entry_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for time_entry_id failed".to_string()
            });
            self
        }
        ///Sends a `GET` request to `/{administrationId}/time_entries/{timeEntryId}`
        pub async fn send(self) -> Result<ResponseValue<types::TimeEntryRead>, Error<()>> {
            let Self {
                client,
                administration_id,
                time_entry_id,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let time_entry_id = time_entry_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/time_entries/{}",
                client.baseurl,
                encode_path(&administration_id.to_string()),
                encode_path(&time_entry_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::delete_time_entry`]

    [`Client::delete_time_entry`]: super::Client::delete_time_entry*/
    #[derive(Debug, Clone)]
    pub struct DeleteTimeEntry<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        time_entry_id: Result<::std::string::String, String>,
    }
    impl<'a> DeleteTimeEntry<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                time_entry_id: Err("time_entry_id was not initialized".to_string()),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn time_entry_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.time_entry_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for time_entry_id failed".to_string()
            });
            self
        }
        ///Sends a `DELETE` request to `/{administrationId}/time_entries/{timeEntryId}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                administration_id,
                time_entry_id,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let time_entry_id = time_entry_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/time_entries/{}",
                client.baseurl,
                encode_path(&administration_id.to_string()),
                encode_path(&time_entry_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.delete(url).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                204u16 => Ok(ResponseValue::empty(response)),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::update_time_entry`]

    [`Client::update_time_entry`]: super::Client::update_time_entry*/
    #[derive(Debug, Clone)]
    pub struct UpdateTimeEntry<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        time_entry_id: Result<::std::string::String, String>,
        body: Result<types::builder::TimeEntryUpdate, String>,
    }
    impl<'a> UpdateTimeEntry<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                time_entry_id: Err("time_entry_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn time_entry_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.time_entry_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for time_entry_id failed".to_string()
            });
            self
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TimeEntryUpdate>,
            <V as std::convert::TryInto<types::TimeEntryUpdate>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `TimeEntryUpdate` for body failed: {}", s));
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::TimeEntryUpdate) -> types::builder::TimeEntryUpdate,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `PATCH` request to `/{administrationId}/time_entries/{timeEntryId}`
        pub async fn send(self) -> Result<ResponseValue<types::TimeEntryRead>, Error<()>> {
            let Self {
                client,
                administration_id,
                time_entry_id,
                body,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let time_entry_id = time_entry_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::TimeEntryUpdate::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/time_entries/{}",
                client.baseurl,
                encode_path(&administration_id.to_string()),
                encode_path(&time_entry_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .patch(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_users`]

    [`Client::get_users`]: super::Client::get_users*/
    #[derive(Debug, Clone)]
    pub struct GetUsers<'a> {
        client: &'a super::Client,
        administration_id: Result<::std::string::String, String>,
        include_accountants: Result<Option<bool>, String>,
    }
    impl<'a> GetUsers<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                administration_id: Err("administration_id was not initialized".to_string()),
                include_accountants: Ok(None),
            }
        }
        pub fn administration_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.administration_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for administration_id failed"
                    .to_string()
            });
            self
        }
        pub fn include_accountants<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.include_accountants = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for include_accountants failed".to_string());
            self
        }
        ///Sends a `GET` request to `/{administrationId}/users`
        pub async fn send(self) -> Result<ResponseValue<::std::vec::Vec<types::User>>, Error<()>> {
            let Self {
                client,
                administration_id,
                include_accountants,
            } = self;
            let administration_id = administration_id.map_err(Error::InvalidRequest)?;
            let include_accountants = include_accountants.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/users",
                client.baseurl,
                encode_path(&administration_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new(
                    "include_accountants",
                    &include_accountants,
                ))
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    pub use self::super::Client;
}
