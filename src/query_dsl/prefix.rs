//! This mod is to..
//! https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-prefix-query.html
//!
//! Returns documents that contain a specific prefix in a provided field

mod dsl {
    use crate::zdbquery::ZDBQuery;
    use pgx::*;
    use serde_json::*;
    use std::iter::FromIterator;

    #[pg_extern(immutable, parallel_safe)]
    pub(super) fn prefix(field: &str, value: &str) -> ZDBQuery {
        make_prefix_dsl(field, value)
    }

    #[inline]
    fn make_prefix_dsl(field: &str, value: &str) -> ZDBQuery {
        ZDBQuery::new_with_query_dsl(json! {
            {
                "prefix": {
                    field: {
                        "value": value
                    }
                }
            }
        })
    }
}

mod tests {
    use crate::query_dsl::prefix::dsl::*;
    use crate::zdbquery::ZDBQuery;
    use pgx::*;
    use pgx_tests::*;
    use serde_json::json;

    #[test]
    fn make_idea_happy() {}

    #[pg_test]
    fn test_prefix() {
        let zdbquery = prefix("fieldname", "te");
        let dls = zdbquery.query_dsl();

        assert!(dls.is_some());
        assert_eq!(
            dls.unwrap(),
            &json! {
                {
                    "prefix": {"fieldname": {"value": "te"}}
                }
            }
        );
    }
}
