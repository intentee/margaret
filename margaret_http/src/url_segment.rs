use margaret_url_path::path_parameter_value::PathParameterValue;
use margaret_url_path::path_tail_value::PathTailValue;
use margaret_url_path::segment_prefix::SegmentPrefix;

pub enum UrlSegment {
    CatchAllParameter {
        name: &'static str,
        prefix: &'static str,
        value: String,
    },
    Literal {
        decoded: &'static str,
        encoded: &'static str,
    },
    Parameter {
        name: &'static str,
        prefix: &'static str,
        value: String,
    },
}

impl UrlSegment {
    #[must_use]
    pub fn catch_all<TPrefix: SegmentPrefix>(
        name: &'static str,
        value: PathTailValue<TPrefix>,
    ) -> Self {
        Self::CatchAllParameter {
            name,
            prefix: TPrefix::TEXT,
            value: value.into_string(),
        }
    }

    #[must_use]
    pub fn parameter<TPrefix: SegmentPrefix>(
        name: &'static str,
        value: PathParameterValue<TPrefix>,
    ) -> Self {
        Self::Parameter {
            name,
            prefix: TPrefix::TEXT,
            value: value.into_string(),
        }
    }
}
