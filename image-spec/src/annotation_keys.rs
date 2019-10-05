//! Pre-defined annotation keys.
//!
//! See the [OCI image spec] for more information.
//!
//! [OCI image spec]: https://github.com/opencontainers/image-spec/blob/v1.0.1/annotations.md#pre-defined-annotation-keys

macro_rules! oci_image_key {
    ($s: literal) => {
        concat!("org.opencontainers.image.", $s)
    };
}

/// Date and time on which the image was built (in RFC 3339 format).
pub const CREATED: &str = oci_image_key!("created");

/// Contact details of the people or organization responsible for the image.
pub const AUTHORS: &str = oci_image_key!("authors");

/// URL to find more information on the image.
pub const URL: &str = oci_image_key!("url");

/// URL to get documentation on the image.
pub const DOCUMENTATION: &str = oci_image_key!("documentation");

/// URL to get source code for building the image.
pub const SOURCE: &str = oci_image_key!("source");

/// Version of the packaged software.
pub const VERSION: &str = oci_image_key!("version");

/// Source control revision identifier for the packaged software.
pub const REVISION: &str = oci_image_key!("revision");

/// Name of the distributing entity, organization or individual.
pub const VENDOR: &str = oci_image_key!("vendor");

/// License(s) under which contained software is distributed as an SPDX License Expression.
pub const LICENSE: &str = oci_image_key!("license");

/// Name of the reference for a target.
pub const REF_NAME: &str = oci_image_key!("ref.name");

/// Validates the format of a reference.
///
/// Returns `true` if the reference has a valid format. Returns `false` if not.
///
/// # Examples
///
/// ```
/// use oci_image_spec::annotation_keys::validate_ref_name;
///
/// assert!(validate_ref_name("stable-release"));
/// assert!(validate_ref_name("v1.0"));
/// ```
pub fn validate_ref_name(ref_name: &str) -> bool {
    // ref       ::= component ("/" component)*
    // component ::= alphanum (separator alphanum)*
    // alphanum  ::= [A-Za-z0-9]+
    // separator ::= [-._:@+] | "--"

    for component in ref_name.split('/') {
        for dash_sp in component.split("--") {
            for sp in dash_sp.split(|c| match c {
                '-' | '.' | '_' | ':' | '@' | '+' => true,
                _ => false,
            }) {
                if sp.is_empty() || sp.chars().any(|c| !c.is_ascii_alphanumeric()) {
                    return false;
                }
            }
        }
    }

    true
}

/// Human-readable title of the image.
pub const TITLE: &str = oci_image_key!("title");

/// Human-readable description of the software packaged in the image.
pub const DESCRIPTION: &str = oci_image_key!("description");

/// Label Schema labels compatible with `org.opencontainers.image` keys.
pub mod label_schema {
    macro_rules! label_schema_label {
        ($s: literal) => {
            concat!("org.label-schema.", $s)
        };
    }

    /// Compatible with `org.opencontainers.image.created` key.
    pub const BUILD_DATE: &str = label_schema_label!("build-date");

    /// Compatible with `org.opencontainers.image.url` key.
    pub const URL: &str = label_schema_label!("url");

    /// Compatible with `org.opencontainers.image.documentation` key.
    pub const USAGE: &str = label_schema_label!("usage");

    /// Compatible with `org.opencontainers.image.source` key.
    pub const VCS_URL: &str = label_schema_label!("vcs-url");

    /// Compatible with `org.opencontainers.image.version` key.
    pub const VERSION: &str = label_schema_label!("version");

    /// Compatible with `org.opencontainers.image.revision` key.
    pub const VCS_REF: &str = label_schema_label!("vcs-ref");

    /// Compatible with `org.opencontainers.image.vendor` key.
    pub const VENDOR: &str = label_schema_label!("vendor");

    /// Compatible with `org.opencontainers.image.title` key.
    pub const TITLE: &str = label_schema_label!("name");

    /// Compatible with `org.opencontainers.image.description` key.
    pub const DESCRIPTION: &str = label_schema_label!("description");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ref_name() {
        assert!(validate_ref_name("stable/v1.0"));
        assert!(validate_ref_name("stable-v1.0_pre:alpha@debug+0--tmp"));
    }

    #[test]
    fn err_validate_ref_name() {
        let test_cases = &["/", "stable/", "/v1.0", "-", "stable-", "-stable", "="];

        for case in test_cases {
            assert!(!validate_ref_name(case));
        }
    }
}
