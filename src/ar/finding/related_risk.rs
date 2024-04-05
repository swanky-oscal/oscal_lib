use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, UUIDDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelatedRisk {
    risk_uuid: UUIDDatatype,
}

impl SchemaElement for RelatedRisk {
    fn schema_title() -> &'static str {
        "Associated Risk"
    }
    fn schema_description() -> &'static str {
        "Relates the finding to a set of referenced risks that were used to determine the finding."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "#/definitions/oscal-complete-oscal-ar:finding/related-risks"
    }
}
